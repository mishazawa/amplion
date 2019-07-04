extern crate cpal;
extern crate portmidi;

mod modules;
mod midi;
mod misc;
mod ui;



use portmidi::{
  MidiMessage,
  PortMidi
};

use std::clone::{ Clone };
use std::sync::mpsc::{self, Sender};
use std::thread;

use modules::{
  mixer::Mixer,
  voice::Voice,
  wavetable::Wavetable,
  wavetable::Waves,
  envelope::Envelope,
  timer::Timer,
  sequencer::{Sequencer, SEQ_LEN}
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

fn on_midi_keyboard_event (mess: MidiMessage, mixer: &mut Mixer, delta_time: f32, osc_sample_rate: i32) {
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
        sample_rate: osc_sample_rate,
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
      println!("KE{:?}", mess.data1);
      match mess.data1 {
        _ => {}
      }
    },
    _ => {
      println!("{:?}", mess.status);
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

  // midi setup
  let context = PortMidi::new().unwrap();
  let (midi_tx, midi_rx) = mpsc::channel();

  // synth setup
  let n3 = Wavetable::new(Waves::SIN, sample_rate);
  let mut env = Envelope::new();

  env.set_params(0.4, 0.4, 0.7, 1.0);
  env.set_amps(0.8, 0.7);

  let mut timer = Timer::new();
  let mut mixer = Mixer::new();

  // ui setup
  let ui: UiThread = UiThread::new();
  // let cursive_sender = ui.sender().clone();

  // ui thread
  // thread::spawn(move || UiThread::run(cursive_sender));


  let mut seq = Sequencer::new();

  seq.set_params(move |s: &mut Sequencer| {
    s.tempo(300.5);
    let voice = Voice {
      note: 60,
      freq: 440.0,
      phase: 0.0,
      sample_rate: 44100,
      start_time: 0.0,
      end_time: 0.0,
      enabled: true
    };

    s.add(String::from("sine"), voice);

    for n in 0..SEQ_LEN {
      if n % 2 == 0 {
        s.enable(String::from("sine"), n as u8);
      } else {
        s.disable(String::from("sine"), n as u8);
      }
    }
  });

  let seq_tx = seq.sender.clone();

  thread::spawn(move || {
    seq.run();
  });

  // midi thread
  #[allow(unreachable_code)]
  thread::spawn(move || {
    // return;
    // if let Err(e) = misc::play(midi_tx.clone(), false) {
    //   println!("{:?}", e);
    // } else {
      midi::read_midi_ports(context, midi_tx.clone());
    // }
  });

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
          on_midi_keyboard_event(mess, &mut mixer, timer.get_delta(), n3.sample_rate());
          on_midi_knob_event(mess);
          seq_tx.send(mess).unwrap();
        }

        for sample in buffer.chunks_mut(format.channels as usize) {
          let amplitude = mixer.mix(&n3, &env, timer.get_delta());
          for out in sample.iter_mut() {
            *out = amplitude;
          };
        }

        // release utilised voices (release phase envelope)
        mixer.remove_unused(&env, timer.get_delta());
      },
      _ => (),
    }
  });

}




