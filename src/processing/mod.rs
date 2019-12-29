use std::sync::mpsc::{Receiver, Sender};

#[derive(Debug)]
pub struct ProcMessage {
    id: String,
    kind: String,
    module: bool,
    wire: bool,
    value: Option<f32>,
    wave: Option<String>,
    from: Option<String>,
    to: Option<String>,
}

impl Default for ProcMessage {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            kind: "".to_string(),
            module: false,
            wire: false,
            wave: None,
            value: None,
            from: None,
            to: None,
        }
    }
}

impl ProcMessage {
    pub fn osc(id: String, value: String) -> Self {
        Self {
            id,
            kind: "osc".to_string(),
            module: true,
            wave: Some(value),
            ..Default::default()
        }
    }
}

pub fn main(audio: Sender<f32>, mess: Receiver<ProcMessage>) {}
