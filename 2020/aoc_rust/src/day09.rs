use std::vec::Vec;

pub fn run_day09(puzzle_input: &str) {
    let numbers: Vec<u64> = std::fs::read_to_string(&puzzle_input)
        .expect(format!("Could not read file \"{}\".", puzzle_input).as_str())
        .split('\n')
        .filter_map(|line| line.parse::<u64>().ok())
        .collect();
    if let Some(non_sum_number) = find_non_sum_number(&numbers, 25) {
        println!(
            "First number that's not a sum as specified: {}",
            non_sum_number
        );
        if let Some(summands) = find_contiguous_summands(
            &numbers, non_sum_number
        ) {
            let min = summands.iter().min().unwrap();
            let max = summands.iter().max().unwrap();
            println!(
                "Found contiguous sequence with minimum {} and maximum {} \
                (in sum: {})",
                min, max,
                min + max
            );
        } else {
            println!("Did not find a contiguous sequence.");
        }
    } else {
        println!("Did not find any non-sum number.");
    }
}

fn find_non_sum_number(
    numbers: &[u64],
    preamble_len: usize
) -> Option<u64> {
    fn find_summands(
        numbers: &[u64],
        i: usize,
        preamble_len: usize
    ) -> Option<(u64, u64)> {
        let num = numbers[i];
        for a in (i-preamble_len)..i {
            for b in (i-preamble_len)..i {
                if a == b { continue; }
                let summand_a = numbers[a];
                let summand_b = numbers[b];
                if summand_a + summand_b == num {
                    return Some((summand_a, summand_b));
                }
            }
        }
        return None
    }
    for i in preamble_len..numbers.len() {
        if let None = find_summands(numbers, i, preamble_len) {
            return Some(numbers[i]);
        }
    }
    return None
}

fn find_contiguous_summands(
    numbers: &[u64],
    desired_sum: u64
) -> Option<Vec<u64>> {
    // O(n^2) solution: Sum up only once.
    let cumsum: Vec<u64> = numbers.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        }).collect();
    for start in 0..numbers.len() {
        let sum_to_start: u64 = cumsum[start];
        for end in (start+1)..=numbers.len() {
            if cumsum[end-1] - sum_to_start == desired_sum {
                return Some(numbers[start..end].to_vec());
            }
        }
    }
    None
}
