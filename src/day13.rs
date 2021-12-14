use super::common;
use std::path::Path;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point2D {
    x: u32,
    y: u32
}

#[derive(PartialEq)]
enum Direction {
    X, Y
}

struct Fold {
    dir: Direction,
    val: u32
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day13_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let (points, folds) = {
        let mut points: Vec<Point2D> = Vec::new();

        let mut token_buf: Vec<u8> = Vec::new();
        token_buf.reserve(8);

        let mut cursor = 0;    
        loop {
            let x = match common::parse_num(&bytes, &mut token_buf, cursor) {
                (Some(val), new_cursor) => {
                    cursor = new_cursor;
                    val
                },
                (None, _) => panic!(),
            };

            cursor += 1;

            let y = match common::parse_num(&bytes, &mut token_buf, cursor) {
                (Some(val), new_cursor) => {
                    cursor = new_cursor;
                    val
                },
                (None, _) => panic!(),
            };

            points.push(Point2D {x, y});

            while !bytes[cursor].is_ascii_alphanumeric() {
                cursor += 1;
            }

            if bytes[cursor].is_ascii_alphabetic() {
                break;
            }
        }

        let mut folds: Vec<Fold> = Vec::new();

        loop {
            while (cursor < bytes.len()) &&
                  (bytes[cursor] as char != 'x') &&
                  (bytes[cursor] as char != 'y') {
                cursor += 1;
            }

            if cursor >= bytes.len() {
                break;
            }

            let dir = if bytes[cursor] as char == 'x' {
                Direction::X
            } else {
                Direction::Y
            };

            cursor += 2; // skip "x="

            let val = match common::parse_num(&bytes, &mut token_buf, cursor) {
                (Some(val), new_cursor) => {
                    cursor = new_cursor;
                    val
                },
                (None, _) => panic!(),
            };

            folds.push(Fold { dir, val });
        }

        (points, folds)
    };

    let mut folded_points: HashSet<Point2D> = HashSet::new();
    folded_points.reserve(points.len());
    for point in &points {
        folded_points.insert(*point);
    }

    for fold in &folds {
        let mut new_points: HashSet<Point2D> = HashSet::new();
        new_points.reserve(folded_points.len());

        if fold.dir == Direction::X {
            for mut point in folded_points {
                if point.x > fold.val {
                    point.x = fold.val - (point.x - fold.val);
                }
        
                new_points.insert(point);
            }
        } else {
            for mut point in folded_points {
                if point.y > fold.val {
                    point.y = fold.val - (point.y - fold.val);
                }
        
                new_points.insert(point);
            }
        }

        folded_points = new_points;
    }

    let mut width = u32::MIN;
    let mut height = u32::MIN;

    for point in &folded_points {
        width = u32::max(width, point.x);
        height = u32::max(height, point.y);
    }

    width += 1; // Bump from max
    height += 1;

    let mut grid = Vec::new();
    let grid_len = (width as usize) * (height as usize); 
    grid.resize(grid_len, false);

    for point in &folded_points {
        let idx = point.x + (point.y * width);
        grid[idx as usize] = true;
    }

    for i in 0..grid.len() {
        if (i % width as usize) == 0 {
            print!("\n");            
        }

        if grid[i] {
            print!("*");
        } else {
            print!(" ");
        }
    }

}