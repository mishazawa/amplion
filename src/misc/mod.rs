use pm::MidiMessage;
use std::time::Duration;
use std::sync::mpsc;
use std::thread;

pub static CHANNEL: u8 = 0;
pub static MELODY: [(u8, u32); 42] = [(60, 1), (60, 1), (67, 1), (67, 1), (69, 1), (69, 1), (67, 2),
                                  (65, 1), (65, 1), (64, 1), (64, 1), (62, 1), (62, 1), (60, 2),
                                  (67, 1), (67, 1), (65, 1), (65, 1), (64, 1), (64, 1), (62, 2),
                                  (67, 1), (67, 1), (65, 1), (65, 1), (64, 1), (64, 1), (62, 2),
                                  (60, 1), (60, 1), (67, 1), (67, 1), (69, 1), (69, 1), (67, 2),
                                  (65, 1), (65, 1), (64, 1), (64, 1), (62, 1), (62, 1), (60, 2)];

pub fn amplify (v: f32, a: f32) -> f32 { v * a }


pub fn play(tx: mpsc::Sender<MidiMessage>, verbose: bool) -> pm::Result<()> {
    for &(note, dur) in MELODY.iter().cycle() {
        let note_on = MidiMessage {
            status: 0x90 + CHANNEL,
            data1: note,
            data2: 100,
        };

        if verbose {
          println!("{}", note_on)
        }

        tx.send(note_on).unwrap();

        // note hold time before sending note off
        thread::sleep(Duration::from_millis(dur as u64 * 400));

        let note_off = MidiMessage {
            status: 0x80 + CHANNEL,
            data1: note,
            data2: 100,
        };
        if verbose {
            println!("{}", note_off);
        }
        tx.send(note_off).unwrap();

        // short pause
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
