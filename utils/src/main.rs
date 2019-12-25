use core::f32::consts::PI;
use std::env;
use std::fs::File;
use std::io::Write;

pub const SAMPLE_RATE: i32 = 44_100;

fn sine_point(n: f32) -> f32 {
    (n * 2.0 * PI / SAMPLE_RATE as f32).sin()
}

fn square_wave(harmonics: usize) -> Vec<f32> {
    (0..SAMPLE_RATE)
        .map(|x| {
            (1..harmonics).step_by(2).fold(0., |acc, n| {
                acc + sine_point(x as f32 * n as f32) / n as f32
            })
        })
        .collect::<Vec<f32>>()
}

fn sine_wave() -> Vec<f32> {
    (0..SAMPLE_RATE)
        .map(|x| sine_point(x as f32))
        .collect::<Vec<f32>>()
}

fn main() -> () {
    let mut arguments = env::args().into_iter();
    if arguments.any(|arg| arg == "all") {
        println!("{:?}", "Build all");
    } else {
        for arg in env::args() {
            match arg.as_ref() {
                "sine" | "sin" | "sinus" => {
                    println!("Build {}", arg);
                    write_sine(arg.to_string()).expect("Fail to write_sine");
                }
                "square" | "sq" => {
                    println!("Build {}", arg);
                    write_square(arg.to_string()).expect("Fail to write_square");
                }
                _ => (),
            }
        }
    }
}

fn sample_to_bin(sample: f32) -> Vec<u8> {
    sample.to_bits().to_be_bytes().to_vec()
}

fn bin_to_sample(bin: [u8; 4]) -> f32 {
    f32::from_bits(u32::from_be_bytes(bin))
}

fn write_table(samples: &[u8], name: String) -> std::io::Result<()> {
    let mut file_name = String::from("data/");
    file_name.push_str(&name);
    file_name.push_str(".1");

    let mut buffer = File::create(file_name).expect("Can't create");
    buffer.write_all(samples.as_ref()).expect("Can't write_all");
    Ok(())
}

fn write_sine(name: String) -> std::io::Result<()> {
    let mut array: Vec<u8> = vec![];

    for sample in sine_wave().iter() {
        array.append(&mut sample_to_bin(*sample));
    }

    write_table(&array.as_slice(), name)
}

fn write_square(name: String) -> std::io::Result<()> {
    let mut array: Vec<u8> = vec![];

    for sample in square_wave(128).iter() {
        array.append(&mut sample_to_bin(*sample));
    }

    write_table(&array.as_slice(), name)
}
