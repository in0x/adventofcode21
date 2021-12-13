use super::common;
use std::path::Path;

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day9_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let (heightmap, map_width, map_height) = common::parse_grid(&bytes);

    let val_at = |x: i32, y: i32| -> u8 {
        let idx = x as usize + (y as usize * map_width);
        heightmap[idx]
    };

    let mut low_points = Vec::new();
    low_points.reserve(100);

    let tap_cmp = |x: i32, y: i32, cmp: u8, result_on_fail: bool| -> bool {
        if x < 0 || x >= map_width as i32 {
            return result_on_fail;
        } 
        if y < 0 || y >= map_height as i32 {
            return result_on_fail;
        }

        cmp < val_at(x, y)
    };

    let tap_cmp_lt = |x: i32, y: i32, cmp: u8| -> bool {
        tap_cmp(x, y, cmp, true)
    };

    let tap_cmp_gt = |x: i32, y: i32, cmp: u8| -> bool {
        tap_cmp(x, y, cmp, false)
    };

    for y in 0..(map_height as i32) {
        for x in 0..(map_width as i32) {
            let center = val_at(x, y);
            let mut is_low = true;
            is_low &= tap_cmp_lt(x - 1, y, center);
            is_low &= tap_cmp_lt(x + 1, y, center);
            is_low &= tap_cmp_lt(x, y - 1, center);
            is_low &= tap_cmp_lt(x, y + 1, center);

            if is_low {
                low_points.push((x, y));
            }
        }
    }

    let low_points_sum = low_points.iter()
        .map(|(x, y)| val_at(*x, *y) as u32).sum::<u32>();

    println!("Total risk score {}", low_points_sum + low_points.len() as u32);

    let mut largest_basins = Vec::new();
    largest_basins.reserve(low_points.len());

    for (low_x, low_y) in &low_points {
        let mut size = 0;
        let mut queue: Vec<(i32, i32)> = Vec::new();
        let mut closed: Vec<(i32, i32)> = Vec::new();
        queue.push((*low_x, *low_y));

        while queue.len() > 0 {
            let (x, y) = queue.pop().unwrap();

            if closed.iter().any(|c| (c.0 == x) && (c.1 == y)) {
                continue;
            }
            
            size += 1;
            closed.push((x, y));

            let point = val_at(x, y);

            let mut floodfill_add = |p_x: i32, p_y: i32, neighbor: u8| {
                if tap_cmp_gt(p_x, p_y, neighbor) && (val_at(p_x, p_y) != 9) {
                    queue.push((p_x, p_y));
                }
            };

            floodfill_add(x + 1, y, point);
            floodfill_add(x - 1, y, point);
            floodfill_add(x, y + 1, point);
            floodfill_add(x, y - 1, point);
        }

        largest_basins.push(size);
    }

    largest_basins.sort();

    let b0 = largest_basins.pop().unwrap();
    let b1 = largest_basins.pop().unwrap();
    let b2 = largest_basins.pop().unwrap();

    println!("Three largest: {}, {}, {}", b0, b1, b2);
    println!("Total basin size: {}", b0 * b1 * b2);
}