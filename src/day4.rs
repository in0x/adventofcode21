use super::common;
use std::fmt;
use std::path::Path;

const BOARD_WIDTH: usize = 5;

#[derive(Default)]
struct Board {
    rows: [[u32;BOARD_WIDTH];BOARD_WIDTH]
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows {
            let _ = writeln!(f, "{} {} {} {} {}", 
                             row[0],row[1],row[2],row[3],row[4]);
        }

        Ok(())
    }
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day4_input.txt");
    let input_bytes = common::read_input_bytes(input_path.as_path());

    let mut cursor = 0;
    let drawn_numbers = {
        let mut values: Vec<u32> = Vec::new();

        let mut cur_token = Vec::new();
        cur_token.reserve(32);
    
        loop {
            match input_bytes[cursor] as char {
                '0'..='9' => {
                    let digit_val = input_bytes[cursor] - ('0' as u8);
                    cur_token.push(digit_val);    
                },
                ',' | '\n' | '\r' | ' ' => {
                    let result = common::build_u32(&cur_token); 
                    values.push(result);
                    cur_token.clear();

                    if (input_bytes[cursor] as char) == '\n' ||
                       (input_bytes[cursor] as char) == '\r' {
                            break;
                    }
                },
                _ => {
                    panic!("Unexpected char \'{}\' at position {}", 
                           input_bytes[cursor] as char, cursor);
                },
            }

            cursor += 1;
        }

        values
    };


    // Scan forward until we hit the board data.
    while !('0'..':').contains(&(input_bytes[cursor] as char)) {
        cursor += 1;
    }

    let boards = {
        #[derive(Default)]
        struct BoardBuilder {
            values: [[u32;BOARD_WIDTH];BOARD_WIDTH],
            cur_idx: usize,
        }

        impl BoardBuilder {
            pub fn push(&mut self, value: u32) {
                let row = self.cur_idx / BOARD_WIDTH;
                let col = self.cur_idx - (BOARD_WIDTH * row);

                self.values[row][col] = value;
                self.cur_idx += 1;
            }

            pub fn finalize(&self) -> Board {
                let mut board = Board::default();
                board.rows.copy_from_slice(&self.values);

                board
            }

            pub fn is_complete(&self) -> bool {
                self.cur_idx == (BOARD_WIDTH * BOARD_WIDTH)
            }
        }

        let mut boards: Vec<Board> = Vec::new();
        let mut builder = BoardBuilder::default();

        let mut cur_token = Vec::new();
        cur_token.reserve(32);
        
        'outer: loop {
            'parse_num: loop {
                if cursor >= input_bytes.len() {
                    break 'parse_num; // We've reached EOF.
                } 

                match input_bytes[cursor] as char {
                    '0'..='9' => {
                        let digit_val = input_bytes[cursor] - ('0' as u8);
                        cur_token.push(digit_val);  
                        cursor += 1;
                    },
                    '\n' | '\r' | ' ' => {
                        break 'parse_num;
                    },
                    _ => {
                        panic!("Unexpected char \'{}\' at position {}", 
                               input_bytes[cursor] as char, cursor);
                    },  
                }
            }

            let result = common::build_u32(&cur_token); 
            builder.push(result);
            cur_token.clear();

            'skip_space: loop {
                if cursor >= input_bytes.len() {
                    break 'skip_space; // We've reached EOF.
                } 
                
                if ('0'..':').contains(&(input_bytes[cursor] as char)) {
                    break 'skip_space; // We've found the next number.
                }

                cursor += 1;
            }

            if builder.is_complete() {
                boards.push(builder.finalize());
                builder = BoardBuilder::default();
            }

            if cursor >= input_bytes.len() {
                break 'outer; // We've reached EOF.
            } 
        }

        boards
    };

    const INVALID_IDX : i32 = -1;

    struct BoardSolver {
        highest_col_idx: [i32; 5],
        highest_row_idx: [i32; 5],
        incomplete_rows_mask: u8,
        incomplete_cols_mask: u8,
    }

    impl BoardSolver {
        pub fn new() -> BoardSolver {
            BoardSolver {        
                highest_col_idx: [INVALID_IDX; 5],
                highest_row_idx: [INVALID_IDX; 5],
                incomplete_rows_mask: 0,
                incomplete_cols_mask: 0,
            }
        }
    }

    let no_solution_mask = 31; // 2^5 -1

    let mut earliest_solve_idxs = Vec::new();

    for board in &boards {
        let mut solver = BoardSolver::new();

        'board_inner: for row_i in 0..BOARD_WIDTH {
            let row = &board.rows[row_i];

            for col_i in 0..BOARD_WIDTH {
                match drawn_numbers.iter().
                position(|x| *x == row[col_i]) {
                    Some(idx) => {
                        let row_max = &mut solver.highest_row_idx[row_i];
                        *row_max = i32::max(*row_max, idx as i32);
                        
                        let col_max = &mut solver.highest_col_idx[col_i];
                        *col_max = i32::max(*col_max, idx as i32);
                    },
                    None => {
                        solver.incomplete_rows_mask |= 1 << row_i;
                        solver.incomplete_cols_mask |= 1 << col_i;

                        // Poison this slot so we cant complete it and skip over it later.
                        solver.highest_row_idx[row_i] = i32::MAX;
                        solver.highest_col_idx[col_i] = i32::MAX;

                        if solver.incomplete_rows_mask == no_solution_mask &&
                            solver.incomplete_cols_mask == no_solution_mask {
                            break 'board_inner;
                        }
                    }
                }
            }
        }
        
        let mut lowest_drawn_idx = i32::MAX;

        for i in 0..BOARD_WIDTH {
            lowest_drawn_idx = lowest_drawn_idx.min(solver.highest_row_idx[i]);
            lowest_drawn_idx = lowest_drawn_idx.min(solver.highest_col_idx[i]);
        }

        earliest_solve_idxs.push(lowest_drawn_idx);
    }

    let pick_earliest_win = false; 

    let final_solving_i = {
        if pick_earliest_win {
            let mut earliest_solve_i = 0;
            for i in 0..earliest_solve_idxs.len() {
                if earliest_solve_idxs[i] < earliest_solve_idxs[earliest_solve_i] {
                    earliest_solve_i = i;
                }
            }
            earliest_solve_i
        } else {
            let mut last_solve_i = 0;
            for i in 0..earliest_solve_idxs.len() {
                if earliest_solve_idxs[i] > earliest_solve_idxs[last_solve_i] {
                    last_solve_i = i;
                }
            }
            last_solve_i
        }
    };
    
    let solved_board = &boards[final_solving_i];
    let idx_of_final_drawn = earliest_solve_idxs[final_solving_i] as usize;
    let final_drawn_numbers = &drawn_numbers[..idx_of_final_drawn + 1];

    let mut score = 0;
    for row_i in 0..BOARD_WIDTH {
        for col_i in 0..BOARD_WIDTH {
            let cur_val = solved_board.rows[row_i][col_i];
            if !final_drawn_numbers.contains(&cur_val) {
                score += cur_val;
            } 
        }
    }

    let final_number_i = earliest_solve_idxs[final_solving_i] as usize;
    println!("Final score {}", score * drawn_numbers[final_number_i]);
}