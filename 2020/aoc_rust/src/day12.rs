use std::vec::Vec;

pub fn run_day12(puzzle_input: &str) {
    let nav_instrs: Vec<(String, u32)> = std::fs::read_to_string(&puzzle_input)
        .expect(format!("Could not read file \"{}\".", puzzle_input).as_str())
        .split('\n').filter_map(|line| {
            if line.len() <= 1 { return None; }
            Some((
                String::from(&line[0..1]),
                line[1..].parse::<u32>().expect(
                    format!("Could not parse \"{}\"", line).as_str()
                )
            ))
        }).collect();
    let dest = follow_ship_only_instructions(&nav_instrs);
    println!(
        "Final position from ship instructions: ({}, {}), \
        Manhattan distance: {}",
        dest.0, dest.1, dest.0.abs() + dest.1.abs()
    );

    let (dest, _) = follow_waypoint_instructions(&nav_instrs, (10, 1));
    println!(
        "Final position from waypoint instructions: ({}, {}), \
        Manhattan distance: {}",
        dest.0, dest.1, dest.0.abs() + dest.1.abs()
    );
}

fn rotate_90_cw(ori: (i64, i64), angle: i64) -> (i64, i64) {
    // In Rust, % is the remainder, not modulo:
    let angle = ((angle % 360) + 360) % 360;
    match angle {
        0 => { ori },
        90 => { (ori.1, -ori.0) },
        180 => { (-ori.0, -ori.1) },
        270 => { (-ori.1, ori.0) },
        e => { panic!("Rotating by {}° not implemented", e); }
    }
}

fn _rotate_90_around_cw(
    pivot: (i64, i64),
    pt: (i64, i64),
    angle: i64,
) -> (i64, i64) {
    let shifted = (pt.0 - pivot.0, pt.1 - pivot.1);
    let rotated = rotate_90_cw(shifted, angle);
    (rotated.0 + pivot.0, rotated.1 + pivot.1)
}

fn follow_ship_only_instructions(
        instrs: &[(String, u32)],
) -> (i64, i64) {
    let mut pos: (i64, i64) = (0, 0);
    let mut ori: (i64, i64) = (1, 0);
    for (action, val) in instrs {
        let val: i64 = *val as i64;
        match action.as_str() {
            "N" => { pos = (pos.0, pos.1 + val); },
            "S" => { pos = (pos.0, pos.1 - val); },
            "E" => { pos = (pos.0 + val, pos.1); },
            "W" => { pos = (pos.0 - val, pos.1); },
            "L" => { ori = rotate_90_cw(ori, -val); },
            "R" => { ori = rotate_90_cw(ori, val); },
            "F" => {
                pos = (pos.0 + ori.0 * val,
                       pos.1 + ori.1 * val);
            },
            e => { panic!("Unrecognized action {}", e); }
        }
    }
    pos
}

fn follow_waypoint_instructions(
        instrs: &[(String, u32)],
        waypoint_start: (i64, i64),
) -> ((i64, i64), (i64, i64)) {
    let mut ship_pos: (i64, i64) = (0, 0);
    let mut waypoint: (i64, i64) = waypoint_start;
    for (action, val) in instrs {
        let val: i64 = *val as i64;
        match action.as_str() {
            "N" => { waypoint = (waypoint.0, waypoint.1 + val); }
            "S" => { waypoint = (waypoint.0, waypoint.1 - val); },
            "E" => { waypoint = (waypoint.0 + val, waypoint.1); },
            "W" => { waypoint = (waypoint.0 - val, waypoint.1); },
            "L" => {
                waypoint = rotate_90_cw(waypoint, -val);
            },
            "R" => {
                waypoint = rotate_90_cw(waypoint, val);
            },
            "F" => {
                ship_pos = (
                    ship_pos.0 + val * waypoint.0,
                    ship_pos.1 + val * waypoint.1
                );
                // waypoint = (
                //      waypoint.0 + val * waypoint.0,
                //      waypoint.1 + val * waypoint.1
                // );
            },
            e => { panic!("Unrecognized action{}", e); }
        }
        println!("pos and waypoint after {}{}: {:?}", action, val, (&ship_pos, &waypoint));
    }
    (ship_pos, waypoint)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waypoint_instr() {
        let instrs: Vec<(String, u32)> = vec![
            (String::from("F"), 10),
            (String::from("N"), 3),
            (String::from("F"), 7),
            (String::from("R"), 90),
            (String::from("F"), 11),
        ];
        let (dest, wp) = follow_waypoint_instructions(&instrs, (10, 1));
        println!("dest: {:?}, wp: {:?}", &dest, &wp);
        assert!(dest == (214, -72));
        assert!(wp == (4, -10));
    }
}
