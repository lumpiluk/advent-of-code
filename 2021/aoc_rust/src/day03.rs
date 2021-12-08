use std::path::Path;

pub fn run_day03<P: AsRef<Path>>(puzzle_input: P) {
    let input: String = String::from(
        std::fs::read_to_string(&puzzle_input)
            .expect(format!(
                "Could not read file \"{}\".",
                puzzle_input.as_ref().display()
            ).as_str())
            .trim()
    );

    fn get_most_and_least_frequent_bits(input: &str, prefix: &str) -> (String, String, usize, String) {
        let mut freqs_zeros: Vec<u64> = Vec::new();
        let mut freqs_ones: Vec<u64> = Vec::new();
        let mut count: usize = 0;
        let mut most_recent_match: &str = "";
        for line in input.split('\n') {
            if !line.starts_with(prefix) { continue; }
            count += 1;
            most_recent_match = line;
            for (i, c) in line.trim().chars().enumerate() {
                if freqs_zeros.len() < i + 1 { freqs_zeros.push(0); }
                if freqs_ones.len() < i + 1 { freqs_ones.push(0); }
                match c {
                    '0' => { freqs_zeros[i] += 1; },
                    '1' => { freqs_ones[i] += 1; },
                    _ => { panic!{"Unrecognized character '{}'", c}; }
                }
            }
        }
        let mut most_frequent_bits: String = String::new();
        let mut least_frequent_bits: String = String::new();
        for (freq_zeros, freq_ones) in freqs_zeros.iter().zip(&freqs_ones) {
            if freq_zeros > freq_ones {
                most_frequent_bits += "0";
                least_frequent_bits += "1";
            } else if freq_zeros < freq_ones {
                most_frequent_bits += "1";
                least_frequent_bits += "0";
            } else {
                // In case of equal frequencies (relevant for part II)
                most_frequent_bits += "1";
                least_frequent_bits += "0";
            }
        }
        (most_frequent_bits, least_frequent_bits, count, most_recent_match.to_string())
    }

    let (gamma_rate_bin, epsilon_rate_bin, _, _) = get_most_and_least_frequent_bits(&input, "");
    println!("gamma: {}, epsilon: {}", &gamma_rate_bin, &epsilon_rate_bin);
    let gamma_rate = usize::from_str_radix(&gamma_rate_bin, 2).unwrap();
    let epsilon_rate = usize::from_str_radix(&epsilon_rate_bin, 2).unwrap();
    println!("gamma: {}, epsilon: {}, product: {}", &gamma_rate, &epsilon_rate, gamma_rate * epsilon_rate);

    fn get_o2_rating(input: &str, bit_index: usize, most_freq: &str) -> usize {
        let (most_freq_new, _, count, most_recent_match) = get_most_and_least_frequent_bits(
            &input,
            &most_freq[0..bit_index]
        );
        println!("o2 bit index {}, most_freq {}, count {}", &bit_index, &most_freq, count);
        if count == 1 {
            return usize::from_str_radix(&most_recent_match, 2).unwrap();
        } else if count < 1 {
            panic!("No matches for o2 rating, bit index {}, most_freq {}", bit_index, most_freq);
        }
        get_o2_rating(
            &input,
            bit_index+1,
            &(most_freq[..bit_index].to_string() + &most_freq_new[bit_index..])
        )
    }
    fn get_co2_rating(input: &str, bit_index: usize, least_freq: &str) -> usize {
        let (_, least_freq_new, count, most_recent_match) = get_most_and_least_frequent_bits(
            &input,
            &least_freq[0..bit_index]
        );
        println!("co2 bit index {}, least_freq {}, count {}", &bit_index, &least_freq, count);
        if count == 1 {
            return usize::from_str_radix(&most_recent_match, 2).unwrap();
        } else if count < 1 {
            panic!("No matches for co2 rating, bit index {}, least_freq {}", bit_index, least_freq);
        }
        get_co2_rating(
            &input,
            bit_index+1,
            &(least_freq[..bit_index].to_string() + &least_freq_new[bit_index..])
        )
    }

    let o2_generator_rating = get_o2_rating(&input, 0, "");
    let co2_scrubber_rating = get_co2_rating(&input, 0, "");
    println!(
        "o2 generator rating: {}, co2 scrubber rating: {}, product: {}",
        o2_generator_rating,
        co2_scrubber_rating,
        o2_generator_rating * co2_scrubber_rating
    );
}
