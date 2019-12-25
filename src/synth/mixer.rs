pub fn mix (samples: Vec<f32>) -> f32 {
  samples.iter().fold(0., |acc, sample| {
    let n = midpoint(*sample);      // 0 .. 1
    normalize((acc + n) - acc * n)  // -1 .. 1
  })
}

fn midpoint (sample: f32) -> f32 {
  sample * 0.5 + 1.
}

fn normalize (sample: f32) -> f32 {
  sample * 2. - 1.
}
