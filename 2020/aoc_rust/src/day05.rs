use crate::fileutils::{self};

pub fn run_day05(puzzle_input: &str) {
    match fileutils::read_lines(puzzle_input) {
        Ok(lines) => {
            let mut max_id = 0;
            let mut ids = Vec::new();
            for line in lines {
                if let Ok(boarding_pass) = line {
                    let seat = PlaneSeat::new(&boarding_pass);
                    max_id = std::cmp::max(max_id, seat.id);
                    ids.push(seat.id);
                    println!("boarding pass: {}, {:?}", boarding_pass, seat);
                }
            }
            println!("Maximum seat ID: {}", max_id);

            ids.sort();
            let mut prev_id = i32::MAX;
            for id in ids {
                if id - prev_id > 1 {
                    println!("The missing seat ID is {}", id-1);
                    break;
                }
                prev_id = id;
            }
        },
        Err(e) => {
            println!("Could not read file \"{}\": {}", puzzle_input, e);
        }
    }
    println!("This is day 5.");
}

#[derive(Debug)]
struct PlaneSeat {
    row: i32,
    col: i32,
    id: i32
}

impl PlaneSeat {
    fn new(seat_spec: &str) -> PlaneSeat {
        fn bin_space_partitioning(
            code: &str,
            lower: char,
            upper: char,
            last: i32
        ) -> i32 {
            let mut low = 0;
            let mut high = last;
            for c in code.chars() {
                if c == lower { high = (high + low) / 2; }
                else if c == upper { low = (high + low) / 2 + 1; }
                else {
                    panic!(
                        "Invalid code for lower={}, upper={}: {}",
                        lower, upper, code
                    );
                }
            }
            assert_eq!(low, high);
            low
        }

        assert_eq!(seat_spec.chars().count(), 10);
        let row = bin_space_partitioning(&seat_spec[0..7], 'F', 'B', 127);
        let col = bin_space_partitioning(&seat_spec[7..10], 'L', 'R', 7);
        PlaneSeat{
            row,
            col,
            id: row * 8 + col,
        }
    }
}
