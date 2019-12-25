use crate::synth::wavetable::{Wavetable, WavetableOsc};
use crate::SAMPLE_RATE;

#[derive(Debug)]
pub struct Oscillator {
    table: &'static Wavetable,
    phase: f32,
    freq: f32,
}

impl Oscillator {
    pub fn new(name: &'static str) -> Self {
        Self {
            freq: 0.,
            phase: 0.,
            table: Wavetable::load(name),
        }
    }
    pub fn next_phase(&mut self) -> () {
        self.phase = (self.phase + self.freq) % SAMPLE_RATE as f32;
    }

    pub fn get_sample(&self) -> f32 {
        *self.table.get(self.phase as usize).unwrap()
    }

    pub fn set_freq(&mut self, freq: f32) -> () {
        self.freq = freq;
    }

    pub fn set_harmonic_offset(&mut self, freq: f32, n: usize) -> () {
        self.set_freq(freq * (n + 1) as f32);
    }
}

#[macro_export]
macro_rules! sine {
    () => {
        Oscillator::new(crate::synth::wavetable::SINE)
    };
}

#[macro_export]
macro_rules! square {
    () => {
        Oscillator::new(crate::synth::wavetable::SQUARE)
    };
}
