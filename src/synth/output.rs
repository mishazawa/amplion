use crate::processing::Wireable;
use crate::synth::mixer;

#[derive(Debug)]
pub struct Output {
    id: String,
    buffer: Vec<f32>,
}

impl Wireable for Output {
    fn input(&mut self, value: f32) {
        self.buffer.push(value);
    }

    fn output(&mut self) -> f32 {
        let mixdown = mixer::mix(self.buffer.clone());
        self.buffer.clear();
        mixdown
    }
}

impl Output {
    pub fn new(id: String) -> Self {
        Self { id, buffer: vec![] }
    }
}
