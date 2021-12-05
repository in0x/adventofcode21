use std::fs;
use std::path::Path;

// We're going to assume the asked for file contains no multi-byte characters.
pub fn read_input_bytes(file_path: &Path) -> Vec<u8> {
    match fs::read(&file_path) {
        Err(why) => panic!("Failed to open input file {}: {}",
                           file_path.to_str().unwrap(), why),
        Ok(bytes) => bytes
    }
}

pub fn build_u32(digits: &Vec<u8>) -> u32 {
    let mut accumulator: u32  = 0;
    let mut position = 1;

    for digit in digits.iter().rev() {
        accumulator += (*digit as u32) * position;
        position *= 10;
    }

    accumulator
}