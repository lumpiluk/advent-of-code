extern crate clap;
use clap::{Arg, App};

// #[macro_use]
// extern crate lazy_static;

mod day01;
mod day02;
mod day03;
mod day04;

fn main() {
    let matches = App::new("Advent of Code solutions 2021")
        .version("4")
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
        1 => day01::run_day01(&puzzle_input_filename),
        2 => day02::run_day02(&puzzle_input_filename),
        3 => day03::run_day03(&puzzle_input_filename),
        4 => day04::run_day04(&puzzle_input_filename),
        _ => println!("Invalid day: {}", day)
    }
}
