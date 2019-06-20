extern crate cpal;
extern crate portmidi as pm;
extern crate rand;

mod osc;
mod midi;
mod misc;

use std::clone::{Clone};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Envelope {
  attack: f32,
  decay: f32,
  sustain: f32,
  release: f32,
}

impl Envelope {
  pub fn new (a: f32, d: f32, s: f32, r: f32) -> Self {
    Self {
      attack: a,
      decay: d,
      sustain: s,
      release: r
    }
  }

  pub fn amplitude (&self, table: &osc::Wavetable, playing: f32) -> f32 {
    let current = table.phase() / table.sample_rate() as f32;
    println!("{:?}", current);
    if current < self.attack {
      return current;
    }

    if current > self.attack && current < self.sustain {
      return current * self.decay;
    }

    if playing > 0.0 {
      return self.sustain;
    } else {
      return self.release;
    }
  }
}

fn main() {
  let context = pm::PortMidi::new().unwrap();
  let timeout = Duration::from_millis(10);
  const BUF_LEN: usize = 1024;
  let (tx, rx) = mpsc::channel();

  let device = cpal::default_output_device().expect("Failed to get default output device");
  let format = device.default_output_format().expect("Failed to get default output format");

  let event_loop = cpal::EventLoop::new();
  let stream_id = event_loop.build_output_stream(&device, &format).unwrap();

  event_loop.play_stream(stream_id.clone());

  let sample_rate = format.sample_rate.0;

  let mut n3 = osc::Wavetable::new(osc::Waves::SIN, sample_rate as i32);
  let env = Envelope::new(0.2, 0.1, 0.5, 0.3);
  thread::spawn(move || {
    if let Err(e) = misc::play(tx.clone(), false) {
      println!("{:?}", e);
    } else {
      let in_ports = context
                      .devices()
                      .unwrap()
                      .into_iter()
                      .filter_map(|dev| context.input_port(dev, BUF_LEN).ok())
                      .collect::<Vec<_>>();
      loop {
        for port in &in_ports {
          if let Ok(Some(events)) = port.read_n(BUF_LEN) {
            for e in events {
              tx.send(e.message).unwrap();
            }
          }
        }
        thread::sleep(timeout);
      }
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
          let amp = env.amplitude(&n3, last_freq);
          for out in sample.iter_mut() {
            *out = misc::amplify(v3, amp);
          };
        }

      },
      _ => (),
    }
  });

}
