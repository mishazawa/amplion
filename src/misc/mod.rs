#![allow(dead_code)]
use pm::MidiMessage;
use std::time::Duration;
use std::sync::mpsc;
use std::thread;

pub static LEDS_TOP_ROW: [u8; 9] = [96, 97, 98, 99, 100, 101, 102, 103, 104];
pub static LEDS_BOTTOM_ROW: [u8; 9] = [112, 113, 114, 115, 116, 117, 118, 119, 120];

pub static CHANNEL: u8 = 0;

pub static MELODY: [(u8, u32); 42] = [(60, 1), (60, 1), (67, 1), (67, 1), (69, 1), (69, 1), (67, 2),
                                  (65, 1), (65, 1), (64, 1), (64, 1), (62, 1), (62, 1), (60, 2),
                                  (67, 1), (67, 1), (65, 1), (65, 1), (64, 1), (64, 1), (62, 2),
                                  (67, 1), (67, 1), (65, 1), (65, 1), (64, 1), (64, 1), (62, 2),
                                  (60, 1), (60, 1), (67, 1), (67, 1), (69, 1), (69, 1), (67, 2),
                                  (65, 1), (65, 1), (64, 1), (64, 1), (62, 1), (62, 1), (60, 2)];



const PLAY_TIME: u64 = 400;

const PAUSE_TIME: u64 = 100;


fn midi_note (note: u8, trigger: bool) -> MidiMessage {
  MidiMessage {
    status: (if trigger {0x90} else {0x80}) + CHANNEL,
    data1: note,
    data2: 100,
  }
}

pub fn play(tx: mpsc::Sender<MidiMessage>, verbose: bool) -> pm::Result<()> {
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
