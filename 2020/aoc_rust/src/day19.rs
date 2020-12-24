extern crate regex;

use std::collections::HashMap;
use regex::Regex;

pub fn run_day19(puzzle_input: &str) {
    let input: Vec<String> = std::fs::read_to_string(&puzzle_input)
        .expect(format!(
            "Could not read file \"{}\".",
            &puzzle_input
        ).as_str()).trim()
        .split("\n\n")
        .map(|s| String::from(s))
        .collect();
    if input.len() != 2 {
        println!("Input file is not separated into rules and messages.");
        std::process::exit(1);
    }
    let rules: String = input[0].clone();
    let messages: Vec<String> = input[1].split('\n')
        .map(|s| String::from(s)).collect();
    let rule_set = RuleSet::new(&rules);
    let match_count = messages.iter()
        .map(|m| if rule_set.matches(m) {1} else {0})
        .sum::<usize>();
    println!("Number of matches: {}", match_count);
}

enum SubRuleItem {
    Literal(char),
    Reference(usize)
}

struct RuleSet {
    /// Each rule consists of a vec of sub-rules, at least one of which
    /// must match.
    /// Each sub-rule consists of a sequence of either literal
    /// chars or references to other rules.
    rules: HashMap<usize, Vec<Vec<SubRuleItem>>>
}

impl RuleSet {
    fn new(str_rules: &str) -> RuleSet {
        lazy_static! {
            // (?m) enables "multiline mode":
            static ref RULE: Regex = Regex::new(
                r"(?m)^(?P<rule_id>\d+): (?P<subrules>.*)$"
            ).unwrap();
        }
        let mut rules: HashMap<usize, Vec<Vec<SubRuleItem>>> = HashMap::new();
        for rule_cap in RULE.captures_iter(&str_rules) {
            let rule_id: usize = rule_cap.name("rule_id").unwrap().as_str()
                .parse::<usize>().unwrap();
            let mut sub_rules: Vec<Vec<SubRuleItem>> = Vec::new();
            for sub_rule_str in rule_cap.name("subrules").unwrap().as_str()
                    .split(" | ") {
                let mut sub_rule: Vec<SubRuleItem> = Vec::new();
                for item_str in sub_rule_str.trim().split(' ') {
                    let item_str = item_str.trim();
                    if item_str.starts_with('"') {
                        sub_rule.push(SubRuleItem::Literal(
                            item_str.chars().nth(1).unwrap()
                        ));
                    } else {
                        sub_rule.push(SubRuleItem::Reference(
                            item_str.parse::<usize>().unwrap()
                        ));
                    }
                }
                sub_rules.push(sub_rule);
            }
            rules.insert(rule_id, sub_rules);
        }
        RuleSet{ rules }
    }

    fn matches(&self, message: &str) -> bool {
        self.match_len(&message, 0) == message.len()
    }

    /// Returns the number of matched characters for the given rule.
    /// Will return 0 if the message doesn't match.
    fn match_len(&self, message: &str, rule_id: usize) -> usize {
        match self.rules.get(&rule_id) {
            Some(sub_rules) => {
                // Any sub-rule must match:
                for sub_rule in sub_rules {
                    // All items must match:
                    // if any doesn't match, continue w/ next sub-rule
                    let mut num_matched: usize = 0;
                    for item in sub_rule {
                        if message.len() <= num_matched {
                            // There's another item, but we've already reached
                            // the end of the message.
                            num_matched = 0;
                            break;
                        }
                        match item {
                            SubRuleItem::Literal(c) => {
                                if message.chars().nth(num_matched).unwrap()
                                        == *c {
                                    num_matched += 1;
                                }
                            },
                            SubRuleItem::Reference(r) => {
                                let new_matches = self.match_len(
                                    &message[num_matched..],
                                    *r
                                );
                                if new_matches > 0 {
                                    num_matched += new_matches;
                                } else {
                                    num_matched = 0;
                                    break;
                                }
                            }
                        }
                    }
                    if num_matched > 0 {
                        return num_matched
                    } // else try the next sub-rule
                }
                0
            },
            None => {
                panic!("No rule found for id {}", rule_id);
            }
        }
    }
}
