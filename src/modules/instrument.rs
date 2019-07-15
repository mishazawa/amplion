use portmidi::{MidiMessage};


use crate::modules::{
  mixer::Mixer,
  envelope::Envelope,
  wavetable::{Osc},
};

pub struct Instrument {
  pub osc: Vec<Osc>,
  pub envelope: Envelope,
  pub polyphony: Mixer,
  pub on_midi_event: fn(MidiMessage, &mut Mixer, f32) -> ()
}

impl Instrument {
  pub fn on_midi_message (&mut self, mess: MidiMessage, delta_time: f32) {
    (self.on_midi_event)(mess, &mut self.polyphony, delta_time);
  }

  pub fn get_amp (&mut self, delta_time: f32) -> f32 {

    let mut amp = 0.0;
    for osc in self.osc.iter() {
      amp += self.polyphony.mix(&osc, &self.envelope, delta_time);
    }

    (amp / self.osc.len() as f32).tanh()
  }
  pub fn remove_unused (&mut self, delta_time: f32) {
    self.polyphony.remove_unused(&self.envelope, delta_time);
  }
}
