#![allow(dead_code)]
use portmidi::{MidiMessage, Result};
use std::time::Duration;
use std::sync::mpsc;
use std::thread;

use crate::modules::{
  voice::Voice,
  sequencer::{Sequencer, tab_to_sequence},
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


const PLAY_TIME: u64 = 400;

const PAUSE_TIME: u64 = 100;


pub fn midi_note (note: u8, trigger: bool) -> MidiMessage {
  MidiMessage {
    status: (if trigger {0x90} else {0x80}) + CHANNEL,
    data1: note,
    data2: 100,
  }
}


pub fn seq_demo (s: &mut Sequencer) {
  s.tempo(10.5);

  s.add(String::from("sine"), Voice {
    note: 89,
    freq: 440.0,
    phase: 0.0,
    sample_rate: 44100,
    start_time: 0.0,
    end_time: 0.0,
    enabled: true
  });

  s.add(String::from("cosine"), Voice {
    note: 53,
    freq: 440.0,
    phase: 0.0,
    sample_rate: 44100,
    start_time: 0.0,
    end_time: 0.0,
    enabled: true
  });

  s.add(String::from("asine"), Voice {
    note: 93,
    freq: 440.0,
    phase: 0.0,
    sample_rate: 44100,
    start_time: 0.0,
    end_time: 0.0,
    enabled: true
  });


  tab_to_sequence(s, String::from("sine"),   [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0]);
  tab_to_sequence(s, String::from("asine"),  [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]);
  tab_to_sequence(s, String::from("cosine"), [1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 1, 1, 1]);
}

pub fn play(tx: mpsc::Sender<MidiMessage>, verbose: bool) -> Result<()> {
  for &(note, dur) in MELODY.iter().cycle() {

    let note1_on = midi_note(note, true);
    let note2_on = midi_note(note - 7, true);


    if verbose {
      println!("on -> {}, {}", note1_on, note2_on);
    }

    tx.send(note1_on).unwrap();
    thread::sleep(Duration::from_millis(dur as u64 * PLAY_TIME));

    tx.send(note2_on).unwrap();
    thread::sleep(Duration::from_millis(dur as u64 * PLAY_TIME));

    let note1_off = midi_note(note, false);
    let note2_off = midi_note(note - 7, false);

    if verbose {
      println!("off -> {}, {}", note1_off, note2_off);
    }

    tx.send(note1_off).unwrap();
    tx.send(note2_off).unwrap();

    // short pause
    thread::sleep(Duration::from_millis(PAUSE_TIME));
  }
  Ok(())
}
