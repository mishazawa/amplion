use std::collections::HashMap;
use std::sync::mpsc::{Receiver, Sender};

use crate::synth::osc::{Freq, Oscillator};
use crate::synth::output::Output;
use crate::synth::mixer;

#[derive(Debug)]
pub struct ModuleMessage {
    pub kind: String,
    pub value: String,
    pub id: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
}

pub trait Wireable {
    fn input(&mut self, value: f32);
    fn output(&mut self) -> f32;
}

#[derive(Debug)]
struct Wire {
    // id: String,
    from: String,
    to: String
}

pub fn main(audio: Sender<f32>, mess: Receiver<ModuleMessage>, request_audio: Receiver<()>) {
    let mut wires: Vec<Wire> = Vec::new();
    let mut freqs: HashMap<String, Freq> = HashMap::new();
    let mut oscs: HashMap<String, Oscillator> = HashMap::new();
    let mut outs: HashMap<String, Output> = HashMap::new();

    loop {
        if let Ok(data) = mess.try_recv() {
            match data.kind.as_ref() {
                "freq" => {
                    freqs.insert(data.id.unwrap(), Freq::new(data.value));
                }
                "osc" => match data.value.as_ref() {
                    "sine" => {
                        oscs.insert(
                            data.id.unwrap(),
                            Oscillator::new(crate::synth::wavetable::SINE),
                        );
                    }
                    "square" => {
                        oscs.insert(
                            data.id.unwrap(),
                            Oscillator::new(crate::synth::wavetable::SQUARE),
                        );
                    }
                    _ => {}
                },
                "out" => {
                    let id = data.id.unwrap();
                    outs.insert(id.clone(), Output::new(id));
                }
                "remove" => {
                    let id = data.id.unwrap();
                    freqs.remove(&id);
                    oscs.remove(&id);
                    outs.remove(&id);
                    wires.dedup_by_key(|item| item.to == id || item.from == id);
                }
                "wire" => match data.value.as_ref() {
                    "connect" => {
                        wires.push(Wire {
                            from: data.from.unwrap(),
                            to: data.to.unwrap(),
                        });
                    },
                    "disconnect" => {
                        // let id = data.id.unwrap();
                        // if let Some(index) = wires.iter().position(|r| r.id == id) {
                        //     wires.remove(index);
                        // }
                    }
                    _ => {}
                },
                "exit" => {
                    break;
                }
                _ => {}
            }
        };

        if let Ok(_) = request_audio.try_recv() {
            let mut samples = vec![];

            for connection in &wires {
                let is_from_freq = freqs.contains_key(&connection.from);
                let is_from_osc = oscs.contains_key(&connection.from);
                // let is_from_out = outs.contains_key(&connection.from);

                // let is_to_freq = freqs.contains_key(&connection.to);
                let is_to_osc = oscs.contains_key(&connection.to);
                let is_to_out = outs.contains_key(&connection.to);

                if is_from_freq && is_to_osc {
                    let freq = match freqs.get_mut(&connection.from) {
                        Some(from) => from.output(),
                        None => 0.0,
                    };
                    let oscillator = oscs.get_mut(&connection.to).unwrap();
                    oscillator.input(freq);
                }

                if is_from_osc && is_to_out {
                    let amplitude = match oscs.get_mut(&connection.from) {
                        Some(from) => from.output(),
                        None => 0.0,
                    };
                    samples.push(amplitude);
                }

            }
            let stream_value = mixer::mix(samples);
            audio.send(stream_value).unwrap()
        }
    }
}
