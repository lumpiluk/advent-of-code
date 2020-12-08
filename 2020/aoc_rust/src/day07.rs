extern crate regex;

use std::collections::{HashMap, HashSet};
use std::string::String;
use regex::Regex;

pub fn run_day07(puzzle_input: &str) {
    let contents = std::fs::read_to_string(&puzzle_input)
        .expect("Something went wrong reading the file");

    let rs = RuleSet::new(&contents);
    let valid_outermost_bags = rs.get_valid_outermost_bags("shiny gold");
    println!(
        "{:?}\n->number of valid outermost bags: {}",
        valid_outermost_bags,
        valid_outermost_bags.len(),
    );

    println!(
        "Number of bags in my shiny gold bag: {}",
        rs.get_number_of_nested_bags("shiny gold", None)
    )
}

#[derive(Debug)]
struct BagContent {
    num: u32,
    color: String
}

struct RuleSet {
    rules: HashMap<String, HashMap<String, BagContent>>,
}

impl RuleSet {
    fn new(rules_text: &str) -> RuleSet {
        lazy_static! {
            // (?xm) enables "insignificant whitespace mode" and
            // "multiline mode":
            static ref RULE: Regex = Regex::new(r"(?xm)
                ^(?P<container_color>\w+\s\w+)\s  # container color
                bags\scontain\s
                (?P<contents_text>.*)$
            ").unwrap();
            static ref CONTENT: Regex = Regex::new(r"(?x)
                (?P<num>\d+)\s
                (?P<content_color>\w+\s\w+)\s
                bags?(?:,\s|\.)
            ").unwrap();
        }

        let mut rs = RuleSet {
            rules: HashMap::new()
        };
        for rule_cap in RULE.captures_iter(&rules_text) {
            // println!("Capture: {:?}", rule_cap);
            let mut bag_contents: HashMap<String, BagContent> = HashMap::new();
            for content_cap in CONTENT.captures_iter(rule_cap
                .name("contents_text").unwrap().as_str()
            ) {
                let color = content_cap.name("content_color")
                    .unwrap().as_str();
                bag_contents.insert(
                    color.to_string(),
                    BagContent {
                        num: content_cap.name("num")
                            .unwrap().as_str().parse::<u32>().unwrap(),
                        color: color.to_string()
                    }
                );
                // println!("\tContent: {:?}",
                //     bag_contents.get(color.to_string()));
            }

            rs.rules.insert(
                rule_cap.name("container_color")
                    .unwrap().as_str().to_string(),
                bag_contents
            );
        }
        rs
    }

    fn get_valid_outermost_bags(&self, inner_color: &str) -> HashSet<String> {
        fn insert_recursively(
            rs: &RuleSet,
            valid: &mut HashSet<String>,
            new_color: &str
        ) {
            if valid.contains(new_color) { return; }
            for (outer, contents) in rs.rules.iter() {
                if contents.contains_key(new_color) {
                    insert_recursively(rs, valid, outer);
                }
            }
            valid.insert(new_color.to_string());
        }

        let mut valid: HashSet<String> = HashSet::new();
        insert_recursively(self, &mut valid, &inner_color);

        // Remove inner color again as we only want the outermost bag colors:
        valid.remove(&inner_color.to_string());
        valid
    }

    fn get_number_of_nested_bags(
        &self,
        outer_color: &str,
        visited_colors: Option<HashSet<String>>
    ) -> u32 {
        let mut count: u32 = 0;
        let mut new_visited_colors: HashSet<String>;
        if let Some(visited_colors) = visited_colors {
            if visited_colors.contains(outer_color) {
                panic!(
                    "Loop detected: outer_color=\"{}\"\n{:?}",
                    outer_color,
                    visited_colors
                );
            }
            new_visited_colors = visited_colors.clone();
        } else {
            new_visited_colors = HashSet::new();
        }
        new_visited_colors.insert(outer_color.to_string());
        if let Some(content) = self.rules.get(outer_color) {
            for (color, content) in content.iter() {
                count += content.num;
                count += content.num * self.get_number_of_nested_bags(
                    color,
                    Some(new_visited_colors.clone())
                )
            }
        }
        count
    }
}
