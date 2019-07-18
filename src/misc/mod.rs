#![allow(dead_code)]
use portmidi::{MidiMessage, Result};
use std::time::Duration;
use std::sync::mpsc;
use std::thread;

use crate::modules::{
  sequencer::{Sequencer, string_to_sequence},
};

pub static LEDS_TOP_ROW: [u8; 9] = [96, 97, 98, 99, 100, 101, 102, 103, 104];
pub static LEDS_BOTTOM_ROW: [u8; 9] = [112, 113, 114, 115, 116, 117, 118, 119, 120];

pub static CHANNEL: u8 = 0;

pub static MELODY: [(u8, u32); 42] = [
  (60, 1), (60, 1), (67, 1), (67, 1), (69, 1), (69, 1), (67, 2),
  (65, 1), (65, 1), (64, 1), (64, 1), (62, 1), (62, 1), (60, 2),
  (67, 1), (67, 1), (65, 1), (65, 1), (64, 1), (64, 1), (62, 2),
  (67, 1), (67, 1), (65, 1), (65, 1), (64, 1), (64, 1), (62, 2),
  (60, 1), (60, 1), (67, 1), (67, 1), (69, 1), (69, 1), (67, 2),
  (65, 1), (65, 1), (64, 1), (64, 1), (62, 1), (62, 1), (60, 2)
  ];


const PLAY_TIME: u64 = 1000;

const PAUSE_TIME: u64 = 100;


pub fn clear_terminal () {
  print!("{}[2J", 27 as char);
}

pub fn midi_note (note: u8, trigger: bool) -> MidiMessage {
  MidiMessage {
    status: (if trigger {0x90} else {0x80}) + CHANNEL,
    data1: note,
    data2: 100,
  }
}

pub fn seq_demo (s: &mut Sequencer) {

  s.debug = false;

  s.tempo(500.0);

  s.add(String::from("AAA"), 63);
  s.add(String::from("AAB"), 65);
  s.add(String::from("ABB"), 61);
  s.add(String::from("BBB"), 34);

  string_to_sequence(s, String::from("AAA"), "1111000000000011".to_string());
  string_to_sequence(s, String::from("AAB"), "0000111100001100".to_string());
  string_to_sequence(s, String::from("ABB"), "1111000011110000".to_string());
  string_to_sequence(s, String::from("BBB"), "1000000010000000".to_string());
  // tab_to_sequence(s, String::from("sine"),   [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
  // tab_to_sequence(s, String::from("cosine"), [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
  // tab_to_sequence(s, String::from("asine"),  [0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0]);
}

pub fn play(tx: mpsc::Sender<MidiMessage>, verbose: bool) -> Result<()> {
  for &(note, dur) in MELODY.iter().cycle() {

    let note1_on = midi_note(note, true);

    if verbose {
      println!("on -> {}", note1_on);
    }

    tx.send(note1_on).unwrap();
    thread::sleep(Duration::from_millis(dur as u64 * PLAY_TIME));

    let note1_off = midi_note(note, false);

    if verbose {
      println!("off -> {}", note1_off);
    }

    tx.send(note1_off).unwrap();


    // short pause
    thread::sleep(Duration::from_millis(PLAY_TIME));
  }
  Ok(())
}
