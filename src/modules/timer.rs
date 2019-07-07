use std::time::{Duration, SystemTime};
#[derive(Debug)]
pub struct Timer {
  time: SystemTime,
  delta: Duration,
}

impl Timer {
  pub fn new () -> Self {
    println!("u128 as f32 max:    {}", u128::max_value() as f32);
    Self {
      time: SystemTime::now(),
      delta: Duration::new(0, 0)
    }
  }

  pub fn tick (&mut self) {
    match self.time.elapsed() {
      Ok(elapsed) => {
        self.reset();
        self.delta = elapsed;
      },
      Err(e) => {
        println!("Error: {:?}", e);
      }
    }
  }

  pub fn get_delta (&self) -> f32 {
    self.delta.as_millis() as f32
  }

  pub fn flush (&mut self) {
    self.time = SystemTime::now();
    self.delta = Duration::new(0, 0);
  }

  fn reset (&mut self) {
    if self.get_delta() == std::f32::INFINITY {
      self.flush();
    }
  }
}
