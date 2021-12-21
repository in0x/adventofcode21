use std::path::Path;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
struct GameState {
    turn_pos: u32,
    other_pos: u32,
    turn_score: u32,
    other_score: u32,
    p1_took_turn: bool,
    p2_took_turn: bool,
}

const MAX_SCORE: u32 = 21;

struct Cache {
    entries: HashMap<GameState, (u64, u64)>
}


const POSSIBLE_ROLLS: [u32;27] = [
    3,
    4,4,4,
    5,5,5, 5,5,5, 
    6,6,6, 6,6,6, 6, 
    7,7,7, 7,7,7, 
    8,8,8, 
    9,];

fn solve2(state: &GameState, cache: &mut Cache) -> (u64, u64) {
    // Slightly clunky but: We computed the "other score"
    // last turn. So we check here before running the next turn.
    if state.other_score >= MAX_SCORE {
        if state.p1_took_turn { return (1, 0) }
        if state.p2_took_turn { return (0, 1) }
    }

    if let Some(entry) = cache.entries.get(state) {
        return *entry;
    }

    let mut score = (0,0);

    for d1 in &POSSIBLE_ROLLS {
        let mut st = *state;

        st.turn_pos += d1;
        if st.turn_pos > 10 { st.turn_pos -= 10 }
        st.turn_score += st.turn_pos;

        // Players take alternating turns, at each which the universes split.
        // For slightly less branching, we switch the current turn values, rather
        // than computing the turn based on the turn flag.
        std::mem::swap(&mut st.turn_pos, &mut st.other_pos);
        std::mem::swap(&mut st.turn_score, &mut st.other_score);
        std::mem::swap(&mut st.p1_took_turn, &mut st.p2_took_turn);

        let (score1, score2) = solve2(&st, cache);
        score.0 += score1;
        score.1 += score2;
    }

    cache.entries.insert(*state, score);

    score
}

pub fn run(_: &Path) {    
    let mut cache = Cache { entries: HashMap::new() };

    let initial_state = GameState {
        turn_pos: 1,
        other_pos: 10,
        turn_score: 0,
        other_score: 0,
        p1_took_turn: false,
        p2_took_turn: true,
    };

    let (p1_wins, p2_wins) = solve2(&initial_state, &mut cache);

    println!("P1 wins {} P2 wins {}", p1_wins, p2_wins);
    println!("Max {}", p1_wins.max(p2_wins));
}