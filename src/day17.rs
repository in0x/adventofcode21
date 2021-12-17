use std::path::Path;

#[derive(Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    pub fn new(x: i32, y: i32) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn zero() -> Vec2 {
        Vec2 { x: 0, y: 0 }
    }

    pub fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y }
    }
}

pub fn run(_: &Path) {
    let (target_min, target_max) = {
        let target_x = Vec2::new(102, 157);
        let target_y = Vec2::new(-146, -90);
        // let target_x = Vec2::new(20, 30);
        // let target_y = Vec2::new(-10, -5);
    
        assert!(target_x.x > 0 && target_x.y > 0);

        (Vec2::new(i32::min(target_x.x, target_x.y), i32::min(target_y.x, target_y.y)),
         Vec2::new(i32::max(target_x.x, target_x.y), i32::max(target_y.x, target_y.y)))
    };

    println!("Target min ({} {}) max ({} {})", target_min.x, target_min.y,
        target_max.x, target_max.y);

    let start_x = {
        let mut x = 0;
        for i in 1..target_min.x {
            let sum = i as f32 * ((1.0 + i as f32) / 2.0);
            x = i;
            if sum >= target_min.x as f32 {
                break;
            }
        }
        x
    };

    let mut max_y = i32::MIN;
    let mut max_vel = None;

    let l_y = target_min.y;
    let u_y = target_min.y.abs();

    for s_y in l_y..u_y {
        let initial_vel = Vec2::new(start_x, s_y);
        let mut hit_vel = None;

        let mut vel = initial_vel;
        let mut pos = Vec2::zero();
        let mut peak = i32::MIN;

        'inner: loop {
            pos = pos.add(vel);
            peak = i32::max(peak, pos.y);

            if (pos.x >= target_min.x) && (pos.x <= target_max.x) {
                if (pos.y >= target_min.y) && (pos.y <= target_max.y) {
                    hit_vel = Some(initial_vel);
                    break 'inner;
                }
            }
    
            if target_max.x >= 0 {
                if pos.x > target_max.x {
                    break 'inner;
                }
            } else {
                if pos.x < target_max.x {
                    break 'inner;
                }
            }
    
            if vel.x == 0 {
                if target_max.x >= 0 {
                    if pos.x < target_min.x {
                        break 'inner;
                    }
                } else {
                    if pos.x > target_min.x {
                        break 'inner;
                    }
                }
    
                if pos.y < target_max.y {
                    break 'inner;
                }
            }
    
            vel.y -= 1;
            if vel.x > 0 {
                vel.x = i32::max(vel.x - 1, 0);
            } else {
                vel.x = i32::min(vel.x + 1, 0);
            }
        }

        match hit_vel {
            Some(_) => {
                if peak > max_y {
                    max_y = peak;
                    max_vel = hit_vel;
                }
            },
            None => ()
        }
    } 

    println!("Max y {} at vel {} {}", max_y, max_vel.unwrap().x, max_vel.unwrap().y);
}