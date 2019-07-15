use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Result};
use std::time::Duration;
use std::sync::mpsc::{self, Sender, Receiver};
use portmidi::{MidiMessage};
use crate::{
  modules::timer::Timer,

  midi,
  misc,
};


pub const SEQ_LEN: usize = 16;

pub struct Track {
  note: u8,
  steps: [bool; SEQ_LEN]
}

impl Track {
  fn new (note: u8) -> Self {
    Self {
      note,
      steps: [false; SEQ_LEN]
    }
  }
}

impl Debug for Track {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    for track in self.steps.iter() {
      write!(f, "{}", if *track {"x"} else {"_"})?;
    }
    write!(f, "")
  }
}

pub struct Sequencer {
  tempo: f32,
  tracks: HashMap<String, Track>,
  pointer: u8,
  playing: bool,
  receiver: Receiver<MidiMessage>,
  pub sender: Sender<MidiMessage>,
  timer: Timer
}

impl Debug for Sequencer {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      misc::clear_terminal();
      write!(f, "tempo: {}\n", self.tempo)?;
      for track in self.tracks.iter() {
        write!(f, "|{:?}|  {}\n", track.1, track.0)?;

      }
      write!(f, "{:?}", format_args!("{:ident$}=\n", "", ident=(self.pointer + 1) as usize))
  }
}


impl Sequencer {
  pub fn new () -> Self {
    let (sender, receiver) = mpsc::channel();
    Self {
      tempo: 110.0,
      tracks: HashMap::new(),
      pointer: (SEQ_LEN - 1) as u8,
      playing: true,
      receiver,
      sender,
      timer: Timer::new()
    }
  }

  pub fn add (&mut self, identifier: String, note: u8) {
    self.tracks.insert(identifier, Track::new(note));
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

  pub fn play (&mut self, tx: Option<&Sender<MidiMessage>>) {
    if let Some(midi_tx) = tx {
      if self.playing == true {
        self.playing = false;
        self.playing_state_off(&midi_tx);
        self.reset_pointer();
      }
    } else {
      self.playing = true;
    }
  }

  pub fn next_step (&mut self) {
    self.pointer = (self.pointer + 1) % SEQ_LEN as u8;
    println!("{:?}", self);
  }

  pub fn set_params (&mut self, f: impl Fn(&mut Sequencer) -> ()) {
    f(self);
  }

  pub fn run (&mut self, midi_tx: Sender<MidiMessage>) {
    loop {
      self.get_midi_event(&midi_tx);

      let tempo_time = Duration::from_millis((60_000.0 / self.tempo) as u64);

      if self.playing {
        let spend_time = Duration::from_millis(self.timer.get_delta() as u64);

        if spend_time >= tempo_time {
          self.playing_state_off(&midi_tx);
          self.timer.flush();
          self.next_step();
          self.playing_state_on(&midi_tx);
        }

        self.timer.tick();
      }
    }
  }

  pub fn get_tracks (&self) -> &HashMap<String, Track> {
    &self.tracks
  }

  fn get_midi_event (&mut self, midi_tx: &Sender<MidiMessage>) {
      if let Ok(mess) = self.receiver.try_recv() {
        match mess.status {
          midi::KNOB_EVENT => {
            match mess.data1 {
              midi::MIDI_MAP_STOP => self.play(Some(&midi_tx)),
              midi::MIDI_MAP_PLAY => self.play(None),
              _ => ()
            }
          },
          _ => ()
        }
      }
  }

  fn playing_state_on (&self, midi_tx: &Sender<MidiMessage>) {
    for (_, track) in &self.tracks {
      if track.steps[self.pointer as usize] {
        // println!("-> on {:?}", self.pointer);
        midi_tx.send(misc::midi_note(track.note, true)).unwrap();
      }
    }
  }

  fn playing_state_off (&self, midi_tx: &Sender<MidiMessage>) {
    for (_, track) in &self.tracks {
      if track.steps[self.pointer as usize] {
        // println!("-> off {:?}", self.pointer);

        midi_tx.send(misc::midi_note(track.note, false)).unwrap();
      }
    }
  }

  fn reset_pointer (&mut self) {
    self.pointer = (SEQ_LEN - 1) as u8;
  }
}

pub fn tab_to_sequence (seq: &mut Sequencer, key: String, arr: [u8; SEQ_LEN]) {
  for n in 0..SEQ_LEN {
    seq.disable(key.to_string(), n as u8);
    if arr[n] != 0 {
      seq.enable(key.to_string(), n as u8);
    }
  }
}


pub fn string_to_sequence (seq: &mut Sequencer, key: String, arr: String) {
  for (i, n) in arr.chars().enumerate() {
    if n == '0' {
      seq.disable(key.to_string(), i as u8);
    } else {
      seq.enable(key.to_string(), i as u8);
    }
  }
}
