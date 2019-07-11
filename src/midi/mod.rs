extern crate portmidi;

use portmidi::{PortMidi, MidiMessage};
use std::thread;
use std::sync::mpsc::{Sender};
use std::time::{Duration};
use std::collections::{HashMap};

const BUF_LEN: usize = 1024;
const MIDI_OCTAVES_START: usize = 21;
const MIDI_OCTAVES_END: usize = 108;


#[derive(Debug)]
pub struct MidiFreqTable(HashMap<u8, f32>);
impl MidiFreqTable {
  pub fn new () -> Self {
    let mut notes = HashMap::new();
    for n in MIDI_OCTAVES_START..MIDI_OCTAVES_END {
      notes.insert(n as u8, midi_to_freq(n as u8));
    }
    Self(notes)
  }

  pub fn lookup (&self, key: u8) -> f32 {
    *self.0.get(&key).unwrap()
  }
}


pub fn midi_to_freq (key: u8) -> f32 {
  (440.0 * (2.0f32).powf((key as f32 - 69.0) / 12.0)).floor()
}

pub const KEY_PRESS:u8 = 144;
pub const KEY_DEPRESS:u8 = 128;
pub const PAD_PRESS:u8 = 153;
pub const PAD_DEPRESS:u8 = 137;
pub const KNOB_EVENT:u8 = 176;


pub const MIDI_MAP_PLAY:u8 = 115;
pub const MIDI_MAP_STOP:u8 = 114;

pub fn read_midi_ports (context: PortMidi, midi_tx: Sender<MidiMessage>) {
  let timeout = Duration::from_millis(10);

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
          midi_tx.send(e.message).unwrap();
        }
      }
    }
    thread::sleep(timeout);
  }
}
