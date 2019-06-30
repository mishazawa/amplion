extern crate cpal;
extern crate portmidi as pm;
extern crate rand;

mod modules;
mod midi;
mod misc;

use std::clone::{Clone};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration};
use modules::{
  mixer::Mixer,
  voice::Voice,
  wavetable::Wavetable,
  wavetable::Waves,
  envelope::Envelope,
  timer::Timer,
};


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

  let sample_rate = format.sample_rate.0 as i32;

  let n3 = Wavetable::new(Waves::SIN, sample_rate);
  let mut env = Envelope::new();

  env.set_params(0.4, 0.4, 0.7, 1.0);
  env.set_amps(0.8, 0.7);

  thread::spawn(move || {
    // if let Err(e) = misc::play(tx.clone(), false) {
      // println!("{:?}", e);
    // } else {
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
    // }

  });

  let mut timer = Timer::new();
  let mut mixer = Mixer::new();

  event_loop.run(move |_, data| {
    match data {
      cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {

        timer.tick();

        if let Ok(mess) = rx.try_recv() {
          match mess.status {

            midi::KEY_DEPRESS => {
              match mixer.voices.get_mut(&mess.data1) {
                Some(voice) => {
                  voice.end_time = timer.get_delta();
                  voice.enabled = false;
                },
                None => println!("Midi {} is not pressed.", &mess.data1)
              }
            },

            midi::KEY_PRESS => {
              mixer.add(Voice {
                note: mess.data1,
                freq: midi::midi_to_freq(mess.data1),
                phase: 0.0,
                sample_rate: n3.sample_rate(),
                start_time: timer.get_delta(),
                end_time: 0.0,
                enabled: true
              });
            },

            _ => ()
          }
        }

        for sample in buffer.chunks_mut(format.channels as usize) {
          let amplitude = mixer.mix(&n3, &env, timer.get_delta());
          for out in sample.iter_mut() {
            *out = Mixer::amplify(amplitude, 1.0);
          };
        }

        mixer.remove_unused(&env, timer.get_delta());
      },
      _ => (),
    }
  });

}
