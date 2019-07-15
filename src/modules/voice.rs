use crate::modules::wavetable::{Osc, Waves};
use crate::midi::{midi_to_freq};
use crate::{sine};


#[derive(Debug)]
pub struct Voice {
  pub enabled: bool,
  pub start_time: f32,
  pub end_time: f32,

  pub note: u8,
  pub freq: f32,
  pub osc: Osc,
}


pub fn use_voice (note: u8) -> Voice {
  Voice {
    note,
    freq: midi_to_freq(note),
    start_time: 0.0,
    end_time: 0.0,
    enabled: true,
    osc: sine!()
  }
}
