// Enable 'unstable library feature':
#![feature(is_sorted, map_first_last)]

extern crate clap;
use clap::{Arg, App};

#[macro_use]
extern crate lazy_static;

mod fileutils;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

fn main() {
    let matches = App::new("Advent of Code solutions")
        .version("14")
        .author("Lukas Stratmann")
        .arg(Arg::with_name("day")
             .short("d")
             .takes_value(true)
             .value_name("DAY")
             .required(true)
             .help("Day in the calendar"))
        .arg(Arg::with_name("puzzle_input")
             .short("i")
             .long("puzzle_input")
             .value_name("FILE")
             .help("The puzzle input file.")
             .required(true))
        .get_matches();

    let day: i32;
    match matches.value_of("day").unwrap().parse::<i32>() {
        Ok(n) => {day = n},
        Err(_) => {
            println!("Day must be numeric.");
            std::process::exit(1);
        }
    }
    let puzzle_input_filename = matches.value_of("puzzle_input").unwrap();
    match day {
        5 => day05::run_day05(&puzzle_input_filename),
        6 => day06::run_day06(&puzzle_input_filename),
        7 => day07::run_day07(&puzzle_input_filename),
        8 => day08::run_day08(&puzzle_input_filename),
        9 => day09::run_day09(&puzzle_input_filename),
        10 => day10::run_day10(&puzzle_input_filename),
        11 => day11::run_day11(&puzzle_input_filename),
        12 => day12::run_day12(&puzzle_input_filename),
        13 => day13::run_day13(&puzzle_input_filename),
        14 => day14::run_day14(&puzzle_input_filename),
        15 => day15::run_day15(&puzzle_input_filename),
        16 => day16::run_day16(&puzzle_input_filename),
        17 => day17::run_day17(&puzzle_input_filename),
        18 => day18::run_day18(&puzzle_input_filename),
        19 => day19::run_day19(&puzzle_input_filename),
        _ => println!("Invalid day: {}", day)
    }
}
