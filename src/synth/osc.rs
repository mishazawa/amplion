use crate::synth::wavetable::{Wavetable, WavetableOsc};
use crate::SAMPLE_RATE;
use crate::processing::Wireable;

#[derive(Debug)]
pub struct Freq(f32);

impl Freq {
    pub fn new(v: String) -> Self {
        Self(v.parse().unwrap())
    }
}

impl Wireable for Freq {
    fn input (&mut self, val: f32) {
        self.0 = val;
    }

    fn output (&mut self) -> f32 {
        self.0
    }

}

#[derive(Debug)]
pub struct Oscillator {
    table: &'static Wavetable,
    phase: f32,
    freq: Freq,
}

impl Oscillator {
    pub fn new(name: &'static str) -> Self {
        Self {
            freq: Freq(0.),
            phase: 0.,
            table: Wavetable::load(name),
        }
    }
    pub fn next_phase(&mut self) -> () {
        self.phase = (self.phase + self.freq.0) % SAMPLE_RATE as f32;
    }

    pub fn get_sample(&self) -> f32 {
        *self.table.get(self.phase as usize).unwrap()
    }

    pub fn set_freq(&mut self, freq: f32) -> () {
        self.freq = Freq(freq);
    }

    pub fn set_harmonic_offset(&mut self, freq: f32, n: usize) -> () {
        self.set_freq(freq * (n + 1) as f32);
    }
}

impl Wireable for Oscillator {
    fn input (&mut self, val: f32) {
        self.set_freq(val);
    }

    fn output (&mut self) -> f32 {
        self.next_phase();
        self.get_sample()
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
