mod common;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

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
    println!("Running day 6");
    day6::run(root_dir);
    println!("Running day 7");
    day7::run(root_dir);
    println!("Running day 8");
    day8::run(root_dir);
    println!("Running day 9");
    day9::run(root_dir);
    println!("Running day 10");
    day10::run(root_dir);
    println!("Running day 11");
    day11::run(root_dir);
    println!("Running day 12");
    day12::run(root_dir);
    println!("Running day 13");
    day13::run(root_dir);
    println!("Running day 14");
    day14::run(root_dir);
    println!("Running day 15");
    day15::run(root_dir);
}
