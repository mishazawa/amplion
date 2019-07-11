pub struct Panorama {
  balance: f32,
}


impl Panorama {
  pub fn new () -> Self {
    Self {
      balance: 0.0
    }
  }

  pub fn apply (&self, amp: f32) -> [f32; 2] {
    [(1.0 - self.balance) * amp, (1.0 + self.balance) * amp]
  }
}
