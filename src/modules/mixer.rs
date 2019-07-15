use super::{
  voice::Voice,
  envelope::Envelope,
};


#[derive(Debug)]
pub struct Mixer {
  pub voices: Vec<Voice>,
}

impl <'a> Mixer {
  pub fn new () -> Self {
    Self { voices: vec![] }
  }

  pub fn amplify (v: f32, a: f32) -> f32 { v * a }

  pub fn add (&mut self, voice: Voice) {
    self.voices.push(voice);
  }

  pub fn remove_unused (&mut self, envelope: &Envelope, time: f32) {
    let empties: Vec<_> = self.voices.iter_mut().filter(|v| {
      v.enabled == false && envelope.is_stopped(v.end_time, time)
    }).enumerate().map(|(k, _)| k.clone()).collect();

    for empty in empties { self.voices.remove(empty); }
  }

  pub fn mix (&mut self, env: &Envelope, time_elapsed: f32) -> f32 {
    let mut amplitude = 0.0;
    for voice in self.voices.iter_mut() {
      voice.osc.next_phase(voice.freq);

      amplitude += env.get_amp_voice(time_elapsed, voice) * voice.osc.get_value();
    }
    Mixer::amplify(amplitude.tanh(), 1.0)
  }
}

