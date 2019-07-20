#![allow(dead_code)]

pub mod clipping {
  pub fn infinite (val: f32) -> f32 {
    val/val.abs()
  }

  pub fn hard (val: f32, threshold: f32) -> f32 {
    let amp = if val >= threshold {
      threshold
    } else if val <= -threshold {
      -threshold
    } else {
      val
    };
    amp
  }

  pub fn diode (val: f32) -> f32 {
    0.105 * (((0.1 * val) / (1.68 * 0.0253)).exp() - 1.0)
  }

  pub fn piece_wise_overdrive (val: f32) -> f32 {
    let amp = if val.abs() < 1.0 / 3.0 {
      2.0 * val
    } else if val.abs() > 2.0 / 3.0 {
      (val/val.abs())
    } else {
      (val/val.abs()) * ((3.0 - (2.0 - 3.0 * val.abs()).powf(2.0)) / 3.0)
    };

    amp
  }


  pub mod soft {
    use core::f32::consts::{PI, E};
    pub fn cubic (val: f32) -> f32 {
      val - (val.powf(3.0) / 3.0)
    }

    pub fn atan (val: f32, level: f32) -> f32 {
      (2.0 * (level * val).atan()) / PI
    }

    pub fn sine (val: f32) -> f32 {
      (2.0 * PI * val).sin()
    }

    pub fn exp (val: f32, gain: f32) -> f32 {
      (val / val.abs()) * (1.0 - E.powf(gain * val))
    }

  }
}

pub fn half_wave_rectification (val: f32) -> f32 {
  return if val >= 0.0 {val} else {0.0};
}

pub fn full_wave_rectification (val: f32) -> f32 {
  return if val >= 0.0 {val} else {-val};
}

pub fn bit_crush (val: f32, depth: u8) -> f32 {
  let amps = 2.0f32.powf(depth as f32);
  2.0 * ((amps * (0.5 * val + 0.5)).round() / amps ) - 1.0
}

pub fn asymmetric (val: f32, dc: f32) -> f32 {
  let mut input = val + dc;

  if input.abs() > 1.0 {
    input = input / input.abs()
  }

  (input - (input.powf(5.0) / 5.0)) - dc
}

pub fn blend (dry: f32, wet: f32, blend: u8) -> f32 {
  let mix = (blend/100) as f32;
  mix * wet + (1.0 - mix) * dry
}
