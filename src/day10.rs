use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn run(root_dir: &Path) {
    let lines = {
        let input_path = root_dir.join("day10_input.txt");
        let file = fs::File::open(input_path).unwrap();
        let reader = io::BufReader::new(file);
        reader.lines()
            .map(|res| res.unwrap())
            .collect::<Vec<_>>()
    };
    let incomplete_lines = {
        let mut open_scope_lut: HashMap<char, char> = HashMap::new();
        open_scope_lut.insert(')', '(');
        open_scope_lut.insert(']', '[');
        open_scope_lut.insert('}', '{');
        open_scope_lut.insert('>', '<');
    
        let mut error_lut: HashMap<char, u32> = HashMap::new();
        error_lut.insert(')', 3);
        error_lut.insert(']', 57);
        error_lut.insert('}', 1197);
        error_lut.insert('>', 25137);
        
        let mut total_error = 0;
        let mut scope_stack = Vec::new();
    
        let mut check_line_for_err = |line: &str| -> Option<u32> {
            scope_stack.reserve(line.len());
    
            for c in line.chars() {
                match c {
                    '(' | '[' | '{' | '<' => scope_stack.push(c),
                    ')' | ']' | '}' | '>' => {
                        let top_scope = scope_stack.pop().unwrap();
                        if *open_scope_lut.get(&c).unwrap() != top_scope {
                            return Some(*error_lut.get(&c).unwrap())
                        } 
                    }
                    _ => panic!("Unexpected char {}", c),
                }
            }
            None
        };

        let remaining_lines = lines.iter().filter(|line| {
            match check_line_for_err(&line) {
                Some(err) => {
                    total_error += err;
                    false
                },
                None => true
            }
        }).collect::<Vec<&String>>();

        println!("Total error score {}", total_error);
        remaining_lines
    };
    
    let mut completion_scores = Vec::new();
    let mut scope_stack = Vec::new();

    let mut complete_lut: HashMap<char, u64> = HashMap::new();
    complete_lut.insert('(', 1);
    complete_lut.insert('[', 2);
    complete_lut.insert('{', 3);
    complete_lut.insert('<', 4);

    for line in incomplete_lines {
        let mut score: u64 = 0;
        scope_stack.reserve(line.len());

        for c in line.chars() {
            match c {
                '(' | '[' | '{' | '<' => scope_stack.push(c),
                ')' | ']' | '}' | '>' => { scope_stack.pop(); },
                _ => panic!("Unexpected char {}", c),
            }
        }

        for c  in scope_stack.iter().rev() {
            score *= 5;
            score += complete_lut.get(&c).unwrap();
        }

        scope_stack.clear();
        completion_scores.push(score);
    }

    completion_scores.sort();
    println!("Middle score {}", completion_scores[completion_scores.len() / 2]);
}