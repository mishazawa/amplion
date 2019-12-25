/* tmp */
use std::fs::File;
use std::io::Read;

pub fn read_file(name: String) -> Vec<f32> {
    let mut f = File::open(concat_filename(name)).expect("Can't open file");
    let mut table = Vec::new();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).expect("Can't read_to_end");

    for sl in buffer.chunks(4) {
      let mut byte = [0; 4];
      for (i, v) in sl.iter().enumerate() { byte[i] = *v; };
      table.push(bin_to_sample(byte))
    }

    table
}

fn concat_filename(name: String) -> String {
    let mut file_name = String::from("data/");
    file_name.push_str(&name);
    file_name.push_str(".1");
    file_name
}

fn bin_to_sample(bin: [u8; 4]) -> f32 {
    f32::from_bits(u32::from_be_bytes(bin))
}
