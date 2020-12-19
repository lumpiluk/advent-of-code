extern crate regex;

use std::vec::Vec;
use std::collections::{HashMap, BTreeSet};
use regex::Regex;

pub fn run_day16(puzzle_input: &str) {
    let input: String = std::fs::read_to_string(&puzzle_input)
        .expect(format!(
            "Could not read file \"{}\".",
            &puzzle_input
        ).as_str());
    let sections: Vec<&str> = input.split("\n\n").collect();
    if sections.len() != 3 {
        panic!("\"{}\" is not a 3-section input file", &puzzle_input);
    }
    let rules: HashMap<String, Vec<(u32, u32)>> = parse_rules(&sections[0]);
    let my_ticket: Vec<Vec<u32>> = parse_tickets(&sections[1]);
    assert!(my_ticket.len() == 1);
    let my_ticket: Vec<u32> = my_ticket[0].clone();
    let nearby: Vec<Vec<u32>> = parse_tickets(&sections[2]);

    let scanning_err_rate: u32 = nearby.iter().map(|t| {
        get_invalid_values(&t, &rules).iter().sum::<u32>()
    }).sum();
    println!("Scanning error rate: {}", scanning_err_rate);

    let nearby_valid: Vec<Vec<u32>> = nearby.iter().filter_map(|t| {
        if get_invalid_values(t, &rules).len() == 0 {
            Some(t.clone())
        } else {
            None
        }
    }).collect();
    // determine order of fields
    if let Some(field_positions) = infer_field_positions(
        &rules,
        &nearby_valid
    ) {
        // multiply departure values of my_ticket
        let mut departure_vals_product: u64 = 1;
        for (field_name, pos) in &field_positions {
            if field_name.starts_with("departure") {
                departure_vals_product *= my_ticket[*pos] as u64;
            }
        }
        println!("Field positions: {:?}", &field_positions);
        println!("Product of my departure values: {}", departure_vals_product);
    } else {
        println!("Could not infer field positions, got stuck.");
    }
}

fn infer_field_positions(
    rules: &HashMap<String, Vec<(u32, u32)>>,
    valid_tickets: &[Vec<u32>],
) -> Option<HashMap<String, usize>> {
    assert!(valid_tickets.len() > 0);
    let num_positions = valid_tickets[0].len();
    let mut field_position_opts: HashMap<String, BTreeSet<usize>> =
        HashMap::new();
    for (rule_name, ranges) in rules {
        field_position_opts.insert(rule_name.clone(), BTreeSet::new());
        for pos in 0..num_positions {
            let mut pos_valid = true;
            for ticket in valid_tickets {
                if ticket.len() < num_positions {
                    panic!("Ticket is shorter than {} items: {:?}",
                           num_positions, &ticket);
                }
                let mut matches_any_range = false;
                for range in ranges {
                    if ticket[pos] >= range.0 && ticket[pos] <= range.1 {
                        matches_any_range = true;
                        break;
                    }
                }
                if !matches_any_range {
                    // We found an invalid ticket for this rule and
                    // position. -> Try the next position.
                    pos_valid = false;
                    break;
                }
            }
            if pos_valid {
                // All tickets are valid for this rule at position pos.
                field_position_opts.get_mut(rule_name).unwrap().insert(pos);
            }
        }
    }

    // Resolve rules that might apply to multiple positions.
    // E.g., {'rule a': [0, 1], 'rule b': [1]} -> {'rule a': 0, 'rule b': 1}
    println!("{:?}", &field_position_opts);
    let mut field_positions: HashMap<String, usize> = HashMap::new();
    loop {
        let mut unique_field: Option<(String, usize)> = None;
        if field_positions.len() == num_positions { break; }
        for (field, positions) in &field_position_opts {
            if field_positions.contains_key(field) { continue; }
            if positions.len() == 1 {
                unique_field = Some((field.clone(), *positions.first().unwrap()));
            }
        }
        if field_positions.len() < num_positions && unique_field == None {
            return None;
        }
        // Found a field with a single position -> remove this option from
        // all other fields.
        let unique_field = unique_field.unwrap();
        field_positions.insert(
            unique_field.0.clone(),
            unique_field.1
        );
        for (_, positions) in &mut field_position_opts {
            positions.remove(&unique_field.1);
        }
    }
    Some(field_positions)
}

fn get_invalid_values(
    ticket: &[u32],
    rules: &HashMap<String, Vec<(u32, u32)>>
) -> Vec<u32> {
    let mut invalid: Vec<u32> = Vec::new();
    for val in ticket {
        if !valid_rule_exists(*val, rules) {
            invalid.push(*val);
        }
    }
    invalid
}

fn valid_rule_exists(
    val: u32,
    rules: &HashMap<String, Vec<(u32, u32)>>
) -> bool {
    for rule_ranges in rules.values() {
        let mut in_some_range = false;
        for range in rule_ranges {
            if val >= range.0 && val <= range.1 {
                in_some_range = true;
            }
        }
        if in_some_range { return true; }
    }
    false
}

fn parse_tickets(tickets_str: &str) -> Vec<Vec<u32>> {
    let lines: Vec<String> = tickets_str.trim().split('\n')
        .map(|s| String::from(s)).collect();
    println!("first line: {}", lines[0]);
    assert!(lines[0] == "your ticket:" || lines[0] == "nearby tickets:");
    let mut tickets: Vec<Vec<u32>> = Vec::new();
    for line in &lines[1..] {
        tickets.push(
            line.split(',').filter_map(|s| {
                s.parse::<u32>().ok()
            }).collect()
        );
    }
    tickets
}

fn parse_rules(
    rules_str: &str
) -> HashMap<String, Vec<(u32, u32)>> {
    lazy_static! {
        // (?m) enables "multiline mode":
        static ref RULE: Regex = Regex::new(
            r"(?m)^(?P<name>[\w\s]+): (?P<ranges>.*)$"
        ).unwrap();
        static ref RANGE: Regex = Regex::new(
            r"(?P<from>\d+)-(?P<to>\d+)"
        ).unwrap();
    }
    let mut rules: HashMap<String, Vec<(u32, u32)>> = HashMap::new();
    for rule_cap in RULE.captures_iter(&rules_str) {
        let mut rule: Vec<(u32, u32)> = Vec::new();
        for range_cap in RANGE.captures_iter(
                rule_cap.name("ranges").unwrap().as_str()) {
            rule.push((
                range_cap.name("from").unwrap().as_str()
                    .parse::<u32>().unwrap(),
                range_cap.name("to").unwrap().as_str()
                    .parse::<u32>().unwrap()
            ));
        }
        rules.insert(
            String::from(rule_cap.name("name").unwrap().as_str()),
            rule
        );
    }
    rules
}
