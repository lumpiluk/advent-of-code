pub fn run_day01(puzzle_input: &str) {
    let input: String = String::from(
        std::fs::read_to_string(&puzzle_input)
            .expect(format!(
                "Could not read file \"{}\".",
                &puzzle_input
            ).as_str())
            .trim()
    );

    let depths: Vec<u64> = input.split('\n')
        .map(|s| s.parse::<u64>().unwrap())
        .collect();

    let mut n_increased: u64 = 0;
    let mut prev_depth = u64::MAX;
    for depth in &depths {
        if depth > &prev_depth { n_increased += 1; }
        prev_depth = depth.clone();
    }
    println!(
        "{} measurements are larger than the previous measurement.",
        n_increased
    );

    let mut prev_window: u64 = depths[0..3].iter().sum();
    let mut n_sums_increased: u64 = 0;
    for i in 1 .. depths.len() - 2 {
        let current_window: u64 = depths[i..i+3].iter().sum();
        if current_window > prev_window { n_sums_increased += 1; }
        prev_window = current_window;
    }
    println!(
        "{} sums are larger than the previous measurement.",
        n_sums_increased
    );
}
