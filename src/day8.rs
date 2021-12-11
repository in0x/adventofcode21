use super::common;
use std::path::Path;

#[derive(Default, Copy, Clone)]
struct BitMask {
    mask: u8
}

impl BitMask {
    pub fn set_bit(&mut self, i: usize) {
        assert!(i < 7); // we only use 7 bits, one for each segment.
        self.mask |= 1 << i; 
    }

    pub fn and(&self, other: BitMask) -> BitMask {
        BitMask { mask: self.mask & other.mask }
    }

    pub fn not(&self, other: BitMask) -> BitMask {
        BitMask { mask: self.mask & (!other.mask) }
    }

    pub fn popcnt(&self) -> u32 {
        self.mask.count_ones()
    }

    pub fn contains_all(&self, other: BitMask) -> bool {
        (self.mask & other.mask) == other.mask
    }
}

fn bit_idx(char: u8) -> usize {
    ((char) - ('a' as u8)) as usize
}

#[derive(Default)]
struct Sample {
    patterns: [BitMask; 10],
    digits: [BitMask; 4],
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day8_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let mut samples = Vec::new();
    {
        let mut cursor = 0;
        loop {
            let mut sample = Sample::default();
            for i in 0..10 {
                let mut mask = BitMask::default();
                while (bytes[cursor] as char).is_ascii_alphabetic() {
                    mask.set_bit(bit_idx(bytes[cursor]));
                    cursor += 1;
                }
                
                cursor += 1;
                sample.patterns[i] = mask;
            }

            while !(bytes[cursor] as char).is_ascii_alphabetic() {
                cursor += 1;
            }

            for i in 0..4 {
                let mut mask = BitMask::default();                
                while (cursor < bytes.len()) && (bytes[cursor] as char).is_ascii_alphabetic() {
                    mask.set_bit(bit_idx(bytes[cursor]));
                    cursor += 1;
                }
                
                cursor += 1;
                sample.digits[i] = mask;
            }            

            samples.push(sample);

            if cursor >= bytes.len() {
                break;
            }

            while !(bytes[cursor] as char).is_ascii_alphabetic() {
                cursor += 1;
            }
        }
    }

    let mut num_unique_digits = 0;
    for sample in &samples {
        for digit in &sample.digits {
            match digit.popcnt() {
                2 | 3 | 4 | 7  => num_unique_digits += 1,
                _ => (),
            }
        }
    }

    println!("Num output digits with unqiue counts: {}", num_unique_digits);

    // Sort by set bits ascending, this way we can reliably
    // grab the patterns with unique num bits by index:
    // Bits: [2, 3, 4, 5, 5, 5, 6, 6, 6, 7]
    // Num:  [1, 7, 4, ?, ?, ?, ?, ?, ?, 8]
    for sample in &mut samples {
        sample.patterns.sort_by(|lhs, rhs| {
            let pcnt_l = lhs.popcnt();
            let pcnt_r = rhs.popcnt();

            pcnt_l.partial_cmp(&pcnt_r).unwrap()
        });
    }

    fn print_bin(mask: BitMask) {
        println!("  0gfedcba");
        println!("{:#010b}", mask.mask);
    }

    fn bit_i(mask: BitMask) -> u8 {
        assert!(mask.popcnt() == 1);
        mask.mask.trailing_zeros() as u8
    }

    let mut mapping_tables = Vec::new();

    for sample in &samples {
        let mut mapping: [u8;7] = Default::default();

        let one = sample.patterns[0];
        let seven = sample.patterns[1];
        let four = sample.patterns[2];

        let c_and_f = one.and(seven);
        let b_and_d = four.not(one);
        let a = seven.not(c_and_f);

        mapping[0] = bit_i(a);

        let three = *sample.patterns.iter().find(|p| {
            p.popcnt() == 5 &&
            p.contains_all(a) &&
            p.contains_all(c_and_f)
        }).unwrap();

        let g = three.not(a).not(c_and_f).not(b_and_d);
        mapping[6] = bit_i(g);

        let d = b_and_d.and(three).and(four);
        let b = b_and_d.not(d);

        mapping[1] = bit_i(b);
        mapping[3] = bit_i(d);

        let five = *sample.patterns.iter().find(|p| {
            p.popcnt() == 5 &&
            p.contains_all(a) &&
            p.contains_all(g) &&
            p.contains_all(d) &&
            p.contains_all(b)
        }).unwrap();

        let f = c_and_f.and(five);
        let c = c_and_f.not(f);

        mapping[2] = bit_i(c);
        mapping[5] = bit_i(f);

        let e = {
            let all_bits = BitMask { mask: 127 };
            all_bits.not(a).not(b).not(c).not(d).not(f).not(g)
        };

        mapping[4] = bit_i(e);
        mapping_tables.push(mapping);
    }
        
    //   0000  
    //  1    2 
    //  1    2 
    //   3333 
    //  4    5 
    //  4    5 
    //   6666 

    let mut total_sum = 0;

    for i in 0..samples.len() {
        let map = &mapping_tables[i];
        
        fn bit(i: u8) -> u8 { 1 << i }
        fn not_bit(i: u8) -> u8 { !(1 << i) }

        let mut pattern_to_value: [u8; 10] = Default::default();
        pattern_to_value[0] = 127 & not_bit(map[3]);
        pattern_to_value[1] = bit(map[2]) | bit(map[5]);
        pattern_to_value[2] = 127 & not_bit(map[1]) & not_bit(map[5]);
        pattern_to_value[3] = 127 & not_bit(map[1]) & not_bit(map[4]);
        pattern_to_value[4] = bit(map[1]) | bit(map[2]) | bit(map[3]) | bit(map[5]);
        pattern_to_value[5] = 127 & not_bit(map[2]) & not_bit(map[4]);
        pattern_to_value[6] = 127 & not_bit(map[2]);
        pattern_to_value[7] = bit(map[0]) | bit(map[2]) | bit(map[5]);
        pattern_to_value[8] = 127;
        pattern_to_value[9] = 127 & not_bit(map[4]);

        for dig_idx in 0..4 {
            let digit = &samples[i].digits[dig_idx];

            let value = pattern_to_value.iter().
                position(|p| *p == digit.mask).unwrap();

            total_sum += value as u32 * 10u32.pow(3 - dig_idx as u32);
        }
    }

    println!("Total digit sum: {}", total_sum);
}