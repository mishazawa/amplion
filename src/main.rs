extern crate cpal;
extern crate portmidi as pm;
extern crate rand;

mod osc;
mod midi;
mod misc;

use std::clone::{Clone};
use std::sync::mpsc;
use std::thread;

fn main() {
  // let context = pm::PortMidi::new().unwrap();
  // let timeout = Duration::from_millis(10);
  // const BUF_LEN: usize = 1024;
  let (tx, rx) = mpsc::channel();

  let device = cpal::default_output_device().expect("Failed to get default output device");
  let format = device.default_output_format().expect("Failed to get default output format");

  let event_loop = cpal::EventLoop::new();
  let stream_id = event_loop.build_output_stream(&device, &format).unwrap();

  event_loop.play_stream(stream_id.clone());

  let sample_rate = format.sample_rate.0;

  let mut n3 = osc::Wavetable::new(osc::Waves::SIN, sample_rate as i32);

  thread::spawn(move || {
    if let Err(e) = misc::play(tx, false) {
      println!("{:?}", e);
    }
  });

  let mut last_freq = 0.0;

  event_loop.run(move |_, data| {
    match data {
      cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {

        if let Ok(mess) = rx.try_recv() {
          match mess.status {
            midi::KEY_DEPRESS => {
              last_freq = 0.0;
            },
            midi::KEY_PRESS => {
              last_freq = midi::midi_to_freq(mess.data1);
            },
            _ => ()
          }
        }

        for sample in buffer.chunks_mut(format.channels as usize) {
          let v3 = n3.next_value(last_freq);
          for out in sample.iter_mut() {
            *out = misc::amplify(v3, 0.5);
          };
        }

      },
      _ => (),
    }
  });

}
