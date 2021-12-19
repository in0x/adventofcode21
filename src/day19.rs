use super::common;
use core::num;
use std::{path::Path, io::BufRead};
use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Vec3 {
        Vec3 {x, y, z}
    }

    fn zero() -> Vec3 {
        Vec3 {x: 0, y: 0, z: 0}
    }

    fn sub(lhs: Vec3, rhs: Vec3) -> Vec3 {
        Vec3::new(
            lhs.x - rhs.x,
            lhs.y - rhs.y,
            lhs.z - rhs.z
        )
    }

    fn mul(lhs: Vec3, rhs: Vec3) -> Vec3 {
        Vec3::new(
            lhs.x * rhs.x,
            lhs.y * rhs.y,
            lhs.z * rhs.z
        )
    }

    fn scale(lhs: Vec3, scl: i32) -> Vec3 {
        Vec3::new(
            lhs.x * scl,
            lhs.y * scl,
            lhs.z * scl
        )
    }

    fn idx(&self, i: usize) -> i32 {
        match i {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!()
        }
    }

    fn set(&mut self, idx: usize, value: i32) {
        match idx {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            _ => panic!()
        }
    }
}

fn calc_distances_vecs_for_points(points: &[Vec3]) -> Vec<Vec3> {
    let mut distances = Vec::new(); 
    let len = points.len();

    for i in 0..len {
        for j in (i+1)..len {
            let dst = Vec3::sub(points[i], points[j]);
            if dst.x > 0 { // flip the distances into a consistent
                distances.push(dst); // direction for easier compares
            } else {
                distances.push(Vec3::scale(dst, -1));
            }
        }
    }
    distances
}

pub fn run(root_dir: &Path) {
    let probes = 
    {
        let input_path = root_dir.join("day19_input.txt");
        let file = std::fs::File::open(input_path).unwrap();
        let reader = std::io::BufReader::new(file);
        let mut probes: Vec<Vec<Vec3>> = Vec::new();;

        for res in reader.lines() {
            let line = res.unwrap();
            let c1 = line.chars().take(1).next();
            if c1.is_none() {
                continue;
            }
            let c2 = line.chars().skip(1).take(1).next().unwrap();
            if c2 == '-' {
                probes.push(Vec::new());
            } else {
                let bytes = line.bytes().collect::<Vec<_>>();
                let values = common::read_list_of_csv_i32s(&bytes);
                assert!(values.len() == 3);
                probes.last_mut().unwrap().push(Vec3::new(values[0], values[1], values[2]));
            }
        }

        probes
    };

    let mut permuts: Vec<Vec<Vec3>> = Vec::new(); 

    for forward in 0..6 {
        let fwd_idx = forward % 3;
        let fwd_flip = if (forward % 2) == 0 { 1 } else { -1 };
        
        for up in 1..=2 {
            let up_idx = (fwd_idx + up) % 3;
            let r_idx = 3 - fwd_idx - up_idx;
                
            let mut up_set = Vec::new();
            let mut down_set = Vec::new();
    
            for location in &probes[1] {
                up_set.push(Vec3::new(
                    location.idx(fwd_idx) * fwd_flip,
                    location.idx(up_idx),
                    location.idx(r_idx)
                ));

                down_set.push(Vec3::new(
                    location.idx(fwd_idx) * fwd_flip,
                    -location.idx(up_idx), // Up negated
                    location.idx(r_idx)
                ));
 
            }
            
            permuts.push(up_set);
            permuts.push(down_set);
        }
    }

    let base_distances = calc_distances_vecs_for_points(&probes[0]);

    for (idx, permutation) in permuts.iter().enumerate() {
        let permut_distances = calc_distances_vecs_for_points(permutation);
        let mut shared_points: HashSet<Vec3> = HashSet::new();
        let mut num_hits = 0;

        'inner: for (dst_idx, dst) in base_distances.iter().enumerate() {
            if permut_distances.contains(dst) {
                {
                    let mut pi = 0;
                    'search: for i in 0..probes[0].len() {
                        for j in (i+1)..probes[0].len() {
                            if pi == dst_idx {
                                shared_points.insert(probes[0][i]);
                                shared_points.insert(probes[0][j]);
                                // println!("Shared Points: ({}, {}, {}) -> ({}, {}, {})",
                                //     probes[0][i].x, probes[0][i].y, probes[0][i].z, 
                                //     probes[0][j].y, probes[0][j].y, probes[0][j].z);
                                break 'search;
                            }
                            pi += 1;
                        }
                    }
                }

                num_hits += 1;
            }
        }

        if shared_points.len() >= 12 {
            println!("These intersect at axis type {} with points!", idx);
            for point in &shared_points {
                println!("({}, {}, {})", point.x, point.y, point.z);
            }
            // break 'inner;
        }

        // 'inner: for dst in &permut_distances {
        //     if base_distances.contains(dst) {
        //         num_hits += 1;
        //     }

        //     if num_hits >= 12 {
        //         println!("These intersect at axis type {}!", idx);
        //         break 'inner;
        //     }
        // }
    }

    // Generate distance from between each point in set
    // rotate in all directions, check if we can find 12 of the same
    // at each rotation
    // Sensor 0 is our "origin", although I could use each sensor as a source of truth
    // to try all the other ones agains


    // For example, if a scanner is at x,y,z coordinates 500,0,-500 and there are beacons at 
    // -500,1000,-1500 and 1501,0,-500, the scanner could report that the first beacon is at 
    // -1000,1000,-1000 (relative to the scanner) but would not detect the second beacon at all.

    // ^^ This is because its not going off of euclidian distance, but single axis distance.
    // B1 has a 1000 unit distance in x (500 -> -500 = 1000), but B2 has a 1001 unit distance
    // (500 -> 1501 = 1001) so it falls outside of the detection range.
}

     // if (forward % 2) == 0 {
                //     // fwd_idx *= -1;
                //     print!("{} ", fwd_idx);
                // } else {
                //     print!("-{} ", fwd_idx);
                // }

                // print!("{} ", up_idx);
                // print!(" {}\n", r_idx);

                // if (forward % 2) == 0 {
                //     // fwd_idx *= -1;
                //     print!("{} ", fwd_idx);
                // } else {
                //     print!("-{} ", fwd_idx);
                // }

                // print!("-{} ", up_idx);
                // print!(" {}\n", r_idx);

                // let permut = Vec3::new(
                //     location.idx(),
                //     0,
                //     0
                // );
            // }