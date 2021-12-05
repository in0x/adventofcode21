use super::common;
use std::path::Path;

fn parse_numbers_forward(input_bytes: &Vec<u8>) -> Vec<u32> {
    let mut numbers = Vec::new();

    let mut working_buf = Vec::new();
    working_buf.reserve(32);

    for character in input_bytes {
        match *character as char {
            '\n' | '\r' => {
                if working_buf.is_empty() {
                    continue;
                }

                let mut accumulator = 0;
                let mut position = 1;

                for digit in working_buf.iter().rev() {
                    accumulator += digit * position;
                    position *= 10;
                }

                working_buf.clear();
                numbers.push(accumulator);
            },
            '0'..='9' => {
                let digit_val = character - ('0' as u8);
                working_buf.push(digit_val as u32);
            },
            _ => {
                panic!("Found unexpected char \'{}\' in stream.", character);
            },
        }
    }

    if working_buf.len() > 0 {
        let mut accumulator = 0;
        let mut position = 1;
    
        for digit in working_buf.iter().rev() {
            accumulator += digit * position;
            position *= 10;
        }    

        numbers.push(accumulator);
    }

    numbers
}

#[allow(dead_code)]
fn parse_numbers_reverse(input_bytes: &Vec<u8>) -> Vec<u32> {
    let mut numbers = Vec::new();
    
    let mut accumulator = 0;
    let mut digit = 1;
    
    for character in input_bytes.iter().rev() {
        match *character as char {
            '\n' | '\r' => {
                if digit > 1 { // If we didnt push atleast one digit, we had consecutive whitespace.
                    numbers.push(accumulator);
                } 
                accumulator = 0;
                digit = 1;
            },
            '0'..='9' => {
                let digit_val = character - ('0' as u8);
                accumulator += (digit_val as u32) * digit;
                digit *= 10;
            },
            _ => {
                panic!("Found unexpected char \'{}\' in stream.", character);
            },
        }
    }

    if digit > 1 {
        numbers.push(accumulator); // The final value wont be terminated by a newline.
    }

    numbers.reverse();

    numbers
}

pub fn run(root_dir:  &Path) {
    let input_path = root_dir.join("day1_input.txt");
    let input_bytes = common::read_input_bytes(input_path.as_path());

    let samples = parse_numbers_forward(&input_bytes);

    let num_count = samples.len();
    println!("Parsed {} numbers!", num_count);

    let mut number_of_increases = 0;

    for i in 1..num_count {
        if samples[i] > samples[i - 1] {
            number_of_increases += 1;
        }
    }

    println!("Single depth increased {} times", number_of_increases);

    let three_sum_count = ((num_count / 3) * 3) - 3;
    let mut num_three_sum_increase = 0;
    for i in 1..three_sum_count {
        
        let lhs_idx = i - 1;
        let lhs_sum = samples[lhs_idx] + samples[lhs_idx + 1] + samples[lhs_idx + 2]; 
        let rhs_sum = samples[i]       + samples[i + 1]       + samples[i + 2]; 

        if rhs_sum > lhs_sum {
            num_three_sum_increase += 1;
        }
    }

    println!("Sliding window sum increased {} times", num_three_sum_increase);
}