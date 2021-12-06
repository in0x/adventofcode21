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

// Returns a pair of the parsed number and the new cursor. The cursor
// will sit at the first position past the parsed number.
pub fn parse_num(bytes: &Vec<u8>, token_buf: &mut Vec<u8>, mut cursor: usize) -> (Option<u32>, usize) {
    while cursor < bytes.len() {
        match bytes[cursor] as char {
            '0'..='9' => {
                let digit_val = bytes[cursor] - ('0' as u8);
                token_buf.push(digit_val);    
            },
            _ => break,
        }

        cursor += 1;
    }

    if token_buf.len() > 0 {
        let num = build_u32(token_buf);
        token_buf.clear();
        (Some(num), cursor)
    } else {
        (None, cursor)
    }
}