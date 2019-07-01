use std::collections::HashMap;
use super::{ voice::Voice };


const SEQ_LEN: usize = 16;

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
  playing: bool
}

impl Sequencer {
  pub fn new () -> Self {
    Self {
      tempo: 110.0,
      tracks: HashMap::new(),
      pointer: 0,
      playing: false
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

  pub fn next_step (&mut self) -> Vec<&String> {
    self.pointer = (self.pointer + 1) % SEQ_LEN as u8;

    let mut tracks_to_play = Vec::new();

    for (key, track) in &self.tracks {
      if track.steps[self.pointer as usize] {
        tracks_to_play.push(key);
      }
    }
    tracks_to_play
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

  assert_eq!(seq.next_step().len(), 0);
  assert_eq!(seq.next_step().len(), 1);
}
