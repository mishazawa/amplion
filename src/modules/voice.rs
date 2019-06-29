#[derive(Debug)]
pub struct Voice {
  pub note: u8,
  pub enabled: bool,
  pub end_time: f32,
  pub phase: f32,
  pub start_time: f32,
  pub freq: f32,
  pub sample_rate: i32,
}

impl Voice {
  pub fn next_phase (&mut self) {
    self.phase = (self.phase + self.freq) % self.sample_rate as f32;
  }
}
