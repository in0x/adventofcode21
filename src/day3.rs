use super::common;
use std::path::Path;

#[derive(Clone, Copy, Default)]
struct DigitCounts {
    zero: u32,
    one: u32,
}

const NUM_BIN_DIGITS : usize = 12; // Each binary number in our input has 12 digits.

fn fliter_and_reduce(input: &Vec<u32>, get_condition: fn(&DigitCounts) -> bool) -> Vec<u32> {
    let mut values = input.clone();
    
    for i in (0..(NUM_BIN_DIGITS)).rev() {
        let mut count = DigitCounts::default();

        for value in &values {
            let bit_set = (*value & (1 << i)) != 0;
            match bit_set {
                true => count.one += 1,
                false => count.zero += 1,
            }
        }

        let select_ones = get_condition(&count);

        values = values.iter().filter(|x| {
            let bit_set = (*x & (1 << i)) != 0;
            select_ones == bit_set 
        }).map(|x| *x).collect();

        if values.len() == 1 {
            break;
        }
    }

    values
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day3_input.txt");
    let input_bytes = common::read_input_bytes(input_path.as_path());

    let mut all_counts: Vec<DigitCounts> = Vec::new();
    all_counts.resize(NUM_BIN_DIGITS, Default::default());

    let mut parsed_values: Vec<u32> = Vec::new();

    let mut cursor = 0;
    loop {
        if cursor >= input_bytes.len() {
            break;
        }

        let mut parsed_value: u32 = 0;

        for digit_n in 0..NUM_BIN_DIGITS {
            match input_bytes[cursor + digit_n] as char {
                '0' => {
                    all_counts[digit_n].zero += 1;
                },
                '1' => {
                    all_counts[digit_n].one += 1;

                    // We're reading the numbers back to front, so offset
                    // for conversion to decimal.
                    let digit_pos = NUM_BIN_DIGITS - 1 - digit_n;
                    parsed_value += 2u32.pow(digit_pos as u32); 
                },
                _ => panic!("Unexpected char at pos {}", cursor + digit_n),
            }
        }

        parsed_values.push(parsed_value);
        cursor += NUM_BIN_DIGITS + 2; // Skip to start of next line.
    }

    let mut gamma = 0; 
    let mut epsilon = 0;
    let mut digit = 0;
    for count in all_counts.iter().rev() {
        if count.one > count.zero {
            gamma += 2u32.pow(digit); 
        } else {
            epsilon += 2u32.pow(digit); 
        }

        digit += 1;
    }

    println!("Gamma {}, Epsilon {}, Power: {}", gamma, epsilon, gamma * epsilon);

    let oxygen_values = fliter_and_reduce(&parsed_values, |count| {
        count.one > count.zero || count.one == count.zero
    });
    
    let co2_values = fliter_and_reduce(&parsed_values, |count| {
        count.one < count.zero
    });

    for v in &oxygen_values {
        println!("Oxygen {}", v);
    }

    for v in &co2_values {
        println!("CO2 {}", v);
    }

    println!("Life support {}", oxygen_values[0] * co2_values[0]);
}