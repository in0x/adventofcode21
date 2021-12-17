use super::common;
use std::path::Path;

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day17_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());
    println!("Day 17 placeholder");
}