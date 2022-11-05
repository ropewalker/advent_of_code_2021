use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

type Entry = (Vec<HashSet<char>>, Vec<HashSet<char>>);

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Entry> {
    input
        .lines()
        .map(|l| {
            let mut iter = l.split('|');
            (
                iter.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.chars().collect())
                    .collect::<Vec<_>>(),
                iter.next()
                    .unwrap()
                    .split_whitespace()
                    .map(|s| s.chars().collect())
                    .collect::<Vec<_>>(),
            )
        })
        .collect()
}

#[aoc(day8, part1)]
fn part1(entries: &[Entry]) -> usize {
    entries
        .iter()
        .flat_map(|(_, four_digit_output_value)| four_digit_output_value)
        .filter(|&digit| {
            digit.len() == 2 || digit.len() == 3 || digit.len() == 4 || digit.len() == 7
        })
        .count()
}

#[aoc(day8, part2)]
fn part2(entries: &[Entry]) -> u32 {
    let mut result = 0;

    for entry in entries {
        let mut unique_signal_patterns = entry.0.to_owned();
        let four_digit_output_value = entry.1.to_owned();

        let mut patterns = vec![None; 10];

        let i = unique_signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| pattern.len() == 2)
            .unwrap()
            .0;

        patterns[1] = Some(unique_signal_patterns[i].to_owned());
        unique_signal_patterns.swap_remove(i);

        let i = unique_signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| pattern.len() == 4)
            .unwrap()
            .0;

        patterns[4] = Some(unique_signal_patterns[i].to_owned());
        unique_signal_patterns.swap_remove(i);

        let i = unique_signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| pattern.len() == 3)
            .unwrap()
            .0;

        patterns[7] = Some(unique_signal_patterns[i].to_owned());
        unique_signal_patterns.swap_remove(i);

        let i = unique_signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| pattern.len() == 7)
            .unwrap()
            .0;

        patterns[8] = Some(unique_signal_patterns[i].to_owned());
        unique_signal_patterns.swap_remove(i);

        let i = unique_signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| {
                pattern.len() == 6
                    && patterns[4]
                        .as_ref()
                        .unwrap()
                        .difference(pattern)
                        .next()
                        .is_none()
            })
            .unwrap()
            .0;

        patterns[9] = Some(unique_signal_patterns[i].to_owned());
        unique_signal_patterns.swap_remove(i);

        let i = unique_signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| {
                pattern.len() == 6
                    && patterns[7]
                        .as_ref()
                        .unwrap()
                        .difference(pattern)
                        .next()
                        .is_none()
            })
            .unwrap()
            .0;

        patterns[0] = Some(unique_signal_patterns[i].to_owned());
        unique_signal_patterns.swap_remove(i);

        let i = unique_signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| pattern.len() == 6)
            .unwrap()
            .0;

        patterns[6] = Some(unique_signal_patterns[i].to_owned());
        unique_signal_patterns.swap_remove(i);

        let i = unique_signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| {
                pattern.len() == 5
                    && pattern
                        .difference(patterns[6].as_ref().unwrap())
                        .next()
                        .is_none()
            })
            .unwrap()
            .0;

        patterns[5] = Some(unique_signal_patterns[i].to_owned());
        unique_signal_patterns.swap_remove(i);

        let i = unique_signal_patterns
            .iter()
            .enumerate()
            .find(|(_, pattern)| {
                pattern.len() == 5
                    && pattern
                        .difference(patterns[9].as_ref().unwrap())
                        .next()
                        .is_none()
            })
            .unwrap()
            .0;

        patterns[3] = Some(unique_signal_patterns[i].to_owned());
        unique_signal_patterns.swap_remove(i);

        patterns[2] = unique_signal_patterns.into_iter().next();

        result += four_digit_output_value.iter().fold(0, |acc, digit| {
            acc * 10
                + patterns
                    .iter()
                    .enumerate()
                    .find(|(_, pattern)| pattern.as_ref().unwrap() == digit)
                    .unwrap()
                    .0 as u32
        });
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 26);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 61_229);
    }
}
