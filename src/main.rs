extern crate cpal;

use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

mod synth;
mod utils;

use crate::synth::osc::Oscillator;
use crate::synth::voice::Voice;


pub const SAMPLE_RATE: i32 = 44_100;

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




    let mut v = Voice::new(vec![
        sine!(),
        square!(),
        sine!(),
        square!(),

        ], |mut env| {
            env.set_params(5., 4., -1., 15.);
            env
        });

    v.play_note(20.);
    // Produce a sinusoid of maximum amplitude.

    event_loop.run(move |id, result| {
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
                    let value = v.get_sample();
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            _ => (),
        }
        v.stop_note();
        v.play_note(10000.);

    });
}
