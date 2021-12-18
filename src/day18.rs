use super::common;
use std::fmt;
use std::path::Path;
use std::collections::VecDeque;

#[derive(Clone, Copy, PartialEq)]
enum Digit {
    ScopeOpen,
    ScopeClose,
    Literal(u32)
}

impl fmt::Display for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Digit::ScopeOpen => write!(f, "{}", '['),
            Digit::ScopeClose => write!(f, "{}", ']'),
            Digit::Literal(x) => write!(f, "{}", x),
        }
    }
}

impl fmt::Debug for Digit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[derive(Clone, Copy, PartialEq)]
enum ParseResult {
    Value(Digit),
    Comma,
    Newline,
}

fn char_to_digit(c: char) -> ParseResult {
    match c {
        '[' => ParseResult::Value(Digit::ScopeOpen),
        ']' => ParseResult::Value(Digit::ScopeClose),
        '0'..='9' => {
            let val = (c as u8) - ('0' as u8);
            ParseResult::Value(Digit::Literal(val as u32))
        },
        ',' => ParseResult::Comma,
        '\n' | '\r' => ParseResult::Newline,    
        t => panic!("Unexpected token \'{}\'", t),
    }
}

fn build_literal(digits: &mut Vec<u32>) -> u32 {
    digits.reverse();

    let mut literal = 0;
    for i in 0..digits.len() {
        literal += digits[i] * 10u32.pow(i as u32);
    }
    digits.clear();

    literal
}

fn number_from_byte_slice(bytes: &[u8]) -> VecDeque<Digit> {
    let mut number: VecDeque<Digit> = VecDeque::new();
    let mut literal_buf = Vec::new();
    for b in bytes {
        match char_to_digit(*b as char) {
            ParseResult::Value(value) => {
                match value {
                    Digit::ScopeClose => {
                        if !literal_buf.is_empty() {
                            let literal = build_literal(&mut literal_buf);
                            number.push_back(Digit::Literal(literal));
                        }
                        
                        number.push_back(value)
                    },
                    Digit::ScopeOpen => number.push_back(value),
                    Digit::Literal(x) => literal_buf.push(x), 
                }
            },
            ParseResult::Comma => {
                if !literal_buf.is_empty() {
                    let literal = build_literal(&mut literal_buf);
                    number.push_back(Digit::Literal(literal));
                }
            },
            ParseResult::Newline => (), // TODO handle on full input
        }
    }
    number
}

fn number_from_str_slice(str: &str) -> VecDeque<Digit> {
    let bytes = str.chars().map(|c| c as u8).collect::<Vec<_>>();
    number_from_byte_slice(&bytes)
}

fn explode_at(number: &mut VecDeque<Digit>, scope_open_idx: usize) -> VecDeque<Digit> {
    let mut exploded = VecDeque::new();
    exploded.reserve(number.len());

    let mut explode_to_left_idx = None;

    for _ in 0..scope_open_idx { // Only go up to the scope, we dont want to push the exploded scope.
        match number.pop_front() {
            Some(digit) => {
                match digit {
                    // Digit::Literal(_) => if explode_to_left_idx.is_none() {
                    //     explode_to_left_idx = Some(exploded.len());
                    // },
                    Digit::Literal(_) => explode_to_left_idx = Some(exploded.len()),
                    _ => ()
                }
                exploded.push_back(digit);
            },
            None => panic!(),
        }
    }

    exploded.push_back(Digit::Literal(0)); // Replace the exploded number with a 0.

    number.pop_front(); // Get rid of the exploded opening scope.

    let (exploded_left, exploded_right) = match (number.pop_front(), number.pop_front()) {
        (Some(l_digit), Some(r_digit)) => {
            match (l_digit, r_digit) {
                (Digit::Literal(l), Digit::Literal(r)) => {
                    (l, r)
                },
                _ => panic!(),
            }
        },
        _ => panic!(), // The two elements following the exploded scope must be literals.
    };

    number.pop_front(); // Get rid of the exploded closing scope.

    let mut explode_to_right_idx = None;

    while !number.is_empty() {
        match number.pop_front() {
            Some(digit) => {
                match digit {
                    Digit::Literal(_) => if explode_to_right_idx.is_none() {
                        explode_to_right_idx = Some(exploded.len());
                    },
                    // Digit::Literal(_) => explode_to_right_idx = Some(exploded.len()),
                    _ => ()
                }
                exploded.push_back(digit);
            },
            None => panic!(),
        }
    }

    fn explode_into(number: &mut VecDeque<Digit>, idx: Option<usize>, value: u32) {
        match idx {
            Some(i) => {
                match &mut number[i] {
                    Digit::Literal(x) => {
                        *x += value;
                    },
                    _ => panic!()
                }
            },
            _ => (),
        }
    }

    explode_into(&mut exploded, explode_to_left_idx, exploded_left);
    explode_into(&mut exploded, explode_to_right_idx, exploded_right);

    exploded
}

#[inline(never)]
fn split_at(number: &mut VecDeque<Digit>, split_digit_idx: usize) -> VecDeque<Digit> {
    let mut split = VecDeque::new();
    split.reserve(number.len());

    for _ in 0..split_digit_idx { // Only go up to before the digit
        match number.pop_front() {
            Some(digit) => {
                split.push_back(digit);
            },
            None => panic!(),
        }
    }

    let split_literal = match number.pop_front() {
        Some(digit) => {
            match digit {
                Digit::Literal(x) => x,
                _ => panic!(),
            }
        },
        None => panic!()
    };

    let left_val = split_literal / 2;
    let right_val = ((split_literal as f32) / 2.0).round() as u32;

    split.push_back(Digit::ScopeOpen);
    split.push_back(Digit::Literal(left_val));
    split.push_back(Digit::Literal(right_val));
    split.push_back(Digit::ScopeClose);

    while !number.is_empty() {
        split.push_back(number.pop_front().unwrap());
    }

    split
}

#[derive(Clone, Copy, PartialEq)]
enum Action {
    Explode(usize),
    Split(usize),
    None,
}

fn find_next_action(number: &VecDeque<Digit>) -> Option<Action> {
    let mut scopes = 0;
    
    for i in 0..number.len() {
        let next_action = match number[i] {
            Digit::ScopeClose => {
                scopes -= 1;
                assert!(scopes >= 0);
                Action::None   
            },
            Digit::ScopeOpen => {
                scopes += 1;
                if scopes > 4 {
                    Action::Explode(i)
                } else {
                    Action::None
                }
            },
            Digit::Literal(x) => if x > 9 {
                Action::Split(i)
            } else {
                Action::None
            },
        };

        if next_action != Action::None {
            return Some(next_action);
        }
    }

    None
}

fn reduce(mut number: VecDeque<Digit>) -> VecDeque<Digit> {
    let mut next_action = find_next_action(&number);

    while next_action.is_some() {
        match next_action {
            Some(Action::Explode(idx)) => {
                // println!("Explode at {}", idx);
                number = explode_at(&mut number, idx);
            },
            Some(Action::Split(idx)) => {
                // println!("Split at {}", idx);
                number = split_at(&mut number, idx);
            },
            _ => (),
            // _ => println!("Finished"),
        }

        next_action = find_next_action(&number);

        println!("Step:");
        print_number(&number);
    }

    number
}


fn print_number(number: &VecDeque<Digit>) {
    for d in number {
        print!("{} ", d);
    }
    println!();
}

// TODO: remove Action::None, just use Option
pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day18_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());
    
    // splitting happens in FILO order (splits may produce new actions
    // which get added to the top of the stack).
    
    let mut  number = number_from_byte_slice(&bytes);

    print_number(&number);

    number = reduce(number);
    
    print_number(&number);
}

#[cfg(test)]
mod tests {
    use crate::day18::print_number;

    fn check(from_str: &str, to_str: &str) {
        let from = super::number_from_str_slice(from_str);
        let to = super::number_from_str_slice(to_str);

        print!("Test:\nFr: ");
        print_number(&from);
        print!("To: ");
        print_number(&to);

        let result = super::reduce(from);

        print!("Re: ");
        print_number(&result);

        assert_eq!(to, result);
    }

    #[test]
    fn explode_1() {
        check("[[[[[9,8],1],2],3],4]", "[[[[0,9],2],3],4]")
    }

    #[test]
    fn explode_2() {
        check("[7,[6,[5,[4,[3,2]]]]]", "[7,[6,[5,[7,0]]]]");
    }

    #[test]
    fn explode3() {
        check("[[6,[5,[4,[3,2]]]],1]", "[[6,[5,[7,0]]],3]");
    }

    #[test]
    fn explode5() {
        check("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", "[[3,[2,[8,0]]],[9,[5,[7,0]]]]");
    }

    #[test]
    fn split1() {
        check("[[3,10],[1,[11,2]]]", "[[3,[5,5]],[1,[[5,6],2]]]");
    }
}