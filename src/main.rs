extern crate cpal;
extern crate rand;

use core::f32::consts::PI;
use std::clone::{Clone};

#[derive(Debug, Copy, Clone)]
pub enum Waves {
  SIN,
  SINE,
  SQ,
  SQUARE,
  SAW,
  SAWTOOTH,
  TRI,
  TRIANGLE,
  NOISE,
  NO,
}

#[derive(Debug)]
pub struct Wavetable {
  pub wave: Waves,
  pub phase: f32,
  pub samples: Vec<f32>,
  pub sample_rate: i32,
}

impl Wavetable {
  fn new (wtype: Waves, sample_rate: i32) -> Self {
    Self {
      wave: wtype,
      samples: Wavetable::gen(wtype, sample_rate),
      phase: 0.0,
      sample_rate,
    }
  }
  fn gen(wave: Waves, sample_rate: i32) -> Vec<f32> {
    let mut samples = Vec::new();
    match wave {
      Waves::SIN | Waves::SINE => {
        for sample_clock in 0..sample_rate {
          samples.push((sample_clock as f32 * 2.0 * PI / sample_rate as f32).sin());
        }
      },
      Waves::SQ | Waves::SQUARE => {
        for sample_clock in 0..sample_rate {
          if sample_clock < sample_rate / 2 {
            samples.push(0.9);
          } else {
            samples.push(-0.9);
          }
        }
      },
      Waves::SAW | Waves::SAWTOOTH => {
        for sample_clock in 0..sample_rate {
          samples.push(1.0 - (1.0 / PI * (sample_clock as f32 * 2.0 * PI / sample_rate as f32)));
        }
      },
      Waves::TRI | Waves::TRIANGLE => {
        for sample_clock in 0..sample_rate {
          let mut a = (2.0 / PI) * (sample_clock as f32 * 2.0 * PI / sample_rate as f32);
          if sample_clock < sample_rate / 2 {
            a = -1.0 + a;
          } else {
            a = 3.0 - a;
          }
          samples.push(a);
        }
      },
      Waves::NO | Waves::NOISE => {
        for _ in 0..sample_rate {
          samples.push(rand::random::<f32>())
        }
      }
      // _ => ()
    };
    samples
  }

  fn next_value (&mut self, freq: f32) -> f32 {
    self.phase = (self.phase + freq) % self.samples.len() as f32;
    *self.samples.get(self.phase as usize).unwrap()
  }
}

fn amplify (v: f32, a: f32) -> f32 { v * a }

fn main() {
  let device = cpal::default_output_device().expect("Failed to get default output device");
  let format = device.default_output_format().expect("Failed to get default output format");

  let event_loop = cpal::EventLoop::new();
  let stream_id = event_loop.build_output_stream(&device, &format).unwrap();

  event_loop.play_stream(stream_id.clone());

  let sample_rate = format.sample_rate.0;

  let mut sine = Wavetable::new(Waves::SINE, sample_rate as i32);

  let amp = 0.2;

  event_loop.run(move |_, data| {
    match data {
      cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
        for sample in buffer.chunks_mut(format.channels as usize) {
          let v = sine.next_value(440.0);
          for out in sample.iter_mut() {
            *out = amplify(v, amp);
          };
        }
      },
      _ => (),
    }
  });
}
