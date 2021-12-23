use std::{path::Path, collections::{BinaryHeap, HashMap}};

type Map = Vec<Vec<char>>;

fn is_solved(map: &Map) -> bool {
    let room_y_top: usize = 2;
    let room_y_bot: usize = map.len() - 2;

    for row in room_y_top..=room_y_bot {
        let solved = map[row][3] == 'A' &&
                     map[row][5] == 'B' &&
                     map[row][7] == 'C' &&
                     map[row][9] == 'D';

        if !solved { return false; }
    }
    true
}

#[cfg(test)]
mod SolveTests {
    use super::{parse_input, is_solved};

    #[test]
    fn test_solved() {
        let input = "#############\n#...........#\n###A#B#C#D###\n  #A#B#C#D#  \n  #########  ";
        let mut map = parse_input(&input);
        map.insert(4, "  #A#B#C#D#  ".chars().collect());
        map.insert(4, "  #A#B#C#D#  ".chars().collect());
        assert!(is_solved(&map));
    }   
    
    #[test]
    fn test_not_solved() {
        let input = "#############\n#...........#\n###B#C#B#D###\n  #A#D#C#A#  \n  #########  ";
        let mut map = parse_input(&input);
        map.insert(4, "  #D#C#B#A#  ".chars().collect());
        map.insert(4, "  #D#B#A#C#  ".chars().collect());
        assert!(!is_solved(&map));
    }
}

fn get_room_i_for_kind(kind: char) -> Option<usize> {
    match kind {
        'A' => Some(3),
        'B' => Some(5),
        'C' => Some(7),
        'D' => Some(9),
        _ => None
    }
}

fn get_cost_for_kind(kind: char) -> Option<u64> {
    match kind {
        'A' => Some(1),
        'B' => Some(10),
        'C' => Some(100),
        'D' => Some(1000),
        _ => None
    }
}

const HALL_LINE_IDX: usize = 1;
const ROOM_Y_TOP: usize = 2;

// #############
// #...........#
// ###B#C#B#D###
//   #A#D#C#A#
//   #########

fn get_next_steps(map: &Map) -> Vec<(u64, Map)>{
    let mut steps = Vec::new();
    let room_y_bot: usize = map.len() - 2;

    let hall = &map[HALL_LINE_IDX];

    // Try to find someone that can move into a room
    for space_i in 0..hall.len() {
        let room_x = match get_room_i_for_kind(hall[space_i]) {
            Some(i) => i,
            None => continue,
        };

        let (from_spc, to_spc) = if space_i > room_x {
            (room_x, space_i)
        } else {
            (space_i + 1, room_x + 1)
        };

        if (from_spc..to_spc).any(|c| hall[c] != '.') {
            continue;
        }

        let free_room = match(ROOM_Y_TOP..=room_y_bot)
            .take_while(|row| map[*row][room_x] == '.')
            .last() {
                Some(row) => row,
                None => continue, 
        };

        // If we want to fill up the room, but our room doesnt have
        // the same kind of dude in it, we cant move there.
        if (free_room != room_y_bot) && 
           (map[free_room + 1][room_x] != hall[space_i]) {
            continue;
        }

        let mut map_permut = map.clone();
        map_permut[free_room][room_x] = hall[space_i];
        map_permut[HALL_LINE_IDX][space_i] = '.';

        let dist = (to_spc - from_spc + free_room - 1) as u64; 
        let cost = get_cost_for_kind(hall[space_i]).unwrap();
        steps.push((dist * cost, map_permut));
    }

    let room_indices = [3, 5, 7, 9];

    // Try to find someone that can move out of a room
    for room_y in ROOM_Y_TOP..=room_y_bot {
        for room_x in room_indices {
            let cost = match get_cost_for_kind(map[room_y][room_x]) {
                Some(c) => c,
                None => continue,
            };
            if (ROOM_Y_TOP..room_y).any(|row| map[row][room_x] != '.' ) ||
              ((room_y + 1)..=room_y_bot).any(|row| map[row][room_x] == '.') {
                continue;
            }

            let hall_indices = [1, 2, 4, 6, 8, 10, 11];

            for hall_i in room_x..map[HALL_LINE_IDX].len() {
                if map[HALL_LINE_IDX][hall_i] != '.' {
                    break; // Someone's blocking the way.
                }
                if !hall_indices.contains(&hall_i) { 
                    continue; // We dont stop in front of rooms.
                }

                let mut map_permut = map.clone();
                map_permut[HALL_LINE_IDX][hall_i] = map[room_y][room_x];
                map_permut[room_y][room_x] = '.';

                let dist = (hall_i - room_x + (room_y - 1)) as u64;
                steps.push((dist * cost, map_permut));
            }

            for hall_i in (hall_indices[0]..=room_x).rev() {
                if map[HALL_LINE_IDX][hall_i] != '.' {
                    break; // Someone's blocking the way.
                }
                if !hall_indices.contains(&hall_i) { 
                    continue; // We dont stop in front of rooms.
                }

                let mut map_permut = map.clone();
                map_permut[HALL_LINE_IDX][hall_i] = map[room_y][room_x];
                map_permut[room_y][room_x] = '.';

                let dist = (room_x  - hall_i + (room_y - 1)) as u64;
                steps.push((dist * cost, map_permut));                
            }
        }
    }

    steps
}

#[derive(Clone, Default, Eq, PartialEq)]
struct Step {
    cost: u64,
    map: Map
}

impl Step {
    fn new(c: u64, m: Map) -> Step {
        Step { cost: c, map: m }
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Step) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost) // Flip the order of comparison so we 
        .then_with(|| self.map.cmp(&other.map)) // get a min-sorted heap.
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_min_cost_solve(_input_map: &Map) -> u64 {
    let mut queue = BinaryHeap::new();
    let mut visited: HashMap<Map, u64> = HashMap::new();

    queue.push(Step::new(0, _input_map.clone()));

    while let Some(cur_step) = queue.pop() {
        if is_solved(&cur_step.map) {
            return cur_step.cost;
        }

        match visited.get(&cur_step.map) {
            Some(&existing_cost) => if cur_step.cost > existing_cost {
                continue;
            },
            None => ()
        }

        for (next_step_cost, next_step_map) in get_next_steps(&cur_step.map) {
            let existing_cost = match visited.get(&next_step_map) {
                Some(c) => *c,
                None => u64::MAX 
            };

            let next_cost = cur_step.cost + next_step_cost;
            if existing_cost > next_cost {
                visited.insert(next_step_map.clone(), next_cost);
                queue.push(Step::new(next_cost, next_step_map));
            }
        }
    }

    panic!()
}

pub fn run(_: &Path) {
    // let input = "#############\n#...........#\n###B#C#B#D###\n  #A#D#C#A#  \n  #########  ";
    let input = "#############\n#...........#\n###B#C#A#D###\n  #B#C#D#A#  \n  #########  ";

    let mut map = input.lines().map(|l| l.chars().collect()).collect::<Vec<_>>();
    println!("{}", get_min_cost_solve(&map));

    map.insert(3, "  #D#C#B#A#  ".chars().collect());
    map.insert(3, "  #D#B#A#C#  ".chars().collect());
    println!("{}", get_min_cost_solve(&map));
}