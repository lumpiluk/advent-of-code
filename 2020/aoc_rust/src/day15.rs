use std::vec::Vec;
use std::collections::HashMap;

pub fn run_day15(puzzle_input: &str) {
    let starting_nums: Vec<u32> = std::fs::read_to_string(&puzzle_input)
        .expect(format!("Could not read file \"{}\".", puzzle_input).as_str())
        .trim().split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect();

    let last_num_spoken = play_game(&starting_nums, 2020);
    println!("Num spoken in turn 2020: {}", last_num_spoken);

    let last_num_spoken = play_game(&starting_nums, 30000000);
    println!("Num spoken in turn 30000000: {}", last_num_spoken);

}

fn play_game(starting_nums: &[u32], end_turn: u32) -> u32 {
    let mut nums_spoken_turns: HashMap<u32, u32> = HashMap::new();
    let mut num_spoken: u32 = 0;
    for turn in 1..=end_turn {
        if turn as usize <= starting_nums.len() {
            num_spoken = starting_nums[turn as usize - 1];
            nums_spoken_turns.insert(num_spoken, turn);
            continue;
        }

        let prev_num_spoken = num_spoken;
        if let Some(last_turn_spoken) = nums_spoken_turns
                .get_mut(&num_spoken) {
            // println!("{} was last spoken in turn {}", &num_spoken, &last_turn_spoken);
            num_spoken = turn - 1 - *last_turn_spoken;
        } else {
            // println!("{} was not spoken yet", &num_spoken);
            num_spoken = 0;
        }
        // println!("{}: {}", &turn, &num_spoken);
        nums_spoken_turns.insert(prev_num_spoken, turn-1);
    }
    num_spoken
}
