extern crate cpal;

#[macro_use]
extern crate lazy_static;

extern crate gui;

use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};
use std::sync::mpsc;
use std::thread;

mod synth;
mod utils;

use crate::synth::osc::Oscillator;
use crate::synth::panorama::Pan;
use crate::synth::voice::Voice;
use crate::synth::wavetable::{preload, SINE, SQUARE};

pub const SAMPLE_RATE: i32 = 44_100;
lazy_static! {
    pub static ref SINEWAVETABLE: Vec<f32> = preload(SINE);
    pub static ref SQUAREWAVETABLE: Vec<f32> = preload(SQUARE);
}

fn main() -> () {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let format = device
        .default_output_format()
        .expect("failed to find a default_output_format");
    let event_loop = host.event_loop();
    let stream_id = event_loop
        .build_output_stream(&device, &format)
        .expect("failed to build_output_stream");
    event_loop
        .play_stream(stream_id.clone())
        .expect("failed to play_stream");
    let mut pan = Pan::new();

    pan.set(0.8);

    let mut v = Voice::new(vec![sine!(), square!(), sine!(), square!()], |mut env| {
        env.set_params(1., 1., 0., 3.);
        env
    });

    let (tx, rx) = mpsc::channel();
    // gui thread
    thread::spawn(move || {
        gui::main(tx);
    });

    // v.play_note(20.);
    event_loop.run(move |id, result| {
        match rx.try_recv() {
            Ok(data) => {
                match data.cmd.as_ref() {
                    "init" => {},
                    "module"=> {
                        match data.r#type.as_ref() {
                            "freq" => {},
                            "osc" => {},
                            "out" => {},
                            "remove" => {},
                            _ => {}
                        }
                    },
                    "wire" => {
                        match data.r#type.as_ref() {
                            "connect" => {},
                            "disconnect" => {},
                            _ => {}
                        }
                    },
                    _ => {}
                }
            }
            Err(_) => {}
        }

        let data = match result {
            Ok(data) => data,
            Err(err) => {
                eprintln!("an error occurred on stream {:?}: {}", id, err);
                return;
            }
        };

        match data {
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::U16(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = ((v.get_sample() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = (v.get_sample() * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    pan.apply(sample, v.get_sample());
                }
            }
            _ => (),
        }
        // v.stop_note();
        // v.play_note(10000.);
    });
}
