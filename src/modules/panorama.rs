pub struct Panorama {
  left: f32,
  right: f32,
}


impl Panorama {
  pub fn new () -> Self {
    Self {
      left: 1.0,
      right: 1.0
    }
  }

  pub fn l(&self, amp: f32) -> f32 {
    amp * self.left
  }
  pub fn r(&self, amp: f32) -> f32 {
    amp * self.right
  }
}
