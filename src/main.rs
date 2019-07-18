extern crate cpal;
extern crate portmidi;

mod modules;
mod midi;
mod misc;

pub const SAMPLE_RATE: i32 = 44_100;
// pub static SAMPLE_RATE: i32 = 22_050;
// pub static SAMPLE_RATE: i32 = 11_025;

use portmidi::{
  MidiMessage,
  PortMidi
};


// use std::clone::{ Clone };
use std::sync::mpsc::{ self };
use std::thread;

use modules::{
  mixer::Mixer,
  voice::{Voice},
  timer::Timer,
  envelope::Envelope,
  wavetable::{ Waves, Osc },
  sequencer::{ Sequencer },
  instrument::{ Instrument },
  lfo::{ Lfo },
  panorama::{ Panorama },
};


fn on_midi_keyboard_event (mess: MidiMessage, mixer: &mut Mixer, delta_time: f32) {
  match mess.status {
    midi::KEY_DEPRESS => {
      for voice in mixer.voices.iter_mut() {
        if voice.note == mess.data1 && voice.enabled{
          voice.end_time = delta_time;
          voice.enabled = false;
        }
      }
    },

    midi::KEY_PRESS => {
      mixer.add(Voice {
        note: mess.data1,
        freq: midi::midi_to_freq(mess.data1),
        start_time: delta_time,
        end_time: 0.0,
        enabled: true,
        osc: sine!(),
      });
    },
    _ => ()
  }
}

fn on_midi_knob_event(mess: MidiMessage) {
  match mess.status {
    midi::PAD_PRESS => {
      println!("PP{:?}", mess.data1);
    },
    midi::PAD_DEPRESS => {
      println!("PD{:?}", mess.data1);
    }
    midi::KNOB_EVENT => {
      // println!("KE{:?}", mess.data1);
      // match mess.data1 {
      //   _ => {}
      // }
    },
    _ => {
      // println!("{:?}", mess.status);
    }
  }
}
use cpal::traits::{DeviceTrait, EventLoopTrait, HostTrait};

fn main() {
  // sound setup

  let host = cpal::default_host();
  let device = host.default_output_device().expect("failed to find a default output device");
  let format = device.default_output_format().expect("failed to find a default output format");
  let event_loop = host.event_loop();
  let stream_id = event_loop.build_output_stream(&device, &format).expect("failed to build output stream");
  event_loop.play_stream(stream_id.clone()).expect("failed to play stream");


  let sample_rate = format.sample_rate.0 as i32;
  println!("sample rate: {:?} by {:?}", sample_rate, SAMPLE_RATE);
  // midi setup
  let context = PortMidi::new().unwrap();
  let (midi_tx, midi_rx) = mpsc::channel();

  // synth setup

  let mut mel = Instrument {
    polyphony: Mixer::new(),
    osc: vec![
      sine!(),
      triangle!(),
    ],
    envelope: Envelope::new(|mut env: Envelope| -> Envelope {
      env.set_params(0.6, 0.4, 0.7, 0.5);
      env.set_amps(0.8, 0.7);
      // env.set_plucked(0.05);
      env
    }),
    on_midi_event: on_midi_keyboard_event
  };

  let mut noise = Instrument {
    polyphony: Mixer::new(),
    osc: vec![
      noise!(),
    ],
    envelope: Envelope::new(|mut env: Envelope| -> Envelope {
      env.set_params(0.6, 0.4, 0.7, 1.2);
      env.set_amps(0.8, 0.7);
      env
    }),
    on_midi_event: on_midi_keyboard_event
  };


  let mut timer = Timer::new();
  let mut lfo = Lfo::new(1.03);


  let pan = Panorama::new();

  // midi thread
  let keyboard_midi_tx = midi_tx.clone();
  #[allow(unreachable_code)]
  thread::spawn(move || {

    // if let Err(e) = misc::play(keyboard_midi_tx, false) {
    //   println!("{:?}", e);
    // }
    midi::read_midi_ports(context, keyboard_midi_tx);
  });

  let mut seq = Sequencer::new();
  seq.set_params(misc::seq_demo);
  let seq_tx = seq.sender.clone();
  let seq_midi_tx = midi_tx.clone();
  thread::spawn(move || seq.run(seq_midi_tx));

  // sound thread
  let (sound_tx, sound_rx) = mpsc::channel();
  thread::spawn(move || {
    event_loop.run(move |id, result| {
      let data = match result {
        Ok(data) => data,
        Err(err) => {
          eprintln!("an error occurred on stream {:?}: {}", id, err);
          return;
        }
      };

      // calc delta time
      timer.tick();

      let delta = timer.get_delta();

      // check midi message
      if let Ok(mess) = midi_rx.try_recv() {
        mel.on_midi_message(mess, timer.get_delta());
        noise.on_midi_message(mess, timer.get_delta());
        on_midi_knob_event(mess);
        seq_tx.send(mess).unwrap();
      }

      match data {
        cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {
          buffer.chunks_mut(format.channels as usize).for_each(|sample| {
            let amplitude = mel.get_amp(delta) * 0.2;
            let no_amplitude = noise.get_amp(delta) * 0.2;
            pan.apply(sample, amplitude + no_amplitude * lfo.get_amp());
          });
        },
        _ => (),
      }
      // release utilised voices (release phase envelope)
      mel.remove_unused(timer.get_delta());

      if delta >= 10_000.0 {
        sound_tx.send(true).unwrap();
      }
    });
  });

  loop {
    if let Ok(data) = sound_rx.try_recv() {
      println!("{:?}", data);
      std::process::exit(0);
    }
  }

}




