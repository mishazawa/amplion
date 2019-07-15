use crate::modules::wavetable::{Osc};

#[derive(Debug)]
pub struct Voice {
  pub enabled: bool,
  pub start_time: f32,
  pub end_time: f32,

  pub note: u8,
  pub freq: f32,
  pub osc: Osc,
}
