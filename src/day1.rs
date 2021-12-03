use std::fs;

pub fn run() {
    println!("Running day 1.");

    let input_path = "day1_input.txt";

    let input_bytes = fs::read(input_path)
        .expect(&format!("Failed to read input file {}", input_path));

    for x in &input_bytes {
        println!(" {}", x);
    }   

}