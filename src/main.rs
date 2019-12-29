#[macro_use]
extern crate lazy_static;
extern crate gui;

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::processing::ProcMessage;
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

    let gui_join = thread::spawn(move || gui::main(gui_tx));
    let audio_join = thread::spawn(move || audio::main(audio_rx));
    let processing_join = thread::spawn(move || processing::main(audio_tx, processing_rx));

    loop {
        match gui_rx.try_recv() {
            Ok(data) => match data.cmd.as_ref() {
                "init" => {}
                "exit" => {
                    break;
                }
                "module" => match data.r#type.as_ref() {
                    "freq" => {}
                    "osc" => {
                        processing_tx.send(ProcMessage::osc(data.id, data.r#type));
                    }
                    "out" => {}
                    "remove" => {}
                    _ => {}
                },
                "wire" => match data.r#type.as_ref() {
                    "connect" => {}
                    "disconnect" => {}
                    _ => {}
                },
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
