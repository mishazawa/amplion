use std::collections::HashMap;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::{self, Sender, Receiver};
use portmidi::{MidiMessage};
use super::{
  voice::Voice,
  super::midi,
  super::misc,
};


pub const SEQ_LEN: usize = 16;

#[derive(Debug)]
struct Track {
  voice: Voice,
  steps: [bool; SEQ_LEN]
}

impl Track {
  fn new (voice: Voice) -> Self {
    Self {
      voice,
      steps: [false; SEQ_LEN]
    }
  }
}

#[derive(Debug)]
pub struct Sequencer {
  tempo: f32,
  tracks: HashMap<String, Track>,
  pointer: u8,
  playing: bool,
  receiver: Receiver<MidiMessage>,
  pub sender: Sender<MidiMessage>
}

impl Sequencer {
  pub fn new () -> Self {
    let (sender, receiver) = mpsc::channel();
    Self {
      tempo: 110.0,
      tracks: HashMap::new(),
      pointer: 0,
      playing: true,
      receiver,
      sender
    }
  }

  pub fn add (&mut self, identifier: String, voice: Voice) {
    self.tracks.insert(identifier, Track::new(voice));
  }

  pub fn tempo (&mut self, tempo: f32) {
    self.tempo = tempo;
  }

  pub fn enable (&mut self, identifier: String, step: u8) {
    match self.tracks.get_mut(&identifier) {
      Some(track) => {
        track.steps[step as usize] = true;
      },
      None => {
        println!("Track: {:?} doesn't exist", identifier);
      }
    }
  }

  pub fn disable (&mut self, identifier: String, step: u8) {
    match self.tracks.get_mut(&identifier) {
      Some(track) => {
        track.steps[step as usize] = false;
      },
      None => {
        println!("Track: {:?} doesn't exist", identifier);
      }
    }
  }

  pub fn play (&mut self, state: bool) {
    self.playing = state;
  }

  pub fn next_step (&mut self) {
    self.pointer = (self.pointer + 1) % SEQ_LEN as u8;
  }

  pub fn set_params (&mut self, f: impl Fn(&mut Sequencer) -> ()) {
    f(self);
  }

  pub fn run (&mut self, midi_tx: Sender<MidiMessage>) {
    loop {
      if let Ok(mess) = self.receiver.try_recv() {
        match mess.status {
          midi::KNOB_EVENT => {
            match mess.data1 {
              midi::MIDI_MAP_PLAY => self.play(true),
              midi::MIDI_MAP_STOP => self.play(false),
              _ => ()
            }
          },
          _ => ()
        }
      }

      if self.playing == true {
        self.next_step();

        for (_, track) in &self.tracks {
          if track.steps[self.pointer as usize] {
            midi_tx.send(misc::midi_note(track.voice.note, true)).unwrap();
          }
        }

        thread::sleep(Duration::from_millis((60_000.0 / self.tempo) as u64));

        for (_, track) in &self.tracks {
          if track.steps[self.pointer as usize] {
            midi_tx.send(misc::midi_note(track.voice.note, false)).unwrap();
          }
        }
      } else {
        thread::sleep(Duration::from_millis(100));
      }
    }
  }
}


#[cfg(test)]

#[test]
fn it_works() {

  use super::voice::{Voice};
  let mut seq = Sequencer::new();
  seq.tempo(100.5);

  let voice = Voice {
                note: 60,
                freq: 440.0,
                phase: 0.0,
                sample_rate: 44100,
                start_time: 0.0,
                end_time: 0.0,
                enabled: true
              };

  seq.add(String::from("sine"), voice);

  for n in 0..SEQ_LEN {
    if n % 2 == 0 {
      seq.enable(String::from("sine"), n as u8);
    } else {
      seq.disable(String::from("sine"), n as u8);
    }
  }

  seq.play(true);

}
