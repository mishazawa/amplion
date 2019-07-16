extern crate cpal;
extern crate portmidi;
extern crate minifb;


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

use minifb::{Key, WindowOptions, Window};

const WIDTH: usize = 640;
const HEIGHT: usize = 360;

// use std::clone::{ Clone };
use std::sync::mpsc::{ self, Receiver };
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

fn norm (val: f32, min: f32, max: f32, mmin: f32, mmax: f32) -> f32 {
  (mmax - mmin) / (max-min) * (val - max ) + mmax
}


fn draw_fn (dest: &mut [u32], rect: &Vec<(f32,f32)>) {
  let half = HEIGHT / 2;
  let quat = half / 2;

  for (index, (lval, rval)) in rect.iter().enumerate() {


    let x = norm(index as f32, 0.0, rect.len() as f32, 0.0, WIDTH as f32)  as usize;
    let ly = norm(*lval, -1.0, 1.0, -((half + quat) as f32), (half + quat) as f32) as usize;
    let ry = norm(*rval, -1.0, 1.0, -((half + quat) as f32), (half + quat) as f32) as usize;

    let color = 0xfe0000;
    dest[ly * WIDTH + x] = color;
    dest[ry * WIDTH + x] = color;
  }
}



fn ui_thread (receiver: Receiver<Vec<(f32, f32)>>) {
  let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

  let mut window = Window::new("Test - ESC to exit",
                               WIDTH,
                               HEIGHT,
                               WindowOptions::default()).unwrap_or_else(|e| {
      panic!("{}", e);
  });

  while window.is_open() && !window.is_key_down(Key::Escape) {

    if let Ok(data) = receiver.try_recv() {
      draw_fn(&mut buffer, &data);

      window.update_with_buffer(&buffer).unwrap();
    }
  }
  ::std::process::exit(1);

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


  let (ui_tx, ui_rx) = mpsc::channel();
  thread::spawn(move || ui_thread(ui_rx));

  // sound thread
  event_loop.run(move |_, data| {
    match data {
      cpal::StreamData::Output { buffer: cpal::UnknownTypeOutputBuffer::F32(mut buffer) } => {

        // calc delta time
        timer.tick();

        // check midi message
        if let Ok(mess) = midi_rx.try_recv() {
          mel.on_midi_message(mess, timer.get_delta());
          noise.on_midi_message(mess, timer.get_delta());
          on_midi_knob_event(mess);
          seq_tx.send(mess).unwrap();
        }

        let mut data_for_ui = vec![];

        for sample in buffer.chunks_mut(format.channels as usize) {
          let amplitude = mel.get_amp(timer.get_delta());
          let no_amplitude = noise.get_amp(timer.get_delta()) * 0.2;

          let [left, right] = pan.apply(amplitude + no_amplitude * lfo.get_amp());
          data_for_ui.push((left, right));
          // sample[0] = left;
          // sample[1] = right;

        }

        ui_tx.send(data_for_ui).unwrap();
        // release utilised voices (release phase envelope)
        mel.remove_unused(timer.get_delta());
      },
      _ => (),
    }
  });

}




