extern crate cpal;
extern crate portmidi;

mod modules;
mod midi;
mod misc;
mod ui;

pub static SAMPLE_RATE: i32 = 44_100;
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
  voice::Voice,
  timer::Timer,
  envelope::Envelope,
  wavetable::{ Wavetable, Waves },
  sequencer::{ Sequencer },
  instrument::{ Instrument },
  lfo::{ Lfo }
};

use ui::{
  UiThread,
  UiMessage
};

fn on_ui_message_event (mess: UiMessage) {
  match mess {
    UiMessage::QUIT => {
      std::process::exit(0);
    }
  }
}

fn on_midi_keyboard_event (mess: MidiMessage, mixer: &mut Mixer, delta_time: f32) {
  match mess.status {
    midi::KEY_DEPRESS => {
      match mixer.voices.get_mut(&mess.data1) {
        Some(voice) => {
          voice.end_time = delta_time;
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
        start_time: delta_time,
        end_time: 0.0,
        enabled: true
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


fn main() {
  // sound setup
  let device = cpal::default_output_device().expect("Failed to get default output device");
  let format = device.default_output_format().expect("Failed to get default output format");
  let event_loop = cpal::EventLoop::new();
  let stream_id = event_loop.build_output_stream(&device, &format).unwrap();

  event_loop.play_stream(stream_id.clone());

  let sample_rate = format.sample_rate.0 as i32;
  println!("{:?}", sample_rate);
  // midi setup
  let context = PortMidi::new().unwrap();
  let (midi_tx, midi_rx) = mpsc::channel();

  // synth setup

  let mut mel = Instrument {
    polyphony: Mixer::new(),
    osc: vec![
      Wavetable::new(Waves::SIN),
      Wavetable::new(Waves::TRI),
    ],
    envelope: Envelope::new(|mut env: Envelope| -> Envelope {
      env.set_params(0.6, 0.4, 0.7, 1.2);
      env.set_amps(0.8, 0.7);
      env
    }),
    on_midi_event: on_midi_keyboard_event
  };

  let mut noise = Instrument {
    polyphony: Mixer::new(),
    osc: vec![
      Wavetable::new(Waves::NO),
    ],
    envelope: Envelope::new(|mut env: Envelope| -> Envelope {
      env.set_params(0.6, 0.4, 0.7, 1.2);
      env.set_amps(0.8, 0.7);
      env
    }),
    on_midi_event: on_midi_keyboard_event
  };


  let mut timer = Timer::new();
  let mut lfo = Lfo::new(0.03);
  // ui setup
  let ui: UiThread = UiThread::new();
  // let cursive_sender = ui.sender().clone();

  // ui thread
  // thread::spawn(move || UiThread::run(cursive_sender));

  // midi thread
  let keyboard_midi_tx = midi_tx.clone();
  #[allow(unreachable_code)]
  thread::spawn(move || {

    midi::read_midi_ports(context, keyboard_midi_tx);
    // if let Err(e) = misc::play(midi_tx.clone(), false) {
    //   println!("{:?}", e);
    // }
  });

  let mut seq = Sequencer::new();
  seq.set_params(misc::seq_demo);
  let seq_tx = seq.sender.clone();
  let seq_midi_tx = midi_tx.clone();
  thread::spawn(move || seq.run(seq_midi_tx));


  // sound thread
  event_loop.run(move |_, data| {
    match data {
      cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {

        // calc delta time
        timer.tick();

        // check ui message
        if let Ok(mess) = ui.receiver().try_recv() {
          on_ui_message_event(mess);
        }

        // check midi message
        if let Ok(mess) = midi_rx.try_recv() {
          mel.on_midi_message(mess, timer.get_delta());
          noise.on_midi_message(mess, timer.get_delta());
          on_midi_knob_event(mess);
          seq_tx.send(mess).unwrap();
        }

        for sample in buffer.chunks_mut(format.channels as usize) {
          let amplitude = mel.get_amp(timer.get_delta());
          let no_amplitude = noise.get_amp(timer.get_delta()) * 0.2;
          for out in sample.iter_mut() {
            *out = amplitude + no_amplitude * lfo.get_amp();
          };
        }

        // release utilised voices (release phase envelope)
        mel.remove_unused(timer.get_delta());
      },
      _ => (),
    }
  });

}




