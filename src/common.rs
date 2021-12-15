use std::fs;
use std::path::Path;

/// We're going to assume the asked for file contains no multi-byte characters.
pub fn read_input_bytes(file_path: &Path) -> Vec<u8> {
    match fs::read(&file_path) {
        Err(why) => panic!("Failed to open input file {}: {}",
                           file_path.to_str().unwrap(), why),
        Ok(bytes) => bytes
    }
}

/// Returns a triple of row-major grid of values, width and height of the grid.
pub fn parse_grid(bytes: &Vec<u8>) -> (Vec<u8>, usize, usize) {    
    let width = bytes.iter()
        .position(|c| c.is_ascii_whitespace()).unwrap();

    let grid = bytes.iter()
        .filter(|c| !c.is_ascii_whitespace())
        .map(|c| c - ('0' as u8))
        .collect::<Vec<_>>();

    let height = grid.len() / width;

    (grid, width, height)
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

/// Returns a pair of the parsed number and the new cursor. The cursor
/// will sit at the first position past the parsed number.
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

pub fn read_list_of_csv_u32s(bytes: &Vec<u8>) -> Vec<u32> {
    let mut cursor = 0;    
    let mut values: Vec<u32> = Vec::new();
    let mut token_buf = Vec::new();
    token_buf.reserve(32);

    while cursor < bytes.len() {
        match parse_num(&bytes, &mut token_buf, cursor) {
            (Some(num), new_cursor) => {
                values.push(num);
                cursor = new_cursor;
            },
            (None, _) => {
                panic!("We should have a number at each scan, but failed at cursor pos {}", cursor);
            } 
        }
        cursor += 1;
    }

    values
    
}

pub fn get_grid_idx(x: usize, y: usize, width: usize) -> usize {
    x + (y * width) 
}

/// Returns the (x, y) coordinates represented by the idx for a grid
/// of size (width, height). The grid is assumed to be stored row-major.
pub fn get_grid_xy(idx: usize, width: usize, height: usize) -> (usize, usize) {
    let y = idx / width;
    let x = idx - (y * width);
    (x, y)
}

/// Generates valid taps for a cross filter at a position (at idx) for a grid of size 
/// (width, height). A tap is invalid if it falls outside the grid (edges and corners).
/// Does not include self as a tap.
pub fn get_cross_taps(idx: usize, width: usize, height: usize) -> [Option<usize>; 4] {
    let (x, y) = {
        let (x, y) = get_grid_xy(idx, width, height);
        (x as i32, y as i32)
    };

    let mut coords: [Option<usize>; 4] = Default::default();
    let mut coord_i = 0;

    let potential_taps = [(x + 1, y), (x - 1, y),
                          (x, y + 1), (x, y - 1)];

    for tap in potential_taps {
        if tap.0 < 0 || tap.0 >= width as i32 {
            continue;
        }
    
        if tap.1 < 0 || tap.1 >= height as i32 {
            continue;
        }

        let tap = tap.0 as usize + (tap.1 as usize * width);
        coords[coord_i] = Some(tap);
        coord_i += 1;
    }
    
    coords
}

/// Generates valid taps for a box filter at a position (at idx) for a grid of size 
/// (width, height). A tap is invalid if it falls outside the grid (edges and corners).
pub fn get_box_taps(idx: usize, width: usize, height: usize) -> [Option<usize>; 9] {
    let (x, y) = {
        let (x, y) = get_grid_xy(idx, width, height);
        (x as i32, y as i32)
    };

    let mut coords: [Option<usize>; 9] = Default::default();
    let mut coord_i = 0;

    for d_x in [-1,0,1] {
        let next_x = x + d_x;
        if next_x < 0 || next_x >= width as i32 {
            continue;
        }
        
        for d_y in [-1,0,1] {
            let next_y = y + d_y;
            if next_y < 0 || next_y >= height as i32 {
                continue;
            }

            let tap = next_x as usize + (next_y as usize * width);
            coords[coord_i] = Some(tap);
            coord_i += 1;
        }
    }
    
    coords
}