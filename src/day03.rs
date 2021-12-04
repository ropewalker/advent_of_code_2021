use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering::*;
use BitCriteria::*;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> (Vec<u32>, usize) {
    (
        input
            .lines()
            .map(|l| u32::from_str_radix(l, 2).unwrap())
            .collect(),
        input.lines().next().unwrap().len(),
    )
}

#[aoc(day3, part1)]
fn part1((entries, entry_len): &(Vec<u32>, usize)) -> u32 {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;
    let mut ones_per_bit = vec![0; *entry_len];

    for entry in entries {
        for (index, count) in ones_per_bit.iter_mut().enumerate().take(*entry_len) {
            *count += (entry >> index) & 1;
        }
    }

    for (i, count) in ones_per_bit.iter().enumerate() {
        let pow2 = 1 << i;

        match (count * 2).cmp(&(entries.len() as u32)) {
            Greater => gamma_rate += pow2,
            Less => epsilon_rate += pow2,
            Equal => {
                gamma_rate += pow2;
                epsilon_rate += pow2;
            }
        }
    }

    gamma_rate * epsilon_rate
}

enum BitCriteria {
    MostCommon,
    LeastCommon,
}

fn calculate_rating(sorted_entries: &[u32], entry_len: &usize, bit_criteria: BitCriteria) -> u32 {
    let mut entries = sorted_entries.to_owned();

    let mut rating = 0;
    let mut pow2 = 1 << (*entry_len - 1);

    while entries.len() > 1 {
        let middle = entries[entries.len() / 2];
        let leading_digit = middle / pow2;

        match (leading_digit, &bit_criteria) {
            (1, MostCommon) | (0, LeastCommon) => rating += pow2,
            _ => {}
        };

        entries = entries
            .iter()
            .filter(|&entry| match (leading_digit, &bit_criteria) {
                (0, MostCommon) => *entry < pow2,
                (1, MostCommon) => *entry >= pow2,
                (0, LeastCommon) => *entry >= pow2,
                (1, LeastCommon) => *entry < pow2,
                _ => unreachable!(),
            })
            .map(|entry| entry % pow2)
            .collect();

        pow2 /= 2;
    }

    rating += entries[0];

    rating
}

fn calculate_oxygen_generator_rating(sorted_entries: &[u32], entry_len: &usize) -> u32 {
    calculate_rating(sorted_entries, entry_len, MostCommon)
}

fn calculate_co2_scrubber_rating(sorted_entries: &[u32], entry_len: &usize) -> u32 {
    calculate_rating(sorted_entries, entry_len, LeastCommon)
}

#[aoc(day3, part2)]
fn part2((entries, len): &(Vec<u32>, usize)) -> u32 {
    let mut sorted_entries = entries.to_owned();
    sorted_entries.sort_unstable();

    let oxygen_generator_rating = calculate_oxygen_generator_rating(&sorted_entries, len);
    let co2_scrubber_rating = calculate_co2_scrubber_rating(&sorted_entries, len);

    oxygen_generator_rating * co2_scrubber_rating
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 198);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 230);
    }
}
