use std::collections::HashMap;
use super::{ voice::Voice };

#[derive(Debug)]
struct Track {
  voice: Voice,
  steps: [bool; 16]
}

#[derive(Debug)]
pub struct Sequencer {
  tempo: f32,
  tracks: HashMap<String, Track>
}

impl Track {
  fn new (voice: Voice) -> Self {
    Self {
      voice,
      steps: [false; 16]
    }
  }
}

impl Sequencer {
  pub fn new () -> Self {
    Self {
      tempo: 110.0,
      tracks: HashMap::new()
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
}
