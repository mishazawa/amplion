pub struct Panorama {
  balance: f32,
}


impl Panorama {
  pub fn new () -> Self {
    Self {
      balance: 0.0
    }
  }

  pub fn apply (&self, sample: &mut [f32], amp: f32) {
    sample[0] = (1.0 - self.balance) * amp;
    sample[1] = (1.0 + self.balance) * amp;
  }
}
