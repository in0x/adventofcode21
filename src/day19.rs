use super::common;
use core::num;
use std::fmt;
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

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, Default)]
struct Vec3f {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3f {
    fn new(x: f32, y: f32, z: f32) -> Vec3f {
        Vec3f {x, y, z}
    }

    fn scale(&self, scl: f32) -> Vec3f {
        Vec3f {x: self.x * scl, y: self.y * scl, z: self.z * scl}
    }
}

impl fmt::Display for Vec3f {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy,Default)]
struct Mtx3x3 {
    rows: [[f32;3];3]
}

impl Mtx3x3 {
    fn minor(&self, row0: usize, row1: usize, col0: usize, col1: usize) -> f32 {
        self.rows[row0][col0] * self.rows[row1][col1] - self.rows[row1][col0] * self.rows[row0][col1]
    }

    fn determinant(&self) -> f32 {
		self.rows[0][0] * self.minor(1, 2, 1, 2) -
        self.rows[0][1] * self.minor(1, 2, 0, 2) +
        self.rows[0][2] * self.minor(1, 2, 0, 1)
	}

    fn adjoint(&self) -> Mtx3x3 {
		let mut adj = Mtx3x3::default();

        adj.rows[0][0] = self.minor(1,2,1,2);  adj.rows[0][1] = -self.minor(0,2,1,2); adj.rows[0][2] = self.minor(0,1,1,2);
        adj.rows[1][0] = -self.minor(1,2,0,2); adj.rows[1][1] = self.minor(0,2,0,2);  adj.rows[1][2] = -self.minor(0,1,0,2);
        adj.rows[2][0] = self.minor(1,2,0,1);  adj.rows[2][1] = -self.minor(0,2,0,1); adj.rows[2][2] = self.minor(0,1,0,1);

        adj
	}

    fn inverse(&self) -> Mtx3x3 {
        let det = self.determinant();
        assert_ne!(det, 0.0);

        let mut inv = self.adjoint();
        inv.scale(det);
        inv
    }

    fn scale(&mut self, scalar: f32) {
        for row in &mut self.rows {
            for col in row {
                *col *= scalar;
            }
        }
    } 

    fn transform(&self, vec: Vec3f) -> Vec3f {
        Vec3f::new
        (self.rows[0][0] * vec.x + self.rows[0][1] * vec.y + self.rows[0][2] * vec.z,
         self.rows[1][0] * vec.x + self.rows[1][1] * vec.y + self.rows[1][2] * vec.z,
         self.rows[2][0] * vec.x + self.rows[2][1] * vec.y + self.rows[2][2] * vec.z)
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
    assert_eq!(distances.len(), ((len as f32 - 1.0) * (len as f32 / 2.0)) as usize);
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

    // TODO: maybe we can accumulate over different orientations?
    // Actually now, I think we need to flip into the same orientation to get the same distances
    // maybe try it just to see

    let mut unique_points: HashSet<Vec3> = HashSet::new();

    let identity_tf_idx = orientations.iter().position(|o| {
        o[0].0 == 0 && o[0].1 == 1 &&
        o[1].0 == 1 && o[1].1 == 1 &&
        o[2].0 == 2 && o[2].1 == 1
    }).unwrap();

    // Pair of translation and orientation to parent space
    let mut transform_to_parent: Vec<(Vec3, usize)> = Vec::new();
    let mut transforms_found = Vec::new();
    transforms_found.resize(probes.len(), false);
    transform_to_parent.resize(probes.len(), (Vec3::zero(), identity_tf_idx));

    transforms_found[0] = true;

    // while transforms_found.iter().any(|f| !f) {
    //     for prober_i_outer in 0..probes.len() {
    //         if !transforms_found[prober_i_outer] {
    //             continue;
    //         } 

    //         let outer_points = &probes[prober_i_outer];
    //         let outer_distances = calc_distances_vecs_for_points(outer_points);

    //         for prob_i_inner in 0..probes.len() {
    //             if prob_i_inner == prober_i_outer {
    //                 continue;
    //             }

    //             if transforms_found[prob_i_inner] {
    //                 continue;
    //             }

    //             'orient: for (tf_idx, tf) in orientations.iter().enumerate() {
    //                 let inner_points = reorient_points(&probes[prob_i_inner], tf);
    //                 let inner_distances = calc_distances_vecs_for_points(&inner_points);
    
    //                 let mut shared_points = HashSet::new();
    
    //                 for outer_el in &outer_distances {
    //                     match inner_distances.iter().find(|inner_el| outer_el.0 == inner_el.0) {
    //                         Some(inner_el) => {
    //                             shared_points.insert(inner_points[inner_el.1]);
    //                             shared_points.insert(inner_points[inner_el.2]);
    //                         },
    //                         None => ()
    //                     }
    //                 }
    
    //                 if shared_points.len() >= 12 {
    //                     for p in &shared_points {
    //                         // println!("Point: {} {} {}", p.x, p.y, p.z);
    //                         unique_points.insert(*p);   
    //                     }

    //                     // println!("Overlap with {} and {}", prober_i_outer, prob_i_inner);
    //                     // break 'orient;
    //                 }
    //             }
    //         }
    //     }
    // }

    for prober_i_outer in 0..probes.len() {
        let outer_points = &probes[prober_i_outer];
        let outer_distances = calc_distances_vecs_for_points(outer_points);

        // for prob_i_inner in (prober_i_outer + 1)..probes.len() {
        for probe_i_inner in 0..probes.len() {
            if probe_i_inner == prober_i_outer {
                continue;
            }

            'orient: for (tf_idx, tf) in orientations.iter().enumerate() {
                let inner_points = reorient_points(&probes[probe_i_inner], tf);
                let inner_distances = calc_distances_vecs_for_points(&inner_points);

                let mut shared_points = HashSet::new();

                for outer_el in &outer_distances {
                    match inner_distances.iter().find(|inner_el| outer_el.0 == inner_el.0) {
                        Some(inner_el) => {

                            // let translate =
                            // if Vec3::sub(outer_points[outer_el.1], inner_points[inner_el.1]) == 
                            //    Vec3::sub(outer_points[outer_el.2], inner_points[inner_el.2]) {

                            //     Vec3::sub(outer_points[outer_el.1], inner_points[inner_el.1])
                            // } else 
                            // if Vec3::sub(outer_points[outer_el.2], inner_points[inner_el.1]) == 
                            //    Vec3::sub(outer_points[outer_el.1], inner_points[inner_el.2]) {
                                
                            //     Vec3::sub(outer_points[outer_el.2], inner_points[inner_el.1])
                            // } else {
                            //     panic!();
                            // };

                            // let hh = Vec3::sub(inner_points[inner_el.1], translate);
                            // let pp = Vec3::sub(inner_points[inner_el.2], translate);
                            // println!("{} {} {}", hh.x, hh.y, hh.z);
                            // println!("{} {} {}", pp.x, pp.y, pp.z);

                            // println!("{} {} tf {} {} {}",prober_i_outer, prob_i_inner, translate.x, translate.y, translate.z);

                            // let rel_1 = Vec3::add(inner_points[inner_el.1], translate);
                            // let rel_2 = Vec3::add(inner_points[inner_el.2], translate);

                            // shared_points.insert(rel_1);
                            // shared_points.insert(rel_2);

                            shared_points.insert(inner_points[inner_el.1]);
                            shared_points.insert(inner_points[inner_el.2]);
                        },
                        None => ()
                    }
                }

                if shared_points.len() >= 12 {
                    
                    let axes = [
                        Vec3f::new(1.0, 0.0, 0.0),
                        Vec3f::new(0.0, 1.0, 0.0),
                        Vec3f::new(0.0, 0.0, 1.0),
                    ];
                    
                    let x_axis = {
                        axes[tf[0].0].scale(tf[0].1 as f32)
                    };

                    let y_axis = {
                        axes[tf[1].0].scale(tf[1].1 as f32)
                    };

                    // should this be cross?
                    let z_axis = {
                        axes[tf[2].0].scale(tf[2].1 as f32)
                    };

                    let mut basis_mtx = Mtx3x3::default();
                    basis_mtx.rows[0][0] = x_axis.x; 
                    basis_mtx.rows[1][0] = x_axis.y; 
                    basis_mtx.rows[2][0] = x_axis.z; 

                    basis_mtx.rows[0][1] = y_axis.x; 
                    basis_mtx.rows[1][1] = y_axis.y; 
                    basis_mtx.rows[2][1] = y_axis.z; 

                    basis_mtx.rows[0][2] = z_axis.x; 
                    basis_mtx.rows[1][2] = z_axis.y; 
                    basis_mtx.rows[2][2] = z_axis.z; 

                    basis_mtx = basis_mtx.inverse();

                    println!("Po {} Pi {} Tf [({},{})({},{})({},{})]", prober_i_outer, probe_i_inner, 
                        tf[0].0, tf[0].1, tf[1].0, tf[1].1, tf[2].0, tf[2].1);
                    
                    for p in &shared_points {
                        let vf = Vec3f::new(p.x as f32, p.y as f32, p.z as f32);

                        let vt = basis_mtx.transform(vf);

                        println!("{}", vt);

                        // println!("Point: {} {} {}", p.x, p.y, p.z);
                        // unique_points.insert(*p);   
                    }
                    panic!();
                    // break 'orient;
                }
            }
        }
    }


    let mut total = 0;
    for probe in &probes {
        for pos in probe {
            total += 1;
        }
    }

    println!("All points: {} Unique points: {}", total, unique_points.len());

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