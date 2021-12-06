use crate::common::parse_num;

use super::common;
use std::{path::Path, fmt};

#[derive(Default, Clone, Copy)]
struct LineSegment {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl fmt::Display for LineSegment {
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

    let mut lines: Vec<LineSegment> = Vec::new();
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
            lines.push(LineSegment {
                x1: line_builder[0] as i32,
                y1: line_builder[1] as i32,
                x2: line_builder[2] as i32,
                y2: line_builder[3] as i32,
            });

            line_builder.clear();
        }

        while cursor < bytes.len() && (!is_numeric(bytes[cursor])) {
            cursor += 1;
        }
    }

    lines = lines.into_iter().filter(|line| {
                (line.x1 == line.x2) || (line.y1 == line.y2)
        }).collect();

    let mut max_x = 0;
    let mut max_y = 0;

    for i in 0..lines.len() {
        max_x = i32::max(max_x, i32::max(lines[i].x1, lines[i].x2));
        max_y = i32::max(max_y, i32::max(lines[i].y1, lines[i].y2));
    }

    let mut grid: Vec<u16> = Vec::new();
    grid.resize(((max_x + 1) * (max_y + 1)) as usize, 0);

    let mut inc_grid_cell = |x: i32, y: i32| {
        let idx = x + (max_x * y);
        grid[idx as usize] += 1;
    };

    for line in &lines {
        if line.x1 == line.x2 {
            let min = i32::min(line.y1, line.y2);
            let max = i32::max(line.y1, line.y2);

            for i in min..=max {
                inc_grid_cell(line.x1, i);
            }
        } else {
            let min = i32::min(line.x1, line.x2);
            let max = i32::max(line.x1, line.x2);

            for i in min..=max {
                inc_grid_cell(i, line.y1);
            }
        }
    }

    let mut num_intersections = 0;
    for cell in &grid {
        if *cell > 1 {
            num_intersections += 1;
        }
    }

    println!("Number of intersections {}", num_intersections);
}