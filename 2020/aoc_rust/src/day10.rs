use std::vec::Vec;
use std::collections::{HashMap};

pub fn run_day10(puzzle_input: &str) {
    let mut adapters: Vec<u32> = std::fs::read_to_string(&puzzle_input)
        .expect(format!("Could not read file \"{}\".", puzzle_input).as_str())
        .split('\n')
        .filter_map(|line| line.parse::<u32>().ok())
        .collect();
    adapters.push(0); // also count the outlet
    adapters.sort();
    let built_in_adapter = adapters.last().unwrap() + 3;
    adapters.push(built_in_adapter);
    let mut joltage_diffs: HashMap<u32, i32> = HashMap::new();
    for (prev, adapter) in adapters.iter().zip(&adapters[1..]) {
        let diff = adapter - prev;
        if let Some(counter) = joltage_diffs.get_mut(&diff) {
            *counter += 1;
        } else {
            joltage_diffs.insert(diff, 1);
        }
    }
    println!("Adapters: {:?}", adapters);
    println!("Joltage differences: {:?}", &joltage_diffs);
    println!(
        "1-jolt differences * 3-jolt differences: {}",
        joltage_diffs.get(&1).unwrap_or(&0)
        * joltage_diffs.get(&3).unwrap_or(&0)
    );
    println!(
        "Number of possible arrangements: {}",
        count_arrangements(&adapters, 3)
    );
}

fn adapter_chain_is_valid(adapters: &[u32], tolerance: u32) -> bool {
    if adapters.len() < 2
        || !adapters.iter().is_sorted()
        || adapters.first().unwrap() != &0
    { return false; }
    adapter_subchain_is_valid(&adapters, tolerance)
}

fn adapter_subchain_is_valid(adapters: &[u32], tolerance: u32) -> bool {
    // Not checking if sorted; assuming this has been done for the full
    // chain already.
    if adapters.len() < 2 { return false; }
    for (prev, adapter) in adapters.iter().zip(&adapters[1..]) {
        if adapter - prev > tolerance { return false; }
    }
    true
}

/// Strategy: Split the chain whenever we would not be able to leave out
/// an adapter because the joltage difference is exactly equal to the
/// tolerance, then brute-force only the sub-chains.
fn count_arrangements(adapters: &[u32], tolerance: u32) -> u64 {
    assert!(tolerance > 1);
    assert!(adapter_chain_is_valid(&adapters, tolerance));
    let mut count: u64 = 1;
    let mut prev_subchain_end: usize = 0;
    println!("New chain: {:?}", &adapters);
    for (i, (prev, adapter)) in adapters.iter().zip(
        &adapters[1..]
    ).enumerate() {
        if adapter - prev == tolerance {
            println!("subchain {} to {}", prev_subchain_end, i);
            count *= count_arrangements_bf(
                &adapters[prev_subchain_end..=i+1],
                tolerance,
                1
            );
            prev_subchain_end = i+1;
        }
    }
    count
}

/// Brute-force counting of possible adapter arrangements.
fn count_arrangements_bf(
    adapters: &[u32],
    tolerance: u32,
    start_i: usize
) -> u64 {
    if adapters.len() <= 2 { return 1; }
    if start_i <= 0 || start_i >= adapters.len() - 1 {
        panic!("start_i={}, len={}, {:?}", start_i, adapters.len(), &adapters);
    }
    if adapters.len() - start_i < 2 { return 0; }
    if !adapter_subchain_is_valid(&adapters, tolerance) { return 0; }
    let mut count: u64 = 1; // without leaving out any adapters

    // We may only leave any but the first or the last adapter
    // (socket and device-integrated adapter):
    for i in start_i..adapters.len()-1 {
        let mut adapters_new = adapters.to_vec();
        adapters_new.remove(i);
        if !adapter_subchain_is_valid(&adapters_new, tolerance) { continue; }
        count += count_arrangements_bf(&adapters_new, tolerance, i);
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_validation() {
        let v: Vec<u32> = vec![0, 1, 2, 5];
        assert!(adapter_chain_is_valid(&v, 3));

        let v: Vec<u32> = vec![0, 1];
        assert!(adapter_chain_is_valid(&v, 3));

        let v: Vec<u32> = vec![0];
        assert!(!adapter_chain_is_valid(&v, 3));

        let v: Vec<u32> = vec![3, 4, 5, 8];
        assert!(!adapter_chain_is_valid(&v, 3));
    }

    #[test]
    fn test_count_arrangements_bf() {
        let v: Vec<u32> = vec![0, 1, 2, 5];
        assert_eq!(count_arrangements_bf(&v, 3, 1), 2);

        let v: Vec<u32> = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
        assert_eq!(count_arrangements_bf(&v, 3, 1), 8);

        let v: Vec<u32> = vec![0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18,
            19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46,
            47, 48, 49, 52];
        assert_eq!(count_arrangements_bf(&v, 3, 1), 19208);
    }

    #[test]
    fn test_count_arrangements() {
        let v: Vec<u32> = vec![0, 1, 2, 5];
        assert_eq!(count_arrangements(&v, 3), 2);

        let v: Vec<u32> = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
        assert_eq!(count_arrangements(&v, 3), 8);

        let v: Vec<u32> = vec![0, 1, 2, 3, 4, 7, 8, 9, 10, 11, 14, 17, 18,
            19, 20, 23, 24, 25, 28, 31, 32, 33, 34, 35, 38, 39, 42, 45, 46,
            47, 48, 49, 52];
        assert_eq!(count_arrangements(&v, 3), 19208);
    }

}
