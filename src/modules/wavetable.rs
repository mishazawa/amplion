#![allow(dead_code)]

use core::f32::consts::PI;
use rand;
use crate::{
  SAMPLE_RATE,
};

const HALF_SR: i32 = SAMPLE_RATE / 2;

fn sine_wave () -> Vec<f32> {
  (0..SAMPLE_RATE).map(|x| {
    (x as f32 * 2.0 * PI / SAMPLE_RATE as f32).sin()
  }).collect::<Vec<f32>>()
}

fn square_wave () -> Vec<f32> {
  (0..SAMPLE_RATE).map(|x| {
    if x < HALF_SR { 1.0 } else { -1.0 }
  }).collect::<Vec<f32>>()
}

fn saw_wave () -> Vec<f32> {
  (0..SAMPLE_RATE).map(|x| {
    (1.0 - (1.0 / PI * (x as f32 * 2.0 * PI / SAMPLE_RATE as f32)))
  }).collect::<Vec<f32>>()
}

fn triangle_wave () -> Vec<f32> {
  (0..SAMPLE_RATE).map(|x| {
    let a = (2.0 / PI) * (x as f32 * 2.0 * PI / SAMPLE_RATE as f32);
    if x < HALF_SR { -1.0 + a } else { 3.0 - a }
  }).collect::<Vec<f32>>()
}

fn noise_wave () -> Vec<f32> {
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
      Waves::SIN | Waves::SINE => sine_wave(),
      Waves::SQ | Waves::SQUARE => square_wave(),
      Waves::SAW | Waves::SAWTOOTH => saw_wave(),
      Waves::TRI | Waves::TRIANGLE => triangle_wave(),
      Waves::NO | Waves::NOISE => noise_wave()
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
  pub fn new (wave: Waves) -> Self {
    Self {
      phase: 0.0,
      table: Table::new(wave)
    }
  }
  pub fn next_phase (&mut self, freq: f32) {
    self.phase = (self.phase + freq) % SAMPLE_RATE as f32;
  }
  pub fn get_value (&self) -> f32 {
    self.table.get(self.phase)
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
