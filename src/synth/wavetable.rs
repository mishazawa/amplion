use crate::utils::io;

pub type Wavetable = Vec<f32>;

pub const SINE: &str = "sine";
pub const SQUARE: &str = "square";

pub trait WavetableOsc {
    fn load(name: &str) -> Wavetable;
}

impl WavetableOsc for Wavetable {
    fn load(name: &str) -> Self {
        io::read_file(name.to_string())
    }
}
