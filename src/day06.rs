use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Vec<u8> {
    input.split(',').map(|c| c.parse().unwrap()).collect()
}

fn count_fish_by_day(fish_cycles: &[u8]) -> [u64; 9] {
    let mut fish_by_day = [0u64; 9];

    for day in fish_cycles {
        fish_by_day[*day as usize] += 1;
    }

    fish_by_day
}

fn pass_day(fish_by_day: &mut [u64; 9]) {
    let spawned = fish_by_day[0];

    for i in 1..=8 {
        fish_by_day[i - 1] = fish_by_day[i];
    }

    fish_by_day[6] += spawned;
    fish_by_day[8] = spawned;
}

fn pass_x_days(fish_by_day: &mut [u64; 9], days: usize) {
    for _ in 1..=days {
        pass_day(fish_by_day);
    }
}

#[aoc(day6, part1)]
fn part1(initial_cycles: &[u8]) -> u64 {
    let mut fish_by_day = count_fish_by_day(initial_cycles);
    pass_x_days(&mut fish_by_day, 80);
    fish_by_day.iter().sum()
}

#[aoc(day6, part2)]
fn part2(initial_cycles: &[u8]) -> u64 {
    let mut fish_by_day = count_fish_by_day(initial_cycles);
    pass_x_days(&mut fish_by_day, 256);
    fish_by_day.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"3,4,3,1,2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 5_934);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 26_984_457_539);
    }
}
