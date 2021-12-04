use core::num;
use std::fs;
use std::path::Path;

pub fn run(root_dir:  &Path) {
    println!("Running day 1.");

    let input_path = root_dir.join("day1_input.txt");

    let input_bytes =  match fs::read(&input_path) {
        Err(why) => panic!("Failed to open input file {}: {}",
                                 input_path.to_str().unwrap(), why),
        Ok(bytes) => bytes
    };

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
    let num_count = numbers.len();
    println!("Parsed {} numbers!", num_count);

    let mut number_of_increases = 0;

    for i in 1..num_count {
        if numbers[i] > numbers[i - 1] {
            number_of_increases += 1;
        }
    }

    println!("Single depth increased {} times", number_of_increases);


}