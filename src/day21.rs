use super::common;
use core::num;
use std::path::Path;

pub fn run(root_dir: &Path) {
    // Player 1 starting position: 1
    // Player 2 starting position: 10

    let mut p1_pos: u32 = 1;
    let mut p2_pos: u32 = 10;

    let mut p1_score: u32 = 0;
    let mut p2_score: u32 = 0;

    let mut num_rolls: u32 = 0;
    loop {
        fn step(mut rolls: u32, mut pos: u32) -> (u32, u32) { // (new_rolls, new_pos)
            rolls += 1;
            let mut move_by = rolls; 
            rolls += 1;
            move_by += rolls;
            rolls += 1;
            move_by += rolls;

            if move_by >= 10 {
                move_by = move_by - ((move_by / 10) * 10);
            }

            pos += move_by;
            if pos > 10 {
                pos -= 10;
            }

            (rolls, pos)
        }
        {
            let (new_rolls, new_pos) = step(num_rolls, p1_pos);
            num_rolls = new_rolls;
            p1_pos = new_pos;
            p1_score += p1_pos;
        }
        if p1_score >= 1000 {
            break;
        }
        {
            let (new_rolls, new_pos) = step(num_rolls, p2_pos);
            num_rolls = new_rolls;
            p2_pos = new_pos;
            p2_score += p2_pos;
        }
        if p2_score >= 1000 {
            break;
        }
    }

    println!("Scores: p1 = {} p2 = {}", p1_score, p2_score);

    let min_score = p1_score.min(p2_score);
    println!("{} * {} = {}", min_score, num_rolls, min_score * num_rolls);
}