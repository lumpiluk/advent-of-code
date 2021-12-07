pub fn run_day02(puzzle_input: &str) {
    let input: String = String::from(
        std::fs::read_to_string(&puzzle_input)
            .expect(format!(
                "Could not read file \"{}\".",
                &puzzle_input
            ).as_str())
            .trim()
    );

    let mut depth: i64 = 0;
    let mut pos_x: i64 = 0;
    for line in input.split('\n') {
        let cmd = line.split_whitespace()
            .next()
            .unwrap_or("");
        let val: i64 = line.split_whitespace()
            .nth(1)
            .unwrap_or("")
            .parse().unwrap();
        // println!("cmd={}, val={}", &cmd, &val);
        match cmd {
            "forward" => { pos_x += val; },
            "down" => { depth += val; },
            "up" => { depth -= val; },
            _ => { panic!("Command {} unknown", &cmd); }
        }
    }
    println!(
        "Part I: depth = {}, horizontal position = {}, product = {}",
        depth,
        pos_x,
        depth * pos_x
    );

    let mut aim: i64 = 0;
    let mut depth: i64 = 0;
    let mut pos_x: i64 = 0;
    for line in input.split('\n') {
        let cmd = line.split_whitespace()
            .next()
            .unwrap_or("");
        let val: i64 = line.split_whitespace()
            .nth(1)
            .unwrap_or("")
            .parse().unwrap();
        match cmd {
            "forward" => { pos_x += val; depth += aim * val; },
            "down" => { aim += val; },
            "up" => { aim -= val; },
            _ => { panic!("Command {} unknown", &cmd); }
        }
    }
    println!(
        "Parth II: depth = {}, horizontal position = {}, product = {}",
        depth,
        pos_x,
        depth * pos_x
    );

}
