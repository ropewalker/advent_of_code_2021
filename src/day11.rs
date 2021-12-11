use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashSet, VecDeque};

const MAP_SIZE: usize = 10;
const OCTOPUSES_COUNT: usize = MAP_SIZE * MAP_SIZE;
const THRESHOLD_ENERGY: u8 = 9;
const MIN_ENERGY: u8 = 0;

#[derive(Clone)]
struct EnergyLevelsMap([[u8; MAP_SIZE]; MAP_SIZE]);

#[aoc_generator(day11)]
fn parse_input(input: &str) -> EnergyLevelsMap {
    let mut map = [[0; MAP_SIZE]; MAP_SIZE];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map[x][y] = c.to_digit(10).unwrap() as u8;
        }
    }

    EnergyLevelsMap(map)
}

impl EnergyLevelsMap {
    fn flashes_after_step(&mut self) -> usize {
        let mut queue = VecDeque::new();
        let mut flashed = HashSet::new();

        for x in 0..MAP_SIZE {
            for y in 0..MAP_SIZE {
                self.0[x][y] += 1;

                if self.0[x][y] > THRESHOLD_ENERGY {
                    self.0[x][y] = MIN_ENERGY;

                    queue.push_back((x, y));
                    flashed.insert((x, y));
                }
            }
        }

        while !queue.is_empty() {
            let (x, y) = queue.pop_front().unwrap();

            for dx in -1i8..=1 {
                for dy in -1i8..=1 {
                    let (nx, ny) = (x as i8 + dx, y as i8 + dy);

                    if nx >= 0
                        && nx < MAP_SIZE as i8
                        && ny >= 0
                        && ny < MAP_SIZE as i8
                        && !flashed.contains(&(nx as usize, ny as usize))
                    {
                        self.0[nx as usize][ny as usize] += 1;

                        if self.0[nx as usize][ny as usize] > THRESHOLD_ENERGY {
                            self.0[nx as usize][ny as usize] = MIN_ENERGY;

                            queue.push_back((nx as usize, ny as usize));
                            flashed.insert((nx as usize, ny as usize));
                        }
                    }
                }
            }
        }

        flashed.len()
    }
}

#[aoc(day11, part1)]
fn part1(energy_levels: &EnergyLevelsMap) -> usize {
    let mut energy_levels = energy_levels.to_owned();

    (0..100).fold(0, |acc, _| acc + energy_levels.flashes_after_step())
}

#[aoc(day11, part2)]
fn part2(energy_levels: &EnergyLevelsMap) -> usize {
    let mut energy_levels = energy_levels.to_owned();
    let mut step = 1;

    loop {
        if energy_levels.flashes_after_step() == OCTOPUSES_COUNT {
            return step;
        }

        step += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 1_656);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 195);
    }
}
