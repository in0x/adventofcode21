use super::common;
use core::num;
use std::hash::Hash;
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

    fn add(lhs: Vec3, rhs: Vec3) -> Vec3 {
        Vec3::new(
            lhs.x + rhs.x,
            lhs.y + rhs.y,
            lhs.z + rhs.z
        )
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

// Each element is a triple of distance and the indicies of the two points involved.
fn calc_distances_vecs_for_points(points: &[Vec3]) -> Vec<(Vec3, usize, usize)> {
    let mut distances = Vec::new(); 
    let len = points.len();

    for i in 0..len {
        for j in (i+1)..len {
            let dst = Vec3::sub(points[i], points[j]);
            if dst.x > 0 { // flip the distances into a consistent
                distances.push((dst, i, j)); // direction for easier compares
            } else {
                distances.push((Vec3::scale(dst, -1), i, j));
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

    // for forward in 0..6 {
    //     let fwd_idx = forward % 3;
    //     let fwd_flip = if (forward % 2) == 0 { 1 } else { -1 };
        
    //     for up in 1..=2 {
    //         let up_idx = (fwd_idx + up) % 3;
    //         let r_idx = 3 - fwd_idx - up_idx;
                
    //         let mut up_set = Vec::new();
    //         let mut down_set = Vec::new();
    
    //         for location in &probes[1] {
    //             up_set.push(Vec3::new(
    //                 location.idx(fwd_idx) * fwd_flip,
    //                 location.idx(up_idx),
    //                 location.idx(r_idx)
    //             ));

    //             down_set.push(Vec3::new(
    //                 location.idx(fwd_idx) * fwd_flip,
    //                 -location.idx(up_idx), // Up negated
    //                 location.idx(r_idx)
    //             ));
 
    //         }
            
    //         permuts.push(up_set);
    //         permuts.push(down_set);
    //     }
    // }


    let orientations = {
        let mut orientations: Vec<[(usize, i32);3]> = Vec::new();
        
        for forward in 0..6 {
            let fwd_idx = forward % 3;
            let fwd_flip = if (forward % 2) == 0 { 1 } else { -1 };
                
            for up in 1..=2 {
                let up_idx = (fwd_idx + up) % 3;
                let r_idx = 3 - fwd_idx - up_idx;

                orientations.push([
                    (fwd_idx, fwd_flip),
                    (up_idx, 1),
                    (r_idx, 1)
                ]);

                orientations.push([
                    (fwd_idx, fwd_flip),
                    (up_idx, -1), // Up negated
                    (r_idx, 1)
                ]);
            }
        }
        orientations
    };

    fn reorient_points(points: &Vec<Vec3>, mapping: &[(usize, i32);3]) -> Vec<Vec3> {
        points.iter().map(|p| {
            Vec3::new(p.idx(mapping[0].0) * mapping[0].1,
                      p.idx(mapping[1].0) * mapping[1].1,
                      p.idx(mapping[2].0) * mapping[2].1)
        })
        .collect::<Vec<_>>()
    }

    // let base_distances = calc_distances_vecs_for_points(&probes[0]);

    // TODO: maybe we can accumulate over different orientations?
    // Actually now, I think we need to flip into the same orientation to get the same distances
    // maybe try it just to see

    // todo we can calculate axes permutations one at a time until we find an overlapping one

    let mut unique_points: HashSet<Vec3> = HashSet::new();

    for prober_i_outer in 0..probes.len() {
        let outer_points = &probes[prober_i_outer];
        let outer_distances = calc_distances_vecs_for_points(outer_points);

        for prob_i_inner in 0..probes.len() {
            if prob_i_inner == prober_i_outer {
                continue;
            }

            'orient: for tf in &orientations {
                let inner_points = reorient_points(&probes[prob_i_inner], tf);
                let inner_distances = calc_distances_vecs_for_points(&inner_points);

                let mut shared_points = HashSet::new();
                let mut shared_indices = Vec::new();

                let mut inner_shared = Vec::new();

                for outer_el in &outer_distances {
                    match inner_distances.iter().find(|inner_el| outer_el.0 == inner_el.0) {
                        Some(inner_el) => {
                            shared_points.insert(outer_points[outer_el.1]);
                            shared_points.insert(outer_points[outer_el.2]); // TODO only for testing
                            // if !(Vec3::sub(outer_points[outer_el.1], inner_points[inner_el.1]) ==
                            //      Vec3::sub(outer_points[outer_el.2], inner_points[inner_el.2])) {
                            //     if !(Vec3::sub(outer_points[outer_el.2], inner_points[inner_el.1]) ==
                            //          Vec3::sub(outer_points[outer_el.1], inner_points[inner_el.2])) {
                            //         panic!();
                            //     }    
                            // }
                            // if !inner_shared.contains(&inner_points[inner_el.1]) {
                                inner_shared.push(inner_points[inner_el.1]);
                            // }

                            // if inner_shared.contains(&inner_points[inner_el.2]){
                                inner_shared.push(inner_points[inner_el.2]);
                            // }

                            shared_indices.push((outer_el.1, inner_el.1, outer_el.2, inner_el.2));
                            shared_indices.push((outer_el.1, inner_el.1, outer_el.2, inner_el.2));
                        },
                        None => ()
                    }
                }

                if shared_points.len() >= 12 {
                    let mut local_filter = HashSet::new();

                    for i in 0..inner_shared.len() {
                        let el = &shared_indices[i];
                        let translate =
                        if Vec3::sub(outer_points[el.0], inner_points[el.1]) == Vec3::sub(outer_points[el.2], inner_points[el.3]) {
                            Vec3::sub(outer_points[el.0], inner_points[el.1])
                        } else if 
                        Vec3::sub(outer_points[el.2], inner_points[el.1]) == Vec3::sub(outer_points[el.0], inner_points[el.3]) {
                            Vec3::sub(outer_points[el.2], inner_points[el.1])
                        } else {
                            panic!();
                        };                    

                        let abs = Vec3::add(inner_shared[i], translate);
                        match local_filter.get(&abs) {
                            Some(_) => (),
                            None => {
                                local_filter.insert(abs);
                                println!("Abs point: {} {} {}", abs.x, abs.y, abs.z);
                            }
                        }
                        // println!("Abs point: {} {} {}", abs.x, abs.y, abs.z);
                    }

                    panic!();

                    // for p in &shared_points {
                    //     unique_points.insert(*p);
                    // }
                    // break 'orient;
                }
            }
        }
    }


    println!("Num points: {}", unique_points.len());

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