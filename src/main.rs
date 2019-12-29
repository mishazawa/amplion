#[macro_use]
extern crate lazy_static;
extern crate gui;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::processing::ModuleMessage;
use crate::synth::wavetable::{preload, SINE, SQUARE};
use crate::utils::audio;

mod processing;
mod synth;
mod utils;

pub const SAMPLE_RATE: i32 = 44_100;

lazy_static! {
    pub static ref SINEWAVETABLE: Vec<f32> = preload(SINE);
    pub static ref SQUAREWAVETABLE: Vec<f32> = preload(SQUARE);
}

fn main() -> () {
    let (gui_tx, gui_rx) = mpsc::channel();
    let (audio_tx, audio_rx) = mpsc::channel();
    let (processing_tx, processing_rx) = mpsc::channel();
    let (request_audio_tx, request_audio_rx) = mpsc::channel();

    let gui_join = thread::spawn(move || gui::main(gui_tx));
    let audio_join = thread::spawn(move || audio::main(audio_rx, request_audio_tx));
    let processing_join = thread::spawn(move || processing::main(audio_tx, processing_rx, request_audio_rx));

    loop {
        match gui_rx.try_recv() {
            Ok(data) => match data.cmd.as_ref() {
                "init" => {}
                "exit" => {
                    processing_tx
                        .send(ModuleMessage {
                            kind: "exit".to_string(),
                            value: data.r#type,
                            id: None,
                            from: None,
                            to: None,
                        })
                        .unwrap();
                    break;
                }
                "module" => {
                    processing_tx
                        .send(ModuleMessage {
                            kind: data.r#type,
                            value: data.value,
                            id: Some(data.id),
                            from: None,
                            to: None,
                        })
                        .unwrap();
                }
                "wire" => {
                    processing_tx
                        .send(ModuleMessage {
                            kind: "wire".to_string(),
                            value: data.r#type,
                            id: None,
                            from: Some(data.value),
                            to: Some(data.id),
                        })
                        .unwrap();
                }
                _ => {}
            },
            Err(_) => {}
        }
        sleep(20);
    }

    gui_join.join().expect("Gui thread crashed.");
    audio_join.join().expect("Audio thread crashed.");
    processing_join.join().expect("Processing thread crashed.");
}

fn sleep(t: u64) {
    thread::sleep(Duration::from_millis(t));
}
