use aoc_runner_derive::{aoc, aoc_generator};

struct Coordinates {
    horizontal_position: u32,
    depth: u32,
}

struct AimedCoordinates {
    horizontal_position: u32,
    depth: u32,
    aim: u32,
}

enum Direction {
    Forward,
    Down,
    Up,
}

impl From<&str> for Direction {
    fn from(direction_str: &str) -> Self {
        match direction_str {
            "forward" => Self::Forward,
            "down" => Self::Down,
            "up" => Self::Up,
            _ => unreachable!(),
        }
    }
}

struct Command {
    direction: Direction,
    units: u32,
}

impl From<&str> for Command {
    fn from(command_str: &str) -> Self {
        let mut iter = command_str.split_whitespace();

        let direction: Direction = iter.next().unwrap().into();
        let units = iter.next().unwrap().parse::<u32>().unwrap();

        Self { direction, units }
    }
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Command> {
    input.lines().map(|l| l.into()).collect()
}

#[aoc(day2, part1)]
fn part1(commands: &[Command]) -> u32 {
    use Direction::*;

    let mut coordinates = Coordinates {
        horizontal_position: 0,
        depth: 0,
    };

    for command in commands {
        match command.direction {
            Forward => coordinates.horizontal_position += command.units,
            Down => coordinates.depth += command.units,
            Up => coordinates.depth -= command.units,
        }
    }

    coordinates.horizontal_position * coordinates.depth
}

#[aoc(day2, part2)]
fn part2(commands: &[Command]) -> u32 {
    use Direction::*;

    let mut coordinates = AimedCoordinates {
        horizontal_position: 0,
        depth: 0,
        aim: 0,
    };

    for command in commands {
        match command.direction {
            Forward => {
                coordinates.horizontal_position += command.units;
                coordinates.depth += coordinates.aim * command.units;
            }
            Down => coordinates.aim += command.units,
            Up => coordinates.aim -= command.units,
        }
    }

    coordinates.horizontal_position * coordinates.depth
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 150);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 900);
    }
}
