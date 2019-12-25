use std::fs::File;
use std::io::Read;

pub fn read_file(name: String) -> Vec<f32> {
    let mut f =
        File::open(concat_filename(&name)).expect(format!("Can't open file {}", name).as_ref());
    let mut table = Vec::new();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)
        .expect(format!("Can't read file {}", name).as_ref());

    for byte in buffer.chunks(4) {
        table.push(bin_to_sample(byte))
    }

    table
}

fn concat_filename(name: &String) -> String {
    let mut file_name = String::from("data/");
    file_name.push_str(&name);
    file_name.push_str(".1");
    file_name
}

fn bin_to_sample(bin: &[u8]) -> f32 {
    let mut byte = [0; 4];
    for (i, v) in bin.iter().enumerate() {
        byte[i] = *v;
    }
    f32::from_bits(u32::from_be_bytes(byte))
}
