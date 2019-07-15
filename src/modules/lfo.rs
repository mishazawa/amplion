use crate::modules::{
  wavetable::{Osc, Waves}
};


use crate::{saw};

pub struct Lfo {
  form: Osc,
  freq: f32,
}

impl Lfo {
  pub fn new (freq: f32) -> Self {
    Self {
      form: saw!(),
      freq
    }
  }
  pub fn get_amp (&mut self) -> f32 {
    self.form.next_phase(self.freq);
    self.form.get_value()
  }
}
