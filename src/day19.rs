use super::common;
use std::fmt;
use std::hash::Hash;
use std::{path::Path, io::BufRead};
use std::collections::{HashSet, HashMap};

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

    fn abs(&self) -> Vec3 {
        Vec3 { x: self.x.abs(), y: self.y.abs(), z: self.z.abs() }
    }

    fn fmag(&self) -> f32 {
        (self.x as f32 * self.x as f32 
       + self.y as f32 * self.y as f32 
       + self.z as f32 * self.z as f32).sqrt()
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}

#[derive(Clone, Copy, Default)]
struct Similarity {
    delta_mag: f32, 
    d_min: i32, 
    d_max: i32 
}

#[derive(Clone, Default)]
struct Signal {
    pos: Vec3
}

#[derive(Clone, Default)]
struct Scanner {
    signals: Vec<Signal>,
    similar: Vec<HashMap<usize, Similarity>>, // todo make tis a big vec
    pos: Vec3
}

fn compare_signals(this_similar: &HashMap<usize, Similarity>, other_similar: &HashMap<usize, Similarity>,
    out_hits: &mut Vec<(Similarity, usize, usize)>) {
    out_hits.clear();

    for (this_idx, this_cmp) in this_similar {
        let found = other_similar.iter().find(|(_, other_cmp)| {
            common::f32_near_equal(this_cmp.delta_mag, other_cmp.delta_mag) &&
            (this_cmp.d_min == other_cmp.d_min) &&
            (this_cmp.d_max == other_cmp.d_max)
        });

        match found {
            Some(other_kvp) => out_hits.push((*other_kvp.1, *this_idx, *other_kvp.0)),
            None => (),
        }
    }
}

#[derive(Clone)]
struct Intersection {
    to: usize,
    from: usize,
    similarities: Vec<(Similarity, usize, usize)>
}

impl Scanner {
    fn update_similarity(&mut self, idx: usize, new_idx: usize) {
        let delta_abs = Vec3::sub(self.signals[idx].pos, self.signals[new_idx].pos).abs();
        
        let delta_mag = delta_abs.fmag();
        let d_min = i32::min(delta_abs.x, i32::min(delta_abs.y, delta_abs.z));
        let d_max = i32::max(delta_abs.x, i32::max(delta_abs.y, delta_abs.z));
        let similarity = Similarity{ delta_mag, d_min, d_max };

        self.similar[idx].insert(new_idx, similarity);
        self.similar[new_idx].insert(idx, similarity);
    }

    fn add(&mut self, pos: Vec3) {
        let to_update_idx = self.signals.len();
        self.signals.push(Signal { pos });
        self.similar.push(HashMap::new());

        for i in 0..to_update_idx {
            self.update_similarity(i, to_update_idx);
        }
    }

    fn compare(&self, other: &Scanner) -> Option<Intersection> {
        let mut hit_buf = Vec::new();

        for i in 0..other.signals.len() {
            for j in 0..self.signals.len() {
                compare_signals(&other.similar[i], &self.similar[j], &mut hit_buf);
                if hit_buf.len() >= 11 {
                    return Some(Intersection {
                        to: i,
                        from: j,
                        similarities: hit_buf
                    });
                }
            }
        }

        None
    }

    fn transform_points_from_intersect(probes: &mut[Scanner], this_idx: usize, other_idx: usize, intersect: &Intersection) {
        for hit in &intersect.similarities {
            if hit.0.d_min == 0 { continue; }

            let dt_0 = Vec3::sub(probes[this_idx].signals[intersect.from].pos, probes[this_idx].signals[hit.2].pos);
            let dt_1 = Vec3::sub(probes[other_idx].signals[intersect.to].pos, probes[other_idx].signals[hit.1].pos);

            if dt_0.x.abs() == dt_0.y.abs() || dt_0.z.abs() == dt_0.y.abs() || dt_0.x.abs() == dt_0.z.abs() {
                continue;
            }
            
            let axis_map = [
                if dt_0.x == dt_1.x { 1 } else if dt_0.x == -dt_1.x { -1 } else { 0 }, 
                if dt_0.y == dt_1.x { 1 } else if dt_0.y == -dt_1.x { -1 } else { 0 },
                if dt_0.z == dt_1.x { 1 } else if dt_0.z == -dt_1.x { -1 } else { 0 },
                if dt_0.x == dt_1.y { 1 } else if dt_0.x == -dt_1.y { -1 } else { 0 },
                if dt_0.y == dt_1.y { 1 } else if dt_0.y == -dt_1.y { -1 } else { 0 },
                if dt_0.z == dt_1.y { 1 } else if dt_0.z == -dt_1.y { -1 } else { 0 },
                if dt_0.x == dt_1.z { 1 } else if dt_0.x == -dt_1.z { -1 } else { 0 },
                if dt_0.y == dt_1.z { 1 } else if dt_0.y == -dt_1.z { -1 } else { 0 },
                if dt_0.z == dt_1.z { 1 } else if dt_0.z == -dt_1.z { -1 } else { 0 }
            ];

            for signal in &mut probes[other_idx].signals {
                let pre = signal.pos;
                signal.pos.x = pre.x * axis_map[0] + pre.y * axis_map[3] + pre.z * axis_map[6];
                signal.pos.y = pre.x * axis_map[1] + pre.y * axis_map[4] + pre.z * axis_map[7];
                signal.pos.z = pre.x * axis_map[2] + pre.y * axis_map[5] + pre.z * axis_map[8];
            }

            let from = probes[this_idx].signals[intersect.from].pos;
            let to = probes[other_idx].signals[intersect.to].pos;
            probes[other_idx].pos = Vec3::sub(from, to);

            let other = &mut probes[other_idx];

            for signal in &mut other.signals {
                signal.pos = Vec3::add(signal.pos, other.pos);
            }

            // println!("Scanner {} at {}", other_idx, other.pos);
            break;
        }
    }
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day19_input.txt");
    let file = std::fs::File::open(input_path).unwrap();
    let reader = std::io::BufReader::new(file);
    let mut probes: Vec<Scanner> = Vec::new();

    for res in reader.lines() {
        let line = res.unwrap();
        let c1 = line.chars().take(1).next();
        if c1.is_none() {
            continue;
        }
        let c2 = line.chars().skip(1).take(1).next().unwrap();
        if c2 == '-' {
            probes.push(Scanner::default());
        } else {
            let bytes = line.bytes().collect::<Vec<_>>();
            let values = common::read_list_of_csv_i32s(&bytes);
            assert!(values.len() == 3);
            
            let scanner = probes.last_mut().unwrap();
            scanner.add(Vec3::new(values[0], values[1], values[2]));
        }
    }

    let mut visited = vec![false; probes.len()];

    probes[0].pos = Vec3::zero(); // Assume the first probe as our "absolute" space
    visited[0] = true;

    while visited.iter().any(|v| !v) {
        for i in 0..probes.len() {
            for j in 0..probes.len() {
                if i == j || !visited[i] || visited[j] {
                    continue;
                }

                let intersect = probes[i].compare(&probes[j]);
                match intersect {
                    None => continue,
                    Some(mut intrs) => {
                        intrs.similarities.sort_by(|a, b| {
                            return a.1.cmp(&b.1).then_with(|| {
                                a.2.cmp(&b.2)
                            })
                        });

                        Scanner::transform_points_from_intersect(&mut probes, i, j, &intrs);
                        visited[j] = true;
                    }
                }

            }
        }
    }

    let mut all_signals = HashSet::new();
    for scanner in &probes {
        for signal in &scanner.signals {
            all_signals.insert(signal.pos);
        }
    }

    let mut max_dist = 0;
    for a in &probes {
        for b in &probes {
            let dist = (a.pos.x - b.pos.x).abs() 
                     + (a.pos.y - b.pos.y).abs()
                     + (a.pos.z - b.pos.z).abs();

            max_dist = max_dist.max(dist);
        }
    }

    println!("Num signals {}", all_signals.len());
    println!("Max dist {}", max_dist);
}