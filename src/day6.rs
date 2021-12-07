use super::common;
use std::path::Path;

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day6_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let mut cursor = 0;
    let initial_values = {
        let mut values: Vec<u32> = Vec::new();

        let mut token_buf = Vec::new();
        token_buf.reserve(32);
    
        while cursor < bytes.len() {
            match common::parse_num(&bytes, &mut token_buf, cursor) {
                (Some(num), new_cursor) => {
                    values.push(num);
                    cursor = new_cursor;
                },
                (None, _) => {
                    panic!("We should have a number at each scan, but failed at cursor pos {}", cursor);
                } 
            }
            cursor += 1;
        }

        values
    };

    // Each slot represents a day remaining until duplication.
    // The slot stores how many fish there are with that lifetime remaining.
    let total_generations = 9;
    let first_generation = 0;
    let last_generation = 6;
    let new_generation = 8;

    let mut each_generation: Vec<u64> = Vec::new();
    each_generation.resize(total_generations, 0);

    for lifetime in &initial_values {
        each_generation[*lifetime as usize] += 1;
    }

    let num_days = 256;
    for _ in 0..num_days {
        
        let count_at_0 = each_generation[first_generation];
        each_generation[first_generation] = 0;

        for i in 1..total_generations {
            each_generation[i - 1] = each_generation[i];
        }

        each_generation[new_generation] = count_at_0;
        each_generation[last_generation] += count_at_0;
    }

    println!("Count at {} days: {}", num_days, each_generation.iter().sum::<u64>());
}