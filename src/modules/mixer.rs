use std::collections::HashMap;

use super::{
  voice::Voice,
  envelope::Envelope,
  wavetable::Wavetable
};


#[derive(Debug)]
pub struct Mixer {
  pub voices: HashMap<u8, Voice>,
}

impl <'a> Mixer {
  pub fn new () -> Self {
    Self { voices: HashMap::new() }
  }

  pub fn amplify (v: f32, a: f32) -> f32 { v * a }

  pub fn add (&mut self, voice: Voice) {
    self.voices.insert(voice.note, voice);
  }

  pub fn remove_unused (&mut self, envelope: &Envelope, time: f32) {
    let empties: Vec<_> = self.voices.iter_mut().filter(|(_, v)| {
      v.enabled == false && envelope.is_stopped(v.end_time, time)
    }).map(|(k, _)| k.clone()).collect();

    for empty in empties { self.voices.remove(&empty); }
  }

  pub fn mix (&mut self, osc: &Wavetable, env: &Envelope, time_elapsed: f32) -> f32 {
    let mut amplitude = 0.0;
    for (_, voice) in &mut self.voices {
      voice.next_phase();
      amplitude += env.get_amp_voice(time_elapsed, &voice) * osc.get_value(voice.phase);
    }
    Mixer::amplify(amplitude.tanh(), 1.0)
  }
}

