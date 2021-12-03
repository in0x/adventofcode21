use std::fs;
use std::path::Path;

pub fn run(root_dir:  &Path) {
    println!("Running day 1.");

    let input_path = root_dir.join("day1_input.txt");

    let input_bytes = fs::read(&input_path)
        .expect(&format!("Failed to read input file {:?}", input_path.to_str()));

    for x in &input_bytes {
        println!(" {}", x);
    }   
}