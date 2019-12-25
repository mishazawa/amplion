use crate::synth::wavetable::{Wavetable, WavetableOsc};
use crate::SAMPLE_RATE;

#[derive(Debug)]
pub struct Oscillator {
    table: Wavetable,
    phase: f32,
}

impl Oscillator {
    pub fn new(name: &str) -> Self {
        Oscillator {
            phase: 0.,
            table: Wavetable::load(name),
        }
    }

    pub fn next_phase(&mut self, freq: f32) -> () {
        self.phase = (self.phase + freq) % SAMPLE_RATE as f32;
    }

    pub fn get_sample(&self) -> f32 {
        *self.table.get(self.phase as usize).unwrap()
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
