use core::f32::consts::PI;
use rand;

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
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
  wave: Waves,
  phase: f32,
  samples: Vec<f32>,
  sample_rate: i32,
}

impl Wavetable {
  pub fn new (wtype: Waves, sample_rate: i32) -> Self {
    Self {
      wave: wtype,
      samples: Wavetable::gen(wtype, sample_rate),
      phase: 0.0,
      sample_rate,
    }
  }
  fn gen (wave: Waves, sample_rate: i32) -> Vec<f32> {
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

  pub fn get_value (&self, phase: f32) -> f32 {
    *self.samples.get(phase as usize).unwrap()
  }

  pub fn sample_rate (&self) -> i32 {
    self.sample_rate
  }
}
