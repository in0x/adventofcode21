use super::common;
use std::path::Path;

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day7_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let mut positions = common::read_list_of_csv_u32s(&bytes);
    positions.sort();

    let median_idx = positions.len() / 2;
    let median = positions[median_idx];

    let mut total_fuel = 0;
    for pos in &positions {

        let diff = (*pos as i32) - (median as i32);
        total_fuel += diff.abs() as u32;
    }

    println!("Total fuel {}", total_fuel);
}