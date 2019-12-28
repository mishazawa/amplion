#[derive(Debug)]
pub struct Pan {
    val: f32,
}

/*
   0.5
L---|---R

  0.3
L--|----R
(0.3)
*/

impl Pan {
    pub fn new() -> Self {
        Self { val: 0.5 }
    }
    pub fn set(&mut self, val: f32) {
        self.val = val;
    }
    pub fn apply(&self, sample: &mut [f32], val: f32) -> () {
        let values = [val * self.val, val * (1. - self.val)];
        for (n, out) in sample.iter_mut().enumerate() {
            *out = *values.get(n).unwrap();
        }
    }
}
