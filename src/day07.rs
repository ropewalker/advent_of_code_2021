use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::Sub;

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<i32> {
    input.split(',').map(|c| c.parse().unwrap()).collect()
}

#[aoc(day7, part1, median)]
fn part1_median(positions: &[i32]) -> i32 {
    let mut positions = positions.to_owned();
    positions.sort_unstable();

    let median = positions[positions.len() / 2];

    positions
        .into_iter()
        .map(|pos| pos.sub(&median).abs())
        .sum()
}

#[aoc(day7, part1)]
fn part1(positions: &[i32]) -> i32 {
    min_cost(positions, manhattan_cost)
}

fn manhattan_cost(positions: &[i32], target: i32) -> i32 {
    positions.iter().map(|pos| pos.sub(&target).abs()).sum()
}

fn geometric_cost(positions: &[i32], target: i32) -> i32 {
    positions
        .iter()
        .map(|pos| {
            let distance = (*pos - target).abs();
            distance * (distance + 1) / 2
        })
        .sum()
}

fn min_cost(positions: &[i32], cost: impl Fn(&[i32], i32) -> i32) -> i32 {
    let mut low = positions.iter().min().unwrap().to_owned();
    let mut high = positions.iter().max().unwrap().to_owned();

    loop {
        let middle = (low + high) / 2;

        let left = middle - 1;
        let right = middle + 1;

        let cost_to_middle = cost(positions, middle);

        if cost(positions, left) < cost_to_middle {
            high = left;
        } else if cost(positions, right) < cost_to_middle {
            low = right;
        } else {
            return cost_to_middle;
        }
    }
}

#[aoc(day7, part2)]
fn part2(positions: &[i32]) -> i32 {
    min_cost(positions, geometric_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 37);
    }

    #[test]
    fn part1_median_example() {
        assert_eq!(part1_median(&parse_input(TEST_INPUT)), 37);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 168);
    }
}
