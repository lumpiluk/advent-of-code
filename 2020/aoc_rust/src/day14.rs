extern crate regex;

use std::vec::Vec;
use std::collections::HashMap;
use regex::Regex;

pub fn run_day14(puzzle_input: &str) {
    lazy_static! {
        static ref MASK: Regex = Regex::new(
            r"^mask = (?P<mask>[01X]+)$"
        ).unwrap();
        static ref MEM: Regex = Regex::new(
            r"^mem\[(?P<addr>\d+)\] = (?P<val>\d+)$"
        ).unwrap();
    }
    let code: Vec<String> = std::fs::read_to_string(&puzzle_input)
        .expect(format!("Could not read file \"{}\".", puzzle_input).as_str())
        .split('\n')
        .map(|l| String::from(l))
        .collect();

    let mem = MaskV1::compute(&code[..], &MASK, &MEM);
    let mem_sum: u64 = mem.values().sum();
    println!("Version 1: Sum of values in memory: {}", mem_sum);

    let mem = MaskV2::compute(&code[..], &MASK, &MEM);
    let mem_sum: u64 = mem.values().sum();
    println!("Version 2: Sum of values in memory: {}", mem_sum);
}

struct MaskV2 {
    ones_fixed: u64,  // if a bit is 1: override mem addr w/ 1
    ones_floating: Vec<u64>,  // if a bit is 1: override mem addr w/ 1
    zeros_floating: Vec<u64>,  // if a bit is 0: override mem addr w/ 0
}

impl MaskV2 {
    fn new(mask: &str) -> MaskV2 {
        assert!(mask.len() == 36);
        let mut ones_fixed = 0;
        let mut ones_floating: Vec<u64> = vec![0];
        let mut zeros_floating: Vec<u64> = vec![u64::MAX];
        for (i, c) in mask.chars().rev().enumerate() {
            match c {
                '1' => { ones_fixed |= 1 << i; },
                'X' => {
                    let mut new_ones_f: Vec<u64> = Vec::new();
                    let mut new_zeros_f: Vec<u64> = Vec::new();
                    for (one_f, zero_f) in ones_floating
                            .iter().zip(&zeros_floating) {
                        // Order is important b/c items will be zipped:
                        new_ones_f.push(*one_f);
                        new_ones_f.push(*one_f | (1 << i));
                        new_zeros_f.push(*zero_f ^ (1 << i));
                        new_zeros_f.push(*zero_f);
                    }
                    ones_floating = new_ones_f;
                    zeros_floating = new_zeros_f;
                },
                '0' => {},  // ignore
                e => { panic!("Invalid bitmask char: {}", e); }
            }
        }
        MaskV2 { ones_fixed, ones_floating, zeros_floating }
    }

    fn invariant() -> MaskV2 {
        MaskV2 {
            ones_fixed: 0,
            ones_floating: Vec::new(),
            zeros_floating: Vec::new()
        }
    }

    fn apply(
        &self,
        addr: u64,
        val: u64,
        mem: &mut HashMap<u64, u64>
    ) {
        let addr = addr | self.ones_fixed;
        for (one_f, zero_f) in self.ones_floating.iter()
                .zip(&self.zeros_floating) {
            mem.insert((addr & zero_f) | one_f, val);
        }
    }

    fn compute(
        code: &[String],
        mask_pattern: &Regex,
        mem_pattern: &Regex
    ) -> HashMap<u64, u64> {
        let mut current_mask: MaskV2 = MaskV2::invariant();
        let mut mem: HashMap<u64, u64> = HashMap::new();
        for line in code {
            if let Some(mask_cap) = mask_pattern.captures(&line) {
                current_mask = MaskV2::new(
                    mask_cap.name("mask")
                       .unwrap().as_str()
                );
            } else if let Some(mem_cap) = mem_pattern.captures(&line) {
                current_mask.apply(
                    mem_cap.name("addr").unwrap().as_str()
                        .parse::<u64>().unwrap(),
                    mem_cap.name("val").unwrap().as_str()
                        .parse::<u64>().unwrap(),
                    &mut mem
                );
            }
        }
        mem
    }
}

struct MaskV1 {
    ones: u64,  // if a bit is 1: override with 1, 0: leave as is
    zeros: u64  // if a bit is 0: override with 0, 1: leave as is
}

impl MaskV1 {
    fn new(mask: &str) -> MaskV1 {
        assert!(mask.len() == 36);
        let mut ones = 0;
        let mut zeros = u64::MAX;
        for (i, c) in mask.chars().rev().enumerate() {
            match c {
                '0' => { zeros ^= 1 << i; },
                '1' => { ones |= 1 << i; },
                'X' => {},  // ignore
                e => { panic!("Invalid bitmask char: {}", e); }
            }
        }
        MaskV1 { ones, zeros }
    }

    fn invariant() -> MaskV1 {
        MaskV1 { ones: 0, zeros: u64::MAX }
    }

    fn apply(&self, val: u64) -> u64 {
        (val & self.zeros) | self.ones
    }

    fn compute(
        code: &[String],
        mask_pattern: &Regex,
        mem_pattern: &Regex
    ) -> HashMap<u64, u64> {
        let mut current_mask: MaskV1 = MaskV1::invariant();
        let mut mem: HashMap<u64, u64> = HashMap::new();
        for line in code {
            if let Some(mask_cap) = mask_pattern.captures(&line) {
                current_mask = MaskV1::new(
                    mask_cap.name("mask")
                       .unwrap().as_str()
                );
            } else if let Some(mem_cap) = mem_pattern.captures(&line) {
                mem.insert(
                    mem_cap.name("addr").unwrap().as_str()
                        .parse::<u64>().unwrap(),
                    current_mask.apply(
                        mem_cap.name("val").unwrap().as_str()
                            .parse::<u64>().unwrap()
                    )
                );
            }
        }
        mem
    }
}
