use std::{path::Path, io::BufRead};

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day25_input.txt");
    let file = std::fs::File::open(input_path).unwrap();
    let reader = std::io::BufReader::new(file);

    let mut map = reader.lines()
        .map(|r| { 
            match r {
                Ok(l) => l.chars().collect(),
                _ => Vec::new() 
            }})
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let width = map[0].len();
    let height = map.len();

    let mut east_moves = Vec::new(); 
    let mut south_moves = Vec::new(); 
    let mut num_steps = 0;

    loop {
        num_steps += 1;
        let mut moves_this_step = 0;

        for y in 0..height {
            for x in 0..width {
                match map[y][x] {
                    '>' => {
                        let next_x = (x + 1) % width;
                        if map[y][next_x] == '.' {
                            east_moves.push(((x, y), (next_x, y)));
                        }
                    }, 
                    _ => (),
                }
            }
        }

        for mov in &east_moves {
            map[mov.0.1][mov.0.0] = '.';
            map[mov.1.1][mov.1.0] = '>';
        }
        moves_this_step += east_moves.len();
        east_moves.clear();

        for y in 0..height {
            for x in 0..width {
                match map[y][x] {
                    'v' => {
                        let next_y = (y + 1) % height;
                        if map[next_y][x] == '.' {
                            south_moves.push(((x, y), (x, next_y)));
                        }
                    }, 
                    _ => (),
                }
            }
        }

        for mov in &south_moves {
            map[mov.0.1][mov.0.0] = '.';
            map[mov.1.1][mov.1.0] = 'v';
        }
        moves_this_step += south_moves.len();
        south_moves.clear();

        if moves_this_step == 0 {
            break;
        }
    }

    println!("Reached rest after {} steps", num_steps);

    // East moving all check if they can move at the same time, only do if they can at that moment
    // Then south facing move, all check at the same time as well, but they see the moves the east
    // facing made before
}