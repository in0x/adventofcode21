mod common;
mod day1;
mod day2;

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print!("Not enough args provided!
                Usage: -- <path_to_input_files>");
        return;
    }

    let root_dir = Path::new(&args[1]);

    println!("Running day 1.");
    day1::run(root_dir);
    println!("Running day 2.");
    day2::run(root_dir);
}
