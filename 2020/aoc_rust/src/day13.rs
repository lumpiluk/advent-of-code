extern crate num;

pub fn run_day13(puzzle_input: &str) {
    let lines: Vec<String> = std::fs::read_to_string(&puzzle_input)
        .expect(
            format!("Could not read file \"{}\".", puzzle_input).as_str()
        )
        .split('\n')
        .map(|l| String::from(l))
        .collect();
    if lines.len() < 3 {
        panic!("Incomplete notes, only {} lines", lines.len());
    }
    let earliest_departure: u64 = lines[0].parse::<u64>()
        .expect(format!(
            "Could not parse earliest departure \"{}\"",
            lines[0]
        ).as_str());
    let bus_ids: Vec<Option<u64>> = lines[1].split(',')
        .map(|s| s.parse::<u64>().ok())
        .collect();
    let bus_ids_filtered: Vec<u64> = bus_ids.iter()
        .filter_map(|id| *id).collect();
    let earliest_bus_id = bus_ids_filtered.iter()
        .min_by(|a, b| {
            let wait_a: u64 = *a - &earliest_departure % *a;
            let wait_b: u64 = *b - &earliest_departure % *b;
            wait_a.cmp(&wait_b)
        })
        .unwrap();
    let wait_time = earliest_bus_id - earliest_departure % earliest_bus_id;
    println!(
        "Earliest bus after {}: {}, waiting time: {}, prod: {}",
        earliest_departure,
        earliest_bus_id,
        wait_time,
        earliest_bus_id * wait_time
    );

    let mut contest_timestamp: u64 = 0;
    let mut max_bus_tab_id: usize = 0;
    let mut reference_bus = bus_ids_filtered[0];
    loop {
        let mut seq_correct = true;
        for (i, bus_id) in bus_ids[1..].iter().enumerate() {
            if let Some(bus_id) = bus_id {
                if (contest_timestamp + i as u64 + 1) % bus_id == 0 {
                    if i + 1 > max_bus_tab_id {
                        max_bus_tab_id = i + 1;
                        // Update the periodicity to the least
                        // common multiple:
                        reference_bus = bus_ids[..=i+1].iter()
                            .fold(1, |lcm, &bus_id| {
                                if let Some(bus_id) = bus_id {
                                    num::integer::lcm(lcm, bus_id)
                                } else {
                                    lcm
                                }
                            });
                    }
                } else {
                    seq_correct = false;
                    break;
                }
            }
        }
        // println!("{}", &contest_timestamp);
        if seq_correct { break; }
        contest_timestamp += reference_bus;
    }
    println!("Contest timestamp: {}", contest_timestamp);
}
