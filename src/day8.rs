use super::common;
use std::path::Path;

#[derive(Default)]
struct Pattern {
    segments: [char;7],
    len: u8,
}

impl Pattern {
    pub fn push(&mut self, c: char) {
        assert!((self.len as usize) < self.segments.len());
        self.segments[self.len as usize] = c;
        self.len += 1;
    }
}

#[derive(Default)]
struct DisplayOutput {
    patterns: [Pattern;10],
    digits: [Pattern;4]
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day8_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let mut display_outputs = Vec::new();
    {
        let mut cursor = 0;
        loop {
            let mut output = DisplayOutput::default();
            for i in 0..10 {
                let mut pattern = Pattern::default();
                
                while (bytes[cursor] as char).is_ascii_alphabetic() {
                    pattern.push(bytes[cursor] as char);
                    cursor += 1;
                }
                
                cursor += 1;
                output.patterns[i] = pattern;
            }

            while !(bytes[cursor] as char).is_ascii_alphabetic() {
                cursor += 1;
            }

            for i in 0..4 {
                let mut pattern = Pattern::default();
                
                while (cursor < bytes.len()) && (bytes[cursor] as char).is_ascii_alphabetic() {
                    pattern.push(bytes[cursor] as char);
                    cursor += 1;
                }
                
                cursor += 1;
                output.digits[i] = pattern;
            }            

            display_outputs.push(output);

            if cursor >= bytes.len() {
                break;
            }

            while !(bytes[cursor] as char).is_ascii_alphabetic() {
                cursor += 1;
            }
        }
    }

    let mut num_unique_digits = 0;
    for output in &display_outputs {
        for digit in &output.digits {
            match digit.len {
                2 | 3 | 4 | 7  => num_unique_digits += 1,
                _ => (),
            }
        }
    }

    println!("Num output digits with unqiue counts: {}", num_unique_digits);

    // for output in &display_outputs {
    //     for pattern in &output.patterns {
    //         for i in 0..pattern.len {
    //             print!("{}", pattern.segments[i as usize]);
    //         }
    //         print!(" ");
    //     }
    //     print!("| ");
    //     for pattern in &output.digits {
    //         for i in 0..pattern.len {
    //             print!("{}", pattern.segments[i as usize]);
    //         }
    //         print!(" ");
    //     }
    //     println!();
    // }
}