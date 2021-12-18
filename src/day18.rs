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
    Ignore,
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
        ' ' => ParseResult::Ignore,    
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

fn numbers_from_byte_slice(bytes: &[u8]) -> Vec<VecDeque<Digit>> {
    let mut all_numbers = Vec::new();
    let mut number = VecDeque::new();
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
            ParseResult::Newline => {
                assert!(literal_buf.is_empty());
                if !number.is_empty() {
                    all_numbers.push(number);
                    number = VecDeque::new();
                }
            },
            ParseResult::Ignore => (),
        }
    }

    all_numbers.push(number);
    all_numbers
}

#[must_use]
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

#[must_use]
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

// #[derive(Clone, Copy, PartialEq)]
// enum Action {
//     Explode(usize),
//     Split(usize),
//     None,
// }

// fn find_next_action(number: &VecDeque<Digit>) -> Option<Action> {
//     let mut scopes = 0;
    
//     for i in 0..number.len() {
//         let next_action = match number[i] {
//             Digit::ScopeClose => {
//                 scopes -= 1;
//                 assert!(scopes >= 0);
//                 Action::None   
//             },
//             Digit::ScopeOpen => {
//                 scopes += 1;
//                 if scopes > 4 {
//                     Action::Explode(i)
//                 } else {
//                     Action::None
//                 }
//             },
//             Digit::Literal(x) => if x > 9 {
//                 Action::Split(i)
//             } else {
//                 Action::None
//             },
//         };

//         if next_action != Action::None {
//             return Some(next_action);
//         }
//     }

//     None
// }

// todo we can search all at once and then pick the preference
fn find_next_explode(number: &VecDeque<Digit>) -> Option<usize> {
    let mut scopes = 0;
    
    for i in 0..number.len() {
        match number[i] {
            Digit::ScopeOpen => {
                scopes += 1;
                if scopes > 4 {
                    return Some(i);
                }
            }
            Digit::ScopeClose => {
                scopes -= 1;
                assert!(scopes >= 0);
            },
            _ => (),
        };
    }

    None
}

fn find_next_split(number: &VecDeque<Digit>) -> Option<usize> {        
    for i in 0..number.len() {
        match number[i] {
            Digit::Literal(x) => if x > 9 {
                return Some(i);
            },
            _ => (),
        };
    }

    None
}

fn reduce(mut number: VecDeque<Digit>) -> VecDeque<Digit> {
    loop {
        match find_next_explode(&number) {
            Some(explode_idx) => {
                number = explode_at(&mut number, explode_idx);
            },
            None => {
                match find_next_split(&number) {
                    Some(split_idx) => {
                        number = split_at(&mut number, split_idx);
                    },
                    None => break,
                }
            }
        }
        // if next_explode.is_some() {
        //     number = explode_at(&mut number, next_explode.unwrap());
        // } else {
        //     let next_split
        // }

        // match next_action {
        //     Some(Action::Explode(idx)) => {
        //         // println!("Explode at {}", idx);
        //     },
        //     Some(Action::Split(idx)) => {
        //         // println!("Split at {}", idx);
        //         number = split_at(&mut number, idx);
        //     },
        //     _ => (),
        //     // _ => println!("Finished"),
        // }

        // next_action = find_next_action(&number);

        // println!("Afterwards:");
        // print_number(&number);
    }

    number
}


fn print_number(number: &VecDeque<Digit>) {
    for i in 0..number.len() {
        print!("{}", number[i]);
        
        if i < (number.len() - 1) {
            match (number[i], number[i+1]) {
                (Digit::Literal(_), Digit::Literal(_)) => print!(","),
                (Digit::ScopeClose, Digit::ScopeOpen) => print!(","),
                _ => (),    
            }
        } 
    }
    println!();
}

fn add_numbers(into: &mut VecDeque<Digit>, from: &VecDeque<Digit>) {
    into.push_front(Digit::ScopeOpen);

    for digit in from {
        into.push_back(*digit);
    }

    into.push_back(Digit::ScopeClose);
}

// TODO: remove Action::None, just use Option
pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day18_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());
    
    // splitting happens in FILO order (splits may produce new actions
    // which get added to the top of the stack).
    
    let numbers = numbers_from_byte_slice(&bytes);

    let mut result: VecDeque<Digit> = numbers[0].clone();
    result = reduce(result);

    for number in &numbers[1..] {
        add_numbers(&mut result, number);
        result = reduce(result);
        print_number(&result);
    } 

    print_number(&result);
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    fn number_from_str_slice(str: &str) -> Vec<VecDeque<super::Digit>> {
        let bytes = str.chars().map(|c| c as u8).collect::<Vec<_>>();
        super::numbers_from_byte_slice(&bytes)
    }
    
    fn check(from_str: &str, to_str: &str) {
        let from = number_from_str_slice(from_str);
        let to = number_from_str_slice(to_str);

        let mut result = from[0].clone();
        result = super::reduce(result);
    
        for number in &from[1..] {
            super::add_numbers(&mut result, number);
            result = super::reduce(result);
        } 

        assert_eq!(to[0], result);
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

    #[test]
    fn full1() {
        check("[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
    }

    #[test]
    fn full2() {
        check("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]", "[[[[5,0],[7,4]],[5,5]],[6,6]]");
    }

    #[test]
    fn full3() {
        check("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]\n[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]\n[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]\n[7,[5,[[3,8],[1,4]]]]\n[[2,[2,2]],[8,[8,1]]]\n[2,9]\n[1,[[[9,3],9],[[9,0],[0,7]]]]\n[[[5,[7,4]],7],1]\n[[[[4,2],2],6],[8,7]]", 
        "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
    }
}