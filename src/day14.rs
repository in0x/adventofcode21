use super::common;
use std::path::Path;
use std::collections::HashMap;

#[derive(Default, Clone)]
struct Rule {
    from: String,
    into: [String; 2],
    into_char: char,
}

pub fn run(root_dir: &Path) {
    let input_path = root_dir.join("day14_input.txt");
    let bytes = common::read_input_bytes(input_path.as_path());

    let (template, rules) = {
        let template = bytes.iter()
            .take_while(|b| (**b).is_ascii_alphabetic())
            .map(|b| *b as char)
            .collect::<Vec<_>>();

        let mut cursor = 0;

        // Scan past template string and initial whitespace
        while bytes[cursor].is_ascii_alphabetic() {
            cursor += 1;
        }
        while !bytes[cursor].is_ascii_alphabetic() {
            cursor += 1;
        }

        let mut rules = Vec::new();

        loop {
            let mut rule = Rule::default();
            let from_0 = bytes[cursor] as char;
            cursor += 1;
            let from_1 = bytes[cursor] as char;
            cursor += 1;

            while !bytes[cursor].is_ascii_alphabetic() {
                cursor += 1;
            }

            let into = bytes[cursor] as char;
            cursor += 1;

            rule.from.push(from_0);
            rule.from.push(from_1);
            rule.into[0].push(from_0);
            rule.into[0].push(into);
            rule.into[1].push(into);
            rule.into[1].push(from_1);
            rule.into_char = into;

            rules.push(rule);

            while cursor < bytes.len() && !bytes[cursor].is_ascii_alphabetic() {
                cursor += 1;
            }

            if cursor >= bytes.len() {
                break;
            }
        }

        (template, rules)
    };  

    let mut pair_map: HashMap<String, usize> = HashMap::new();
    let mut char_counts: HashMap<char, usize> = HashMap::new();

    for i in 1..template.len() {
        let pair = String::from_iter(&template[i-1..=i]);

        match pair_map.get_mut(&pair) {
            Some(count) => *count += 1,
            None => {
                pair_map.insert(pair, 1);
            }
        }
    }

    for character in &template {
        match char_counts.get_mut(character) {
            Some(count) => *count += 1,
            None => {
                char_counts.insert(*character, 1);
            }
        }
    }

    #[derive(PartialEq)]
    enum MutType { Add, Sub }
    let mut mutations: Vec<(&String, usize, MutType)> = Vec::new();

    let num_iterations = 40;
    for _ in 0..num_iterations {
        for rule in &rules {
            let found_count = match pair_map.get_mut(&rule.from) {
                Some(0) => {
                    None
                },
                Some(c) => {
                    mutations.push((&rule.from, *c, MutType::Sub));
                    Some(*c)
                },
                None => None,
            };

            if found_count.is_none() {
                continue;
            }

            mutations.push((&rule.into[0], found_count.unwrap(), MutType::Add));
            mutations.push((&rule.into[1], found_count.unwrap(), MutType::Add));

            match char_counts.get_mut(&rule.into_char) {
                Some(count) => *count += found_count.unwrap(),
                None => {
                    char_counts.insert(rule.into_char, found_count.unwrap());
                }
            }
        }

        for mutation in &mutations {
            if mutation.2 == MutType::Add {
                match pair_map.get_mut(mutation.0) {
                    Some(count) => *count += mutation.1,
                    None => {
                        pair_map.insert(mutation.0.clone(), mutation.1);
                    }
                }
            } else {
                match pair_map.get_mut(mutation.0) {
                    Some(count) => *count -= mutation.1,
                    None => (),
                }
            }
        }

        mutations.clear();
    }

    // println!("{:?}", pair_map);

    let mut min_count = usize::MAX;
    let mut max_count = usize::MIN;

    for kvp in &char_counts {
        min_count = usize::min(*kvp.1, min_count);
        max_count = usize::max(*kvp.1, max_count);
    }

    println!("Min {} Max {} Diff {}", min_count, max_count, max_count - min_count);
}