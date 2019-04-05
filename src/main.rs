use io::Read;
use std::{env, fs, io};

mod opcode;
use opcode::Opcode;

fn main() -> Result<(), String> {
    let filename = env::args().nth(1).ok_or("Please provide a filename")?;
    let metadata = fs::metadata(&filename).or(Err("Could not read file metadata"))?;
    let mut file = fs::File::open(filename).or(Err("Could not open file"))?;

    let mut buffer: Vec<u8> = vec![0; metadata.len() as usize];
    file.read(&mut buffer).or(Err("Error reading file"))?;

    for i in (0..metadata.len()).step_by(2) {
        let hi = buffer[i as usize];
        let lo = buffer[i as usize + 1];
        println!(
            "{:04x} {:02x} {:02x} {:?}",
            0x200 + i,
            hi,
            lo,
            Opcode::from([hi, lo])
        );
    }
    Ok(())
}
