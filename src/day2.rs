use super::common;
use std::path::Path;

// Returns a pair of parsed number and new cursor position
fn parse_number(bytes: &Vec<u8>, mut cursor: usize) -> (u32, usize) {
    let mut cur_token = Vec::new();
    cur_token.reserve(32);

    loop {
        if cursor >= bytes.len() {
            break; // The last token in the stream may be a number,
                   // and we have no EOF token to terminate on.
        }

        match bytes[cursor] as char {
            '0'..='9' => {
                let digit_val = bytes[cursor] - ('0' as u8);
                cur_token.push(digit_val);

                cursor += 1;
            },
            '\n' | '\r' => {
                break;
            },
            _ => {
                panic!("Unexpected char \'{}\' at position {}", 
                       bytes[cursor] as char, cursor);
            },
        }
    }
 
    let mut accumulator: u32  = 0;
    let mut position = 1;

    for digit in cur_token.iter().rev() {
        accumulator += (*digit as u32) * position;
        position *= 10;
    }

    (accumulator, cursor)
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day2_input.txt");
    let input_bytes = common::read_input_bytes(input_path.as_path());

    let forward_skip = 7;
    let down_skip = 4;
    let up_skip = 2;

    let use_pitch = true;

    let mut total_z  = 0;
    let mut total_y: i32 = 0;
    let mut pitch: i32 = 0;

    let mut i = 0;
    loop {
        let nav_char = input_bytes[i] as char; 
        match nav_char {
            'f' => i += forward_skip,
            'u' => i += up_skip,
            'd' => i += down_skip,
            _ => {
                panic!("Unexpected char \'{}\' at position {}.", 
                       nav_char, i)
            },
        }

        while input_bytes[i] as char == ' ' {
            i += 1;
        }

        let (mag, new_i) = parse_number(&input_bytes, i);

        i = new_i;
        if use_pitch {
            match nav_char as char {
                'f' => {
                    total_z += mag;
                    total_y += mag as i32 * pitch;
                },
                'u' => pitch -= mag as i32,
                'd' => pitch += mag as i32,
                _ => (),
            }
        } else {
            match nav_char as char {
                'f' => total_z += mag,
                'u' => total_y -= mag as i32,
                'd' => total_y += mag as i32,
                _ => (),
            }
        }
        
        if i >= input_bytes.len() {
            break;
        }

        loop {
            match input_bytes[i] as char {
                '\n' | '\r' | ' ' => i += 1,
                _ => break,
            }
        }
    }

    println!("Total y {} , Total z {}", total_y, total_z);
    println!("Combined {}", total_y * total_z as i32);
}