use std::panic;
use std::time::{Instant};

#[derive(Debug)]
pub struct Envelope {
  attack:  f32,
  decay:   f32,
  sustain: f32,
  release: f32,
  /* timers */

  start_time: Instant,
  stop_time:  Instant

}

impl Default for Envelope {
  fn default () -> Self {
    Self {
      attack: 0.3,
      decay: 0.1,
      release: 0.4,
      sustain: -1.0,
      stop_time: Instant::now(),
      start_time: Instant::now(),
    }
  }
}

/*
  Env new -> start -> get_amp .... -> stop -> get_amp ... ... ...
*/

impl Envelope {
  pub fn new (f: impl Fn(Self) -> Self) -> Self {
    f(Self {..Default::default()})
  }
  pub fn set_params (&mut self, a: f32, d: f32, s: f32, r: f32) {
    self.attack = a;
    self.decay = d;
    self.sustain = s;
    self.release = r;
  }
  pub fn start (&mut self) {
    self.stop_time = Instant::now();
    self.start_time = Instant::now();
  }

  pub fn stop (&mut self) {
    self.stop_time = Instant::now();
  }

  pub fn get_amp (&mut self) -> f32 {
    let mut amp = 0.0;
    let max_amp: f32 = 1.;
    let sustain_amp: f32 = 0.707;
    let time    = self.start_time.elapsed().as_millis() as f32;
    let attack  = self.attack  * 1000.0;
    let decay   = self.decay   * 1000.0;
    let release = self.release * 1000.0;

    let mut is_stopped = false;
    // check if stop time is after start time
    let delta = panic::catch_unwind(|| self.stop_time.duration_since(self.start_time));

    if delta.is_err() && self.is_plucked(time) {
      self.stop();
      is_stopped = true;
    }
    // start time is later than end time
    if delta.is_err() {
      if time <= attack {
        amp = (time / attack) * max_amp;
      }

      if time <= attack + decay  {
        amp = ((sustain_amp - max_amp) / decay) * (time - attack) + max_amp;
      }

      if time >= attack + decay {
        amp = sustain_amp;
      }

      if amp <= 0.0001 {
        amp = 0.0001;
      }
    }

    // start time is earlier then end time
    if is_stopped || delta.is_ok() {
      amp = ((time - self.stop_time.elapsed().as_millis() as f32) / release) * (0.0 - sustain_amp) + sustain_amp;
    }
    amp
  }

  fn is_plucked (&self, time: f32) -> bool {
    self.sustain < 1.0 && time >= self.sustain * 1000.0
  }
}
