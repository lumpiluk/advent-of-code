use std::collections::HashSet;
use std::vec::Vec;
use crate::fileutils::{self};

pub fn run_day06(puzzle_input: &str) {
    match fileutils::read_lines(puzzle_input) {
        Ok(lines) => {
            let mut group_answers_any: HashSet<char> = HashSet::new();
            let mut group_answers_every: Vec<HashSet<char>> = Vec::new();
            let mut answers_sum_any = 0;
            let mut first_person_in_group = true;
            group_answers_every.push(HashSet::new());

            for line in lines {
                if let Ok(line) = line {
                    if line.trim().is_empty() {
                        // start new passenger group
                        group_answers_any.clear();
                        group_answers_every.push(HashSet::new());
                        first_person_in_group = true;
                        continue;
                    }
                    if first_person_in_group {
                        for c in line.chars() {
                            group_answers_every.last_mut().unwrap().insert(c);
                        }
                        first_person_in_group = false;
                    } else {
                        let mut person_answers = HashSet::new();
                        for c in line.chars() {
                            person_answers.insert(c);
                        }
                        *group_answers_every.last_mut().unwrap() =
                            group_answers_every
                            .last()
                            .unwrap()
                            .intersection(&person_answers)
                            .cloned().collect();
                    }
                    for c in line.chars() {
                        answers_sum_any +=
                            (!group_answers_any.contains(&c)) as i32;
                        group_answers_any.insert(c);
                    }

                }
            }
            println!("Sum of group 'any' answers: {}", answers_sum_any);
            let answers_sum_every: u32 = group_answers_every.iter()
                .map(|ans_set| ans_set.len() as u32)
                .sum();
            println!("Sum of group 'every' answers: {}", answers_sum_every);
        },
        Err(e) => {
            println!("Could not read file \"{}\": {}", puzzle_input, e);
        }
    }
}
