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
struct Voice {
  note: u8,
  freq: f32,
  phase: f32,
  sample_rate: i32,
  start_time: f32,
  end_time: f32,
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

  pub fn add (&mut self, voice: Voice) {
    self.voices.insert(voice.note, voice);
  }

  pub fn remove (&mut self, key: &u8) {
    self.voices.remove(key);
  }

  pub fn mix (&mut self, osc: &osc::Wavetable) -> f32 {
    let mut amplitude = 0.0;
    let channels = self.voices.len() as f32;
    for (_, voice) in &mut self.voices {
      voice.next_phase();
      amplitude += osc.get_value(voice.phase) / channels;
    }
    amplitude
  }
}

// fn filter_depressed_voices (voices: &mut HashMap<u8, Voice>) {
//   voices = voices.filter()
// }

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

  env.set_params(0.1, 0.1, 0.3, 5.0);

  thread::spawn(move || {
    // if let Err(e) = misc::play(tx.clone(), true) {
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

  let mut last_freq = 0.0;

  let mut timer = misc::Timer::new();

  let mut mixer = Mixer::new();

  event_loop.run(move |_, data| {
    match data {
      cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {

        timer.tick();

        if let Ok(mess) = rx.try_recv() {

          match mess.status {

            midi::KEY_DEPRESS => {
              env.gate(false, timer.get_delta());
              // match voices.get_mut(&mess.data1) {
              //     Some(voice) => {
              //       voice.end_time = timer.get_delta();
              //     },
              //     None => println!("Midi {} is unreviewed.", &mess.data1)
              // }
              mixer.remove(&mess.data1);
            },

            midi::KEY_PRESS => {
              env.gate(true, timer.get_delta());
              mixer.add(Voice {
                note: mess.data1,
                freq: midi::midi_to_freq(mess.data1),
                phase: 0.0,
                sample_rate: n3.sample_rate(),
                start_time: timer.get_delta(),
                end_time: 0.0,
              });

              last_freq = midi::midi_to_freq(mess.data1);
            },

            _ => ()
          }
        }

        for sample in buffer.chunks_mut(format.channels as usize) {
          let amplitude = mixer.mix(&n3);

          for out in sample.iter_mut() {
            *out = misc::amplify(amplitude, 0.3);
          };

        }

      },
      _ => (),
    }
  });

}
