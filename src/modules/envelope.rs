use super::voice::{Voice};

#[derive(Debug)]
pub struct Envelope {
  max_amp: f32,
  sustain_amp: f32,
  attack_time: f32,
  decay_time: f32,
  release_time: f32,
}

impl Default for Envelope {
  fn default () -> Envelope {
    Envelope {
      attack_time: 0.3,
      decay_time: 0.1,
      release_time: 0.4,
      sustain_amp: 0.5,
      max_amp: 1.0,
    }
  }
}

impl Envelope {
  pub fn new (f: impl Fn(Self) -> Self) -> Self {
    f(Self {..Default::default()})
  }

  pub fn is_stopped (&self, release_value: f32, time_elapsed: f32) -> bool {
    time_elapsed - release_value > self.release_time * 1000.0
  }

  pub fn get_amp_voice (&self, time_elapsed: f32, voice: &Voice) -> f32 {
    let mut amp = 0.0;
    let time = time_elapsed - voice.start_time;

    let attack = self.attack_time * 1000.0;
    let decay = self.decay_time * 1000.0;
    let release = self.release_time * 1000.0;

    if voice.enabled {
      if time <= attack {
        amp = (time / attack) * self.max_amp;
      }

      if time <= attack + decay  {
        amp = ((self.sustain_amp - self.max_amp) / decay) * (time - attack) + self.max_amp;
      }

      if time >= attack + decay {
        amp = self.sustain_amp;
      }

    } else {
      amp = ((time_elapsed - voice.end_time) / release) * (0.0 - self.sustain_amp) + self.sustain_amp;
    }

    if amp <= 0.0001 {
      amp = 0.0001;
    }

    amp
  }

  pub fn set_params (&mut self, a: f32, d: f32, s: f32, r: f32) {
    self.attack_time = a;
    self.decay_time = d;
    self.release_time = r;
    self.sustain_amp = s;
  }

  pub fn set_amps (&mut self, m: f32, s: f32) {
    self.max_amp = m;
    self.sustain_amp = s;
  }
}
