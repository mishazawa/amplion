use crate::modules::{
  wavetable::{Wavetable, Waves},
  voice::{Voice, create_blank_voice}
};


pub struct Lfo {
  form: Wavetable,
  voice: Voice,
}

impl Lfo {
  pub fn new (freq: f32, sample_rate: i32) -> Self {
    Self {
      form: Wavetable::new(Waves::SAW, sample_rate),
      voice: create_blank_voice(freq, sample_rate)
    }
  }
  pub fn get_amp (&mut self) -> f32 {
    self.voice.next_phase();
    self.form.get_value(self.voice.phase)
  }
}
