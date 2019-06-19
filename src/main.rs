extern crate cpal;

use core::f32::consts::PI;

#[derive(Debug)]
pub enum Waves {
  SINE,
  SQUARE,
}

#[derive(Debug)]
pub struct Wavetable {
  pub wave: Waves,
  pub phase: f32,
  pub samples: Vec<f32>
}

impl Wavetable {
  fn new (wtype: Waves) -> Self {
    Self { wave: wtype, samples: Vec::new() , phase: 0.0 }
  }

  pub fn gen (&mut self, sample_rate: i32) {
    match self.wave {
      Waves::SINE => {
        for sample_clock in 0..sample_rate {
          self.samples.push((sample_clock as f32 * 2.0 * PI / sample_rate as f32).sin());
        }
      },
      Waves::SQUARE => {
        for sample_clock in 0..sample_rate {
          if sample_clock < sample_rate / 2 {
            self.samples.push(0.9);
          } else {
            self.samples.push(-0.9);
          }
        }
      },
    };
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

  let mut sine = Wavetable::new(Waves::SQUARE);
  sine.gen(sample_rate as i32);

  event_loop.run(move |_, data| {
    match data {
      cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
        for sample in buffer.chunks_mut(format.channels as usize) {
          let v = sine.next_value(880.0);
          for out in sample.iter_mut() {
            *out = amplify(v, 0.0125);
          };
        }
      },
      _ => (),
    }
  });
}
