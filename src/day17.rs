use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

const PREFIX: &str = "target area: ";

struct TargetArea {
    x_min: i32,
    x_max: i32,
    y_min: i32,
    y_max: i32,
}

#[aoc_generator(day17)]
fn parse_input(input: &str) -> TargetArea {
    let mut split = input[PREFIX.len()..]
        .split(", ")
        .map(|r| r[2..].split("..").map(|c| c.parse().unwrap()))
        .flatten();

    TargetArea {
        x_min: split.next().unwrap(),
        x_max: split.next().unwrap(),
        y_min: split.next().unwrap(),
        y_max: split.next().unwrap(),
    }
}

#[aoc(day17, part1)]
fn part1(target_area: &TargetArea) -> i32 {
    let y_velocity = target_area.y_min.abs() - 1;

    y_velocity * (y_velocity + 1) / 2 //This works only because we are very lucky, and it is possible to find x_velocity that would get the probe inside the target area in the same number of steps.
}

//This assumes that target area lies in the bottom right quadrant.
#[aoc(day17, part2)]
fn part2(target_area: &TargetArea) -> usize {
    let mut steps_to_y_velocities = HashMap::new();

    for y_velocity in target_area.y_min..target_area.y_min.abs() {
        if y_velocity > 0 {
            for step in 1.. {
                let depth = ((y_velocity + step) * (y_velocity + step + 1)
                    - y_velocity * (y_velocity + 1))
                    / 2;

                if depth > target_area.y_min.abs() {
                    break;
                }

                if (target_area.y_max.abs()..=target_area.y_min.abs()).contains(&depth) {
                    let y_velocities = steps_to_y_velocities
                        .entry(y_velocity * 2 + step + 1)
                        .or_insert_with(HashSet::new);

                    y_velocities.insert(y_velocity);
                }
            }
        } else {
            for step in 1.. {
                let depth = ((y_velocity.abs() + step) * (y_velocity.abs() + step - 1)
                    - y_velocity.abs() * (y_velocity.abs() - 1))
                    / 2;

                if depth.abs() > target_area.y_min.abs() {
                    break;
                }

                if (target_area.y_max.abs()..=target_area.y_min.abs()).contains(&depth) {
                    let y_velocities = steps_to_y_velocities
                        .entry(step)
                        .or_insert_with(HashSet::new);

                    y_velocities.insert(y_velocity);
                }
            }
        }
    }

    let max_steps = *steps_to_y_velocities.keys().max().unwrap();
    let mut velocities = HashSet::new();

    for x_velocity in 1..=target_area.x_max {
        let final_x = x_velocity * (x_velocity + 1) / 2;

        if (target_area.x_min..=target_area.x_max).contains(&final_x) {
            for step in x_velocity..=max_steps {
                if let Some(y_velocities) = steps_to_y_velocities.get(&step) {
                    for &y_velocity in y_velocities {
                        velocities.insert((x_velocity, y_velocity));
                    }
                }
            }
        }

        if final_x > target_area.x_min {
            for excessive_steps in 1..x_velocity {
                let x_target =
                    (x_velocity * (x_velocity + 1) - excessive_steps * (excessive_steps + 1)) / 2;

                if x_target < target_area.x_min {
                    break;
                }

                if (target_area.x_min..=target_area.x_max).contains(&x_target) {
                    if let Some(y_velocities) =
                        steps_to_y_velocities.get(&(x_velocity - excessive_steps))
                    {
                        for &y_velocity in y_velocities {
                            velocities.insert((x_velocity, y_velocity));
                        }
                    }
                }
            }
        }
    }

    velocities.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"target area: x=20..30, y=-10..-5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 45);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 112);
    }
}
