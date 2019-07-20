pub fn infinite_clipping (val: f32) -> f32 {
  return if val > 0.0 {1.0} else {-1.0};
}
