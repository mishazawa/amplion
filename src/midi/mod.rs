pub fn midi_to_freq (key: u8) -> f32 {
  (440.0 * (2.0f32).powf((key as f32 - 69.0) / 12.0)).floor()
}

pub const KEY_PRESS:u8 = 144;
pub const KEY_DEPRESS:u8 = 128;
