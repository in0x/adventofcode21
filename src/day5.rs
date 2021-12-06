use crate::common::parse_num;

use super::common;
use std::{path::Path, fmt};

#[derive(Default, Clone, Copy)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{}) -> ({},{})", self.x1, self.y1, self.x2, self.y2)
    }
}


fn is_numeric(ascii_byte: u8) -> bool {
    ('1'..='9').contains(&(ascii_byte as char))
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day5_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let mut lines: Vec<Line> = Vec::new();
    let mut cursor = 0;

    let mut token_buf = Vec::new();
    token_buf.reserve(32);

    let mut line_builder = Vec::new();
    line_builder.reserve(4);

    while cursor < bytes.len() {     
        let (parsed_num, new_cursor) = parse_num(&bytes, &mut token_buf, cursor);
        match parsed_num {
            Some(num) => {
                cursor = new_cursor;
                line_builder.push(num);
            },
            None => {
                panic!("Unexpectedly failed to parse number at pos {}", cursor)
            }
        }

        if line_builder.len() == 4 {
            lines.push(Line {
                x1: line_builder[0],
                y1: line_builder[1],
                x2: line_builder[2],
                y2: line_builder[3],
            });

            line_builder.clear();
        }

        while cursor < bytes.len() && (!is_numeric(bytes[cursor])) {
            cursor += 1;
        }
    }

    for line in &lines {
        println!("{}", line);
    }
    println!("Num lines {}", lines.len());

    // Parse all the lines
    // Throw away lines that dont lie at 90 degree angles
    // For each line determin if the intersect
    // Cache the intersection point, or if already cached increase the point
}