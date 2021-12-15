use super::common;
use std::path::Path;

#[allow(unused)]
fn print_grid(grid: &Vec<u8>, width: usize) {
    for i in 0..grid.len() {
        if (i % width) == 0 {
            print!("\n");
        }
        print!("{} ", grid[i]);
    }
}

pub fn run(root_dir: &Path) {
    let (mut grid, width, height) = {     
        let input_path = root_dir.join("day11_input.txt");
        let bytes = common::read_input_bytes(input_path.as_path());
        common::parse_grid(&bytes)
    };

    let mut num_flashes = 0;
    let mut step = 0;
    loop {
        if grid.iter().all(|x| *x == 0) {
            println!("Sync on step {}", step);
            break;
        }

        for val in &mut grid {
            *val += 1;
        }

        let mut this_step = Vec::new();
        let mut next_step = Vec::new();

        for i in 0..grid.len() {
            if grid[i] == 10 {
                this_step.push(i);
            }
        }

        while this_step.len() > 0 {
            for idx in &this_step {
                grid[*idx] = 0;
                num_flashes += 1;

                let taps = common::get_box_taps(*idx, width, height);
                for tap in taps {
                    match tap {
                        Some(i) => {
                            if grid[i] > 0 {
                                grid[i] += 1;
                                grid[i] = u8::min(grid[i], 11); // avoid overflow.

                                if grid[i] == 10 {
                                    next_step.push(i);
                                }
                            }    
                        }
                        None => (),
                    }
                }
            }

            this_step.clear();
            std::mem::swap(&mut this_step, &mut next_step);
        }

        step += 1;
    }

    println!("Num flashes {}", num_flashes);
}