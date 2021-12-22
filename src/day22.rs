use super::common;
use std::fmt;
use std::io::BufRead;
use std::path::Path;
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3 {x, y, z}
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, Default)]
struct Step {
    min: Vec3,
    max: Vec3,
    on: bool
}

pub fn run(root_dir: &Path) {
    let steps = {
        let mut steps = Vec::new();

        let input_path = root_dir.join("day22_input.txt");
        let file = std::fs::File::open(input_path).unwrap();
        let reader = std::io::BufReader::new(file);
        
        for res in reader.lines() {
            let line = res.unwrap();
            if line.is_empty() {
                continue;
            }

            let mut step = Step::default();
            step.on = line.contains("on"); 

            let num_iter = line.bytes().skip(if step.on {3} else {4});
            let bytes = num_iter.collect::<Vec<_>>();
            
            let mut token_buf = Vec::new();

            let mut cursor = 0;
            let mut values: [i32;6] = Default::default();
            for i in 0..values.len() {
                while !(bytes[cursor] as char).is_numeric() && ((bytes[cursor] as char) != '-') {
                    cursor += 1;
                }

                match common::parse_num(&bytes, &mut token_buf, cursor) {
                    (Some(x), new_cursor) => {
                        values[i] = x;
                        cursor = new_cursor;
                    },
                    (None, _) => panic!(),
                }
            }

            step.min = Vec3::new(values[0], values[2], values[4]);
            step.max = Vec3::new(values[1], values[3], values[5]);
            steps.push(step);
        }
        steps
    };

    // for step in &steps {
    //     println!("{} {} {}", if step.on {"on"} else {"off"}, step.lower_bound, step.upper_bound);
    // }

    let mut lookup: HashSet<Vec3> = HashSet::new();   

    for step in &steps {
        let mut min = step.min;
        let mut max = step.max;
        min.x = min.x.max(-50);
        min.y = min.y.max(-50);
        min.z = min.z.max(-50);
        max.x = max.x.min(50);
        max.y = max.y.min(50);
        max.z = max.z.min(50);

        for x in min.x..=max.x {
            for y in min.y..=max.y {
                for z in min.z..=max.z {
                    let pos = Vec3::new(x, y, z);

                    if step.on {
                        lookup.insert(pos);
                    } else {
                        lookup.remove(&pos);
                    }
                }
            }
        }
    }

    println!("num on {}", lookup.len());
}