use super::common;
use std::path::Path;

#[derive(Default)]
struct Pattern {
    segments: [char;7],
    len: u8,
}

impl Pattern {
    pub fn push(&mut self, c: char) {
        let idx = self.len as usize;
        assert!(idx < self.segments.len());

        self.len += 1;
        self.segments[idx] = c;
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

    // Each entry is a table that maps a wire to segment. So the index
    // is the wire in the original and the value is the real segment it
    // should map to.
    let mut translation_tables: Vec<[char;7]> = Vec::new();
    translation_tables.reserve(display_outputs.len());

    fn set_on_table(table: &mut[char], value_of: char, index_at: char) {
        let idx = (index_at as u8) - ('a' as u8);
        table[idx as usize] = value_of;
    }

    fn map_value(table: &[char], lookup: char) -> char {
        let idx = (lookup as u8) - ('a' as u8);
        table[idx as usize]
    }

    fn set_op(lhs: &[char], rhs: &[char], should_contain: bool) -> Pattern {
        let mut common = Pattern::default();
        for seg in lhs {
            if (rhs.contains(&seg) == should_contain) &&
               (*seg != char::default()) {
                common.push(*seg)
            }
        }
        common
    }

    let and_op = true;
    let not_op = false;

    fn print_table(table: &[char;7]) {
        println!(" {}{}{}{} ", table[0], table[0], table[0], table[0]);
        println!("{}    {}", table[1], table[2]);
        println!("{}    {}", table[1], table[2]);
        println!(" {}{}{}{} ", table[3], table[3], table[3], table[3]);
        println!("{}    {}", table[4], table[5]);
        println!("{}    {}", table[4], table[5]);
        println!(" {}{}{}{} ", table[6], table[6], table[6], table[6]);
    }

    for output in &display_outputs {
        let mut translation: [char;7] = Default::default();

        let one_pattern = output.patterns.iter().find(|pattern| {
            pattern.len == 2 
        }).unwrap();

        let four_pattern = output.patterns.iter().find(|pattern| {
            pattern.len == 4
        }).unwrap();

        let seven_pattern = output.patterns.iter().find(|pattern| {
            pattern.len == 3
        }).unwrap();

        let c_and_f = set_op(&one_pattern.segments, &seven_pattern.segments, and_op);
            
        let a = set_op(&seven_pattern.segments, &c_and_f.segments, not_op).segments[0];

        set_on_table(&mut translation, 'a', a);

        let b_and_d = set_op(&four_pattern.segments, &one_pattern.segments, not_op);

        let three_pattern = output.patterns.iter().find(|pattern| {
            pattern.len == 5 &&
            pattern.segments.contains(&a) &&
            pattern.segments.contains(&c_and_f.segments[0]) &&
            pattern.segments.contains(&c_and_f.segments[1])
        }).unwrap();

        let g = *three_pattern.segments.iter().find(|wire| {
            **wire != a &&
            **wire != c_and_f.segments[0] &&
            **wire != c_and_f.segments[1] &&
            **wire != b_and_d.segments[0] &&
            **wire != b_and_d.segments[1] 
        }).unwrap();

        set_on_table(&mut translation, 'g', g);

        let d = *b_and_d.segments.iter().find(|wire| {
            three_pattern.segments.contains(wire) &&
            four_pattern.segments.contains(wire)
        }).unwrap();

        set_on_table(&mut translation, 'd', d);

        let mut b = ' ';
        for i in 0..b_and_d.len as usize {
            if b_and_d.segments[i] != d {
                b = b_and_d.segments[i];
                set_on_table(&mut translation, 'b', b);
            }
        }

        let five_pattern = output.patterns.iter().find(|pattern| {
            pattern.len == 5 &&
            pattern.segments.contains(&a) &&
            pattern.segments.contains(&g) &&
            pattern.segments.contains(&d) &&
            pattern.segments.contains(&b)
        }).unwrap();

        let f = *c_and_f.segments.iter().find(|wire| { 
            five_pattern.segments.contains(wire)
        }).unwrap();

        set_on_table(&mut translation, 'f', f);

        let mut c = ' ';
        for i in 0..c_and_f.len as usize {
            if c_and_f.segments[i] != f {
                c = c_and_f.segments[i];
                set_on_table(&mut translation, 'c', c);
            }
        }

        let missing_char = ('a'..='g').find(|wire| {
            !translation.contains(wire)
        }).unwrap();

        for final_char in &mut translation {
            if *final_char == char::default() {
                *final_char = missing_char;
            }
        }

        translation_tables.push(translation);
    }

//     0:      1:      2:      3:      4:
//     aaaa    ....    aaaa    aaaa    ....
//    b    c  .    c  .    c  .    c  b    c
//    b    c  .    c  .    c  .    c  b    c
//     ....    ....    dddd    dddd    dddd
//    e    f  .    f  e    .  .    f  .    f
//    e    f  .    f  e    .  .    f  .    f
//     gggg    ....    gggg    gggg    ....
   
//      5:      6:      7:      8:      9:
//     aaaa    aaaa    aaaa    aaaa    aaaa
//    b    .  b    .  .    c  b    c  b    c
//    b    .  b    .  .    c  b    c  b    c
//     dddd    dddd    ....    dddd    dddd
//    .    f  e    f  .    f  e    f  .    f
//    .    f  e    f  .    f  e    f  .    f
//     gggg    gggg    ....    gggg    gggg

    fn pattern_to_digit(pattern: &str) -> u32 {
        match pattern {
            "abcefg" => 0,
            "cf" => 1,
            "acdeg" => 2,
            "acdfg" => 3,
            "bcdf" => 4,
            "abdfg" => 5,
            "abdefg" => 6,
            "acf" => 7,
            "abcdefg" => 8,
            "abcdfg" => 9,
            _ => panic!("Unexpected string {}", pattern),
        }
    }

    fn translate_pattern(pattern: &Pattern, table: &[char;7]) -> String {
        let mut translated = Pattern::default();
        for i in 0..(pattern.len as usize) {
            let mapped = map_value(table, pattern.segments[i]);
            translated.push(mapped);
        }

        translated.segments.sort();
        
        String::from_iter(translated.segments.iter().filter(|c| {
            **c != char::default() 
         }).into_iter())
    }

    let mut total_sum = 0;

    for i in 0..display_outputs.len() {
        let mapping_table = &translation_tables[i];

        let mut four_digit = 0;
        for dig_idx in 0..4 {
            let pattern = translate_pattern(&display_outputs[i].digits[dig_idx], mapping_table);
            let digit =  pattern_to_digit(pattern.trim_start());

            four_digit += digit * 10u32.pow(3 - dig_idx as u32);
        }

        total_sum += four_digit;
    }

    println!("Total digit sum: {}", total_sum);
}