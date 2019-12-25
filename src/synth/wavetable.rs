use crate::utils::io;
use crate::{SINEWAVETABLE, SQUAREWAVETABLE};

pub type Wavetable = Vec<f32>;

pub const SINE: &str = "sine";
pub const SQUARE: &str = "square";

pub trait WavetableOsc {
    fn load(name: &str) -> &Wavetable;
}

impl WavetableOsc for Wavetable {
    fn load(name: &str) -> &Wavetable {
        match name {
            SINE => &SINEWAVETABLE,
            SQUARE => &SQUAREWAVETABLE,
            _ => &SINEWAVETABLE,
        }
    }
}

pub fn preload(name: &str) -> Wavetable {
    io::read_file(name.to_string())
}
