extern crate cpal;
extern crate portmidi as pm;
extern crate rand;

mod osc;
mod midi;
mod misc;

use std::clone::{Clone};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Envelope {
  max_amp: f32,
  attack_time: f32,
  decay_time: f32,
  sustain_amp: f32,
  release_time: f32,
  samples: Vec<f32>,
  current_time: i32,
  duration_time: f32,

}

impl Default for Envelope {
  fn default () -> Envelope {
    Envelope {
      attack_time: 0.0,
      decay_time: 0.0,
      release_time: 0.0,
      sustain_amp: 0.8,
      max_amp: 1.0,
      current_time: 0,
      duration_time: 0.4,
      samples: Vec::new()
    }
  }
}

impl Envelope {
  pub fn new (sample_rate: i32, a: f32, d: f32, s: f32, r: f32) -> Self {
    let mut env = Self {
      attack_time: a,
      decay_time: d,
      sustain_amp: s,
      release_time: r,
      ..Default::default()
    };

    env.gen_adsr(sample_rate);
    env
  }

  fn gen_adsr (&mut self,
    sample_rate: i32,
    ) {
    let mut samples = Vec::new();

    let attack = self.attack_time * sample_rate as f32;
    let decay = self.decay_time * sample_rate as f32;
    let release = self.release_time * sample_rate as f32;
    let duration = self.duration_time * sample_rate as f32;

    let mut index = 0.0;

    while index < duration {
      let mut amp = 0.0;


      if index <= attack {
        amp = index * self.max_amp / attack;
      }

      if index <= attack + decay {
        amp = ((self.sustain_amp - self.max_amp) / decay) * (index - attack) + self.max_amp;
      }

      if index <= duration - release {
        amp = self.sustain_amp;
      }

      if index > release {
        amp = -(self.sustain_amp / release) * (index - (duration - release)) + self.sustain_amp;
      }

      samples.push(amp);
      index += 1.0;
    }

    self.samples = samples;
  }

  pub fn gate (&mut self, on: bool) {
    if !on {
      self.current_time = 0;
    }
  }

  pub fn next_value (&mut self) -> f32 {
    self.current_time = (self.current_time + 1) % self.samples.len() as i32;
    *self.samples.get(self.current_time as usize).unwrap()
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

  let mut n3 = osc::Wavetable::new(osc::Waves::SIN, sample_rate);
  let mut _env = Envelope::new(sample_rate, 0.8, 0.1, 0.8, 0.5);

  thread::spawn(move || {
    if let Err(e) = misc::play(tx.clone(), false) {
      println!("{:?}", e);
    } else {
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
    }

  });

  let mut last_freq = 0.0;
  let mut note_on = false;

  event_loop.run(move |_, data| {
    match data {
      cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {

        if let Ok(mess) = rx.try_recv() {
          match mess.status {
            midi::KEY_DEPRESS => {
              last_freq = 0.0;
              note_on = false;
              _env.gate(note_on);
            },
            midi::KEY_PRESS => {
              note_on = true;
              last_freq = midi::midi_to_freq(mess.data1);
            },
            _ => ()
          }
        }

        for sample in buffer.chunks_mut(format.channels as usize) {
          let v3 = n3.next_value(last_freq);
          let amp = _env.next_value();
          println!("{:?}", amp);
          for out in sample.iter_mut() {
            *out = misc::amplify(v3, amp);
          };
        }

      },
      _ => (),
    }
  });

}
