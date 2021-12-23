use super::common;
use std::fmt;
use std::io::BufRead;
use std::path::Path;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Vec3 {
    v: [i64; 3]
}

impl Vec3 {
    fn new(x: i64, y: i64, z: i64) -> Vec3 {
        Vec3 { v: [x,y,z] }
    }
}

impl std::ops::Index<usize> for Vec3 {
    type Output = i64;
    fn index<'a>(&'a self, i: usize) -> &'a i64 {
        &self.v[i]
    }
}

impl std::ops::IndexMut<usize> for Vec3 {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut i64 {
        &mut self.v[i]
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.v[0], self.v[1], self.v[2])
    }
}

#[derive(Clone, Copy, Default)]
struct AABB {
    min: Vec3,
    max: Vec3,
}

#[derive(Clone, Copy, Default)]
struct Step {
    bb: AABB,
    on: bool
}

fn hmin(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 { v: [
        a[0].min(b[0]),
        a[1].min(b[1]),
        a[2].min(b[2]),
    ] }
}

fn hmax(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 { v: [
        a[0].max(b[0]),
        a[1].max(b[1]),
        a[2].max(b[2]),
    ] }
}

fn overlap(a: AABB, b: AABB) -> bool {
    for i in 0..3 {
        if a.max[i] < b.min[i] || a.min[i] > b.max[i] {
            return false;
        }
    }
    true
}

fn intersect(a: AABB, b: AABB) -> Option<AABB> {
    if !overlap(a, b) {
        return None;
    }

    Some(AABB {
        min: hmax(a.min, b.min),
        max: hmin(a.max, b.max)
    })
}

/// Cut b out of a 
fn cut(a: AABB, b: AABB) -> Vec<AABB> {
    let try_its = intersect(a, b);
    if try_its.is_none() {
        return vec![a];
    }

    let its = try_its.unwrap();
    let mut cubes = Vec::new();

    // top xz cut 
    if b.max[1] < a.max[1] {
        let min = Vec3::new(a.min[0], its.max[1] + 1, a.min[2]);
        let max = Vec3::new(a.max[0], a.max[1], a.max[2]);
        cubes.push(AABB {min, max});
    }

    // bottom xz cut
    if b.min[1] > a.min[1] {
        let min = Vec3::new(a.min[0], a.min[1], a.min[2]);
        let max = Vec3::new(a.max[0], its.min[1] - 1, a.max[2]);
        cubes.push(AABB {min, max});
    }

    // right yz cut
    if b.max[0] < a.max[0] {
        let min = Vec3::new(its.max[0] + 1, its.min[1], a.min[2]);
        let max = Vec3::new(a.max[0], its.max[1], a.max[2]);
        cubes.push(AABB {min, max});
    }

    // left yz cut
    if b.min[0] > a.min[0] {
        let min = Vec3::new(a.min[0], its.min[1], a.min[2]);
        let max = Vec3::new(its.min[0] - 1, its.max[1], a.max[2]);
        cubes.push(AABB {min, max});
    }

    // front xy cut
    if b.min[2] > a.min[2] {
        let min = Vec3::new(its.min[0], its.min[1], a.min[2]);
        let max = Vec3::new(its.max[0], its.max[1], its.min[2] - 1);
        cubes.push(AABB {min, max});
    }

    // back xy cut
    if b.max[2] < a.max[2] {
        let min = Vec3::new(its.min[0], its.min[1], its.max[2] + 1);
        let max = Vec3::new(its.max[0], its.max[1], a.max[2]);
        cubes.push(AABB {min, max});
    }

    cubes
}

#[cfg(test)]
mod Tests {
    use super::*;

    #[test]
    fn test_cut_out_of_mid_6_segs() {
        let from = AABB {
            min: Vec3::new(0,0,0),
            max: Vec3::new(5,5,5)
        };

        let out = AABB {
            min: Vec3::new(2,2,2),
            max: Vec3::new(3,3,3)
        };

        let cuts = cut(from, out);

        println!("#Cuts {}", cuts.len());
        for c in cuts {
            println!("min {} max {}", c.min, c.max);
        }
    }

    #[test]
    fn test_left_half() {
        let from = AABB {
            min: Vec3::new(0,0,0),
            max: Vec3::new(5,5,5)
        };

        let out = AABB {
            min: Vec3::new(0,0,0),
            max: Vec3::new(2,5,5)
        };

        let cuts = cut(from, out);

        println!("#Cuts {}", cuts.len());
        for c in cuts {
            println!("min {} max {}", c.min, c.max);
        }
    }

    #[test]
    fn test_right_half() {
        let from = AABB {
            min: Vec3::new(0,0,0),
            max: Vec3::new(5,5,5)
        };

        let out = AABB {
            min: Vec3::new(3,0,0),
            max: Vec3::new(5,5,5)
        };

        let cuts = cut(from, out);

        println!("#Cuts {}", cuts.len());
        for c in cuts {
            println!("min {} max {}", c.min, c.max);
        }
    }

    #[test]
    fn test_left_half_neg() {
        let from = AABB {
            min: Vec3::new(-5,-5,-5),
            max: Vec3::new(0,0,0)
        };

        let out = AABB {
            min: Vec3::new(-5,-5,-5),
            max: Vec3::new(-2,0,0)
        };

        let cuts = cut(from, out);

        println!("#Cuts {}", cuts.len());
        for c in cuts {
            println!("min {} max {}", c.min, c.max);
        }
    }

    fn score(vols: &[AABB]) -> i64 {
        let mut total_vol = 0;
        for vol in vols {
            total_vol += (vol.max[0] - vol.min[0] + 1).abs() *
                         (vol.max[1] - vol.min[1] + 1).abs() *
                         (vol.max[2] - vol.min[2] + 1).abs();
        }
        total_vol
    }

    fn apply(cut_out: AABB, existing: &[AABB], on: bool) -> Vec<AABB> {
        let mut new_vols = Vec::new();
        for vol in existing {
            let cuts = cut(*vol, cut_out);
            for c in cuts {
                new_vols.push(c);
            }
        }

        if on {
            new_vols.push(cut_out);
        }

        new_vols
    }

    #[test]
    fn test() {
        let mut vols = Vec::new();
        let on1 = AABB {
            min: Vec3::new(10,10,10),
            max: Vec3::new(12,12,12),
        };
        vols.push(on1);

        assert_eq!(27, score(&vols));

        let on2 = AABB {
            min: Vec3::new(11,11,11),
            max: Vec3::new(13,13,13),
        };

        vols = apply(on2, &vols, true);
        assert_eq!(46, score(&vols));

        let off = AABB {
            min: Vec3::new(9,9,9),
            max: Vec3::new(11,11,11)
        };

        vols = apply(off, &vols, false);
        assert_eq!(38, score(&vols));

        let on3 = AABB {
            min: Vec3::new(10,10,10),
            max: Vec3::new(10,10,10),
        };

        vols = apply(on3, &vols, true);
        assert_eq!(39, score(&vols));
    }
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

            let bb = AABB {
                min: Vec3{ v: [values[0] as i64, values[2] as i64, values[4] as i64] },
                max: Vec3{ v: [values[1] as i64, values[3] as i64, values[5] as i64] },
            };
            step.bb = bb;
            steps.push(step);
        }
        steps
    };

    let mut volumes: Vec<AABB> = Vec::new();

    for step in steps {
        let mut new_volumes = Vec::new();

        for vol in &volumes {
            let cuts = cut(*vol, step.bb);
            for bbs in cuts {
                new_volumes.push(bbs);
            }
        }

        if step.on {
            new_volumes.push(step.bb);
        }

        volumes = new_volumes;
    }

    let mut clamp_vol = 0;
    let mut total_vol = 0;

    fn hmin(v: Vec3, x: i64) -> Vec3 {
        Vec3::new(
            v[0].min(x),
            v[1].min(x),
            v[2].min(x),
        )
    }

    fn hmax(v: Vec3, x: i64) -> Vec3 {
        Vec3::new(
            v[0].max(x),
            v[1].max(x),
            v[2].max(x),
        )
    }

    for vol in &volumes {
        let min = hmax(vol.min, -50);
        let max = hmin(vol.max, 50);

        clamp_vol  += (max[0] - min[0] + 1).max(0) *
                      (max[1] - min[1] + 1).max(0) *
                      (max[2] - min[2] + 1).max(0);


        total_vol += (vol.max[0] - vol.min[0] + 1).abs() *
                     (vol.max[1] - vol.min[1] + 1).abs() *
                     (vol.max[2] - vol.min[2] + 1).abs();
    }

    println!("Clamp volume: {}", clamp_vol);
    println!("Total volume: {}", total_vol);
}