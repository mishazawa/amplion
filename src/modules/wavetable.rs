#![allow(dead_code)]

use core::f32::consts::PI;
use rand;
use std::collections::{HashMap};
use crate::{
  SAMPLE_RATE,
  modules::envelope::{Envelope}
};

const HALF_SR: i32 = SAMPLE_RATE / 2;

pub fn sine () -> Vec<f32> {
  (0..SAMPLE_RATE).map(|x| {
    (x as f32 * 2.0 * PI / SAMPLE_RATE as f32).sin()
  }).collect::<Vec<f32>>()
}

fn square () -> Vec<f32> {
  (0..SAMPLE_RATE).map(|x| {
    if x < HALF_SR { 1.0 } else { -1.0 }
  }).collect::<Vec<f32>>()
}

fn saw () -> Vec<f32> {
  (0..SAMPLE_RATE).map(|x| {
    (1.0 - (1.0 / PI * (x as f32 * 2.0 * PI / SAMPLE_RATE as f32)))
  }).collect::<Vec<f32>>()
}

fn triangle () -> Vec<f32> {
  (0..SAMPLE_RATE).map(|x| {
    let a = (2.0 / PI) * (x as f32 * 2.0 * PI / SAMPLE_RATE as f32);
    if x < HALF_SR { -1.0 + a } else { 3.0 - a }
  }).collect::<Vec<f32>>()
}

fn noise () -> Vec<f32> {
  (0..SAMPLE_RATE).map(|_| rand::random::<f32>()).collect::<Vec<f32>>()
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
pub enum Waves {
  SIN,
  SINE,
  SQ,
  SQUARE,
  SAW,
  SAWTOOTH,
  TRI,
  TRIANGLE,
  NOISE,
  NO,
}

#[derive(Debug)]
pub struct Table(Vec<f32>);

impl Table {
  fn new (wave: Waves) -> Self {
    let tb: Vec<f32> = match wave {
      Waves::SIN | Waves::SINE => sine(),
      Waves::SQ | Waves::SQUARE => square(),
      Waves::SAW | Waves::SAWTOOTH => saw(),
      Waves::TRI | Waves::TRIANGLE => triangle(),
      Waves::NO | Waves::NOISE => noise()
    };
    Self(tb)
  }
  pub fn get (&self, phase: f32) -> f32 {
    *self.0.get(phase as usize).unwrap()

  }
}



#[derive(Debug)]
pub struct Osc {
  phase: f32,
  table: Table
}


impl Osc {
  fn new (wave: Waves) -> Self {
    Self {
      phase: 0.0,
      table: Table::new(wave)
    }
  }
  fn next_phase (&mut self, freq: f32) {
    self.phase = (self.phase + freq) % SAMPLE_RATE as f32;
  }
}


#[derive(Debug)]
struct Time {
  pub end_time: f32,
  pub start_time: f32,
}

fn sum () -> f32 { 0.0 }

#[derive(Debug)]
struct Instrument {
  voices: Vec<Osc>,
  frequencies: Vec<f32>,
  time: Time,
  // effects: Vec<Effect>,
  envelope: Envelope
}

impl Instrument {
  fn new (f: impl Fn(Self) -> Self) -> Self {
    f(Self {
      voices: vec![],
      frequencies: vec![],
      time: Time { end_time: 0.0, start_time: 0.0 },
      envelope: Envelope::new(|e| e)
    })
  }

  fn play (&mut self, _time: f32) -> f32 {
    self.sum()// + self.envelope.get_amp_voice(time, )
  }

  fn sum (&mut self) -> f32 {
    let mut amplitude = 0.0;

    for (i, freq) in &mut self.frequencies.iter().enumerate() {
      let voice = self.voices.get_mut(i).unwrap();
      voice.next_phase(*freq);

      amplitude += voice.table.get(voice.phase);
    }

    amplitude.tanh()
  }
}


#[derive(Debug)]
pub struct Wavetable {
  wave: Waves,
  phase: f32,
  samples: Vec<f32>,
}

impl Wavetable {
  pub fn new (wtype: Waves) -> Self {
    Self {
      wave: wtype,
      samples: Wavetable::gen(wtype),
      phase: 0.0,
    }
  }
  fn gen (wave: Waves) -> Vec<f32> {
    let mut samples = Vec::new();
    match wave {
      Waves::SIN | Waves::SINE => {
        for sample_clock in 0..SAMPLE_RATE {
          samples.push((sample_clock as f32 * 2.0 * PI / SAMPLE_RATE as f32).sin());
        }
      },
      Waves::SQ | Waves::SQUARE => {
        for sample_clock in 0..SAMPLE_RATE {
          if sample_clock < SAMPLE_RATE / 2 {
            samples.push(0.9);
          } else {
            samples.push(-0.9);
          }
        }
      },
      Waves::SAW | Waves::SAWTOOTH => {
        for sample_clock in 0..SAMPLE_RATE {
          samples.push(1.0 - (1.0 / PI * (sample_clock as f32 * 2.0 * PI / SAMPLE_RATE as f32)));
        }
      },
      Waves::TRI | Waves::TRIANGLE => {
        for sample_clock in 0..SAMPLE_RATE {
          let mut a = (2.0 / PI) * (sample_clock as f32 * 2.0 * PI / SAMPLE_RATE as f32);
          if sample_clock < SAMPLE_RATE / 2 {
            a = -1.0 + a;
          } else {
            a = 3.0 - a;
          }
          samples.push(a);
        }
      },
      Waves::NO | Waves::NOISE => {
        for _ in 0..SAMPLE_RATE {
          samples.push(rand::random::<f32>())
        }
      }
      // _ => ()
    };
    samples
  }

  pub fn get_value (&self, phase: f32) -> f32 {
    *self.samples.get(phase as usize).unwrap()
  }
}


#[macro_export]
macro_rules! sine {
  () => (Osc::new(Waves::SINE))
}

#[macro_export]
macro_rules! square {
  () => (Osc::new(Waves::SQUARE))
}

#[macro_export]
macro_rules! triangle {
  () => (Osc::new(Waves::TRI))
}

#[macro_export]
macro_rules! saw {
  () => (Osc::new(Waves::SAW))
}

#[macro_export]
macro_rules! noise {
  () => (Osc::new(Waves::NOISE))
}
