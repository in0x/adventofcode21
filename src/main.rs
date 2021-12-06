mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        print!("Not enough args provided!
                Usage: -- <path_to_input_files>");
        return;
    }

    let root_dir = Path::new(&args[1]);

    println!("Running day 1");
    day1::run(root_dir);
    println!("Running day 2");
    day2::run(root_dir);
    println!("Running day 3");
    day3::run(root_dir);
    println!("Running day 4");
    day4::run(root_dir);
    println!("Running day 5");
    day5::run(root_dir);
}
