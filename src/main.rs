extern crate cpal;

use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

mod synth;
mod utils;

use crate::synth::osc::Oscillator;
use crate::synth::mixer;

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

    let mut sine = sine!();
    let mut sine1 = sine!();
    let mut sine2 = sine!();
    let mut sine3 = sine!();
    let mut sine4 = sine!();
    let mut sine5 = sine!();

    let freq = 440.;
    let freq1 = 3450.;
    let freq2 = 140.;
    let freq3 = 40.;
    let freq4 = 4540.;
    let freq5 = 1440.;
    // Produce a sinusoid of maximum amplitude.
    let mut next_value = || {
        sine.next_phase(freq);
        sine1.next_phase(freq1);
        sine2.next_phase(freq2);
        sine3.next_phase(freq3);
        sine4.next_phase(freq4);
        sine5.next_phase(freq5);
        mixer::mix(vec![
            sine.get_sample(),
            sine1.get_sample(),
            sine2.get_sample(),
            sine3.get_sample(),
            sine4.get_sample(),
            sine5.get_sample()
            ])
    };

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
                    let value = ((next_value() * 0.5 + 0.5) * std::u16::MAX as f32) as u16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::I16(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = (next_value() * std::i16::MAX as f32) as i16;
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            cpal::StreamData::Output {
                buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer),
            } => {
                for sample in buffer.chunks_mut(format.channels as usize) {
                    let value = next_value();
                    println!("{:?}", value);
                    for out in sample.iter_mut() {
                        *out = value;
                    }
                }
            }
            _ => (),
        }
    });
}
