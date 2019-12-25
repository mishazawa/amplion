use crate::synth::osc::{Oscillator};
use crate::synth::envelope::Envelope;
use crate::synth::mixer;

#[derive(Debug)]
pub struct Voice {
  oscillators: Vec<Oscillator>,
  envelope: Envelope,
}


impl Voice {
  pub fn new (oscs: Vec<Oscillator>, f: impl Fn(Envelope) -> Envelope) -> Self {
    let env = Envelope::new(f);
    Self {
      oscillators: oscs,
      envelope: env,
    }
  }
  pub fn play_note (&mut self, freq: f32) -> () {
    self.envelope.start();
    self.attenuate(freq);
  }

  pub fn stop_note (&mut self) -> () {
    self.envelope.stop();
  }

  pub fn get_sample (&mut self) -> f32 {
    let env_amp = self.envelope.get_amp();
    let mut amps = vec![];
    for osc in self.oscillators.iter_mut() {
      osc.next_phase();
      amps.push(osc.get_sample());
    }
    mixer::mix(amps) * env_amp
  }

  fn attenuate (&mut self, freq: f32) -> () {
    for (i, osc) in self.oscillators.iter_mut().enumerate() {
      osc.set_harmonic_offset(freq, i);
    }
  }
}
