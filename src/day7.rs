use super::common;
use std::path::Path;

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day7_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let mut positions = common::read_list_of_csv_u32s(&bytes);
    positions.sort();

    {
        let median_idx = positions.len() / 2;
        let median = positions[median_idx];
    
        let mut total_fuel = 0;
        for pos in &positions {
            let diff = (*pos as i32) - (median as i32);
            total_fuel += diff.abs() as u32;
        }

        println!("Total fuel non-weighted {}", total_fuel);
    }

    {
        let (min_pos, max_pos) = {
            let mut min = u32::MAX;
            let mut max = u32::MIN;
            for pos in &positions {
                min = u32::min(min, *pos);
                max = u32::max(max, *pos);
            }
            (min, max)
        };

        fn get_total_cost(positions: &Vec<u32>, midpoint: u32) -> u32 {
            let mut total_cost = 0;
            for pos in positions {
                let diff = ((*pos as f32) - (midpoint as f32)).abs();
                let cost_sum = diff * ((1.0 + diff) / 2.0);
                total_cost += cost_sum as u32;
            }

            total_cost
        }

        let mut pos_at_min_cost = 0;
        let mut min_total_cost = u32::MAX;

        for i in min_pos..=max_pos {
            let total_cost_at_i = get_total_cost(&positions, i);
            if total_cost_at_i < min_total_cost {
                min_total_cost = total_cost_at_i;
                pos_at_min_cost = i;
            }            
        }

        println!("Total weighted cost {}", min_total_cost);
    }

}