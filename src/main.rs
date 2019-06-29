extern crate cpal;
extern crate portmidi as pm;
extern crate rand;
mod osc;
mod midi;
mod misc;
mod env;

use std::clone::{Clone};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Voice {
  note: u8,
  freq: f32,
  phase: f32,
  sample_rate: i32,
  start_time: f32,
  end_time: f32,
  enabled: bool
}

impl Voice {
  pub fn next_phase (&mut self) {
    self.phase = (self.phase + self.freq) % self.sample_rate as f32;
  }
}

#[derive(Debug)]
struct Mixer {
  voices: HashMap<u8, Voice>,
}

impl Mixer {
  pub fn new () -> Self {
    Self { voices: HashMap::new() }
  }

  pub fn amplify (v: f32, a: f32) -> f32 { v * a }

  pub fn add (&mut self, voice: Voice) {
    self.voices.insert(voice.note, voice);
  }

  pub fn remove_unused (&mut self, envelope: &env::Envelope, time: f32) {
    let empties: Vec<_> = self.voices.iter_mut().filter(|(_, v)| {
      v.enabled == false && envelope.get_diff(v.end_time, time)
    }).map(|(k, _)| k.clone()).collect();

    for empty in empties { self.voices.remove(&empty); }
  }

  pub fn normalize (&self, values: Vec<f32>) -> f32 {
    let summary = values.iter().fold(0.0, |acc, &x| acc + x);
    if values.len() > 0 { summary / values.len() as f32 } else { summary }
  }

  pub fn mix (&mut self, osc: &osc::Wavetable, env: &env::Envelope, time_elapsed: f32) -> f32 {
    let mut amps = Vec::new();
    for (_, voice) in &mut self.voices {
      voice.next_phase();
      amps.push(env.get_amp_voice(time_elapsed, &voice) * osc.get_value(voice.phase));
    }
    self.normalize(amps)
  }
}

fn main() {
  let context = pm::PortMidi::new().unwrap();
  let timeout = Duration::from_millis(10);
  const BUF_LEN: usize = 1024;
  let (tx, rx) = mpsc::channel();

  let device = cpal::default_output_device().expect("Failed to get default output device");
  let format = device.default_output_format().expect("Failed to get default output format");

  let event_loop = cpal::EventLoop::new();
  let stream_id = event_loop.build_output_stream(&device, &format).unwrap();

  event_loop.play_stream(stream_id.clone());

  let sample_rate = format.sample_rate.0 as i32;

  let n3 = osc::Wavetable::new(osc::Waves::SIN, sample_rate);
  let mut env = env::Envelope::new();

  env.set_params(1.1, 0.4, 0.5, 5.0);

  thread::spawn(move || {
    // if let Err(e) = misc::play(tx.clone(), false) {
    //   println!("{:?}", e);
    // } else {
      let in_ports = context
                      .devices()
                      .unwrap()
                      .into_iter()
                      .filter_map(|dev| context.input_port(dev, BUF_LEN).ok())
                      .collect::<Vec<_>>();
      loop {
        for port in &in_ports {
          if let Ok(Some(events)) = port.read_n(BUF_LEN) {
            for e in events {
              tx.send(e.message).unwrap();
            }
          }
        }
        thread::sleep(timeout);
      }
    // }

  });

  let mut timer = misc::Timer::new();
  let mut mixer = Mixer::new();

  event_loop.run(move |_, data| {
    match data {
      cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {

        timer.tick();

        if let Ok(mess) = rx.try_recv() {

          match mess.status {

            midi::KEY_DEPRESS => {
              match mixer.voices.get_mut(&mess.data1) {
                Some(voice) => {
                  voice.end_time = timer.get_delta();
                  voice.enabled = false;
                },
                None => println!("Midi {} is unreviewed.", &mess.data1)
              }
              // mixer.remove(&mess.data1);
            },

            midi::KEY_PRESS => {
              mixer.add(Voice {
                note: mess.data1,
                freq: midi::midi_to_freq(mess.data1),
                phase: 0.0,
                sample_rate: n3.sample_rate(),
                start_time: timer.get_delta(),
                end_time: 0.0,
                enabled: true
              });
            },

            _ => ()
          }
        }

        for sample in buffer.chunks_mut(format.channels as usize) {
          let amplitude = mixer.mix(&n3, &env, timer.get_delta());
          for out in sample.iter_mut() {
            *out = Mixer::amplify(amplitude, 1.0);
          };
        }

        mixer.remove_unused(&env, timer.get_delta());
      },
      _ => (),
    }
  });

}
