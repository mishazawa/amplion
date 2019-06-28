#[derive(Debug)]
pub struct Envelope {
  max_amp: f32,
  sustain_amp: f32,
  attack_time: f32,
  decay_time: f32,
  release_time: f32,
  on_trigger_time: f32,
  off_trigger_time: f32,
  enabled: bool
}

impl Default for Envelope {
  fn default () -> Envelope {
    Envelope {
      attack_time: 0.3,
      decay_time: 0.1,
      release_time: 0.4,
      sustain_amp: 0.5,
      max_amp: 1.0,
      on_trigger_time: 0.0,
      off_trigger_time: 0.0,
      enabled: false
    }
  }
}

impl Envelope {
  pub fn new () -> Self {
    Self {..Default::default()}
  }

  pub fn get_amp (&mut self, time_elapsed: f32) -> f32 {
    let mut amp = 0.0;
    let time = time_elapsed - self.on_trigger_time;

    let attack = self.attack_time * 1000.0;
    let decay = self.decay_time * 1000.0;
    let release = self.release_time * 1000.0;

    if self.enabled {
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
      amp = ((time_elapsed - self.off_trigger_time) / release) * (0.0 - self.sustain_amp) + self.sustain_amp;
    }

    if amp <= 0.0001 { amp = 0.0; }

    amp
  }

  pub fn gate (&mut self, flag: bool, time: f32) {
    self.enabled = flag;
    if flag {
      self.on_trigger_time = time;
    } else {
      self.off_trigger_time = time;
    }
  }

  pub fn set_params (&mut self, a: f32, d: f32, s: f32, r: f32) {
    self.attack_time = a;
    self.decay_time = d;
    self.release_time = r;
    self.sustain_amp = s;
  }
}
