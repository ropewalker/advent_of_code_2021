use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

type Point = Vec2;

struct DepthMap(HashMap<Point, i32>);

impl DepthMap {
    fn neighbor_heights(&self, point: &Point) -> Vec<(Point, i32)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(x, y)| Point {
                x: point.x + x,
                y: point.y + y,
            })
            .filter_map(|p| self.0.get_key_value(&p))
            .map(|(p, h)| (p.to_owned(), h.to_owned()))
            .collect()
    }

    fn is_low_point(&self, point: &Point) -> bool {
        if let Some(height) = self.0.get(point) {
            return self
                .neighbor_heights(point)
                .iter()
                .all(|(_, neighbor_height)| *neighbor_height > *height);
        }

        false
    }

    fn risk_level(&self, point: &Point) -> i32 {
        if self.is_low_point(point) {
            self.0.get(point).unwrap() + 1
        } else {
            0
        }
    }

    fn basin_size(&self, point: &Point) -> usize {
        if let Some(height) = self.0.get(point) {
            if *height == 9 {
                return 0;
            }

            let mut queue = VecDeque::from(vec![(point.to_owned(), *height)]);
            let mut visited: HashSet<Point> = [point.to_owned()].iter().cloned().collect();

            while !queue.is_empty() {
                let (current_point, current_height) = queue.pop_front().unwrap();

                for (neighbor_point, neighbor_height) in self.neighbor_heights(&current_point) {
                    if neighbor_height > current_height
                        && neighbor_height != 9
                        && !visited.contains(&neighbor_point)
                    {
                        queue.push_back((neighbor_point.to_owned(), neighbor_height));
                        visited.insert(neighbor_point);
                    };
                }
            }

            return visited.len();
        }

        0
    }
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> DepthMap {
    DepthMap(
        input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars().enumerate().map(move |(x, c)| {
                    (
                        Point {
                            x: x as i32,
                            y: y as i32,
                        },
                        c.to_digit(10).unwrap() as i32,
                    )
                })
            })
            .flatten()
            .collect(),
    )
}

#[aoc(day9, part1)]
fn part1(map: &DepthMap) -> i32 {
    map.0.keys().map(|point| map.risk_level(point)).sum()
}

#[aoc(day9, part2)]
fn part2(map: &DepthMap) -> usize {
    let mut basin_sizes = map
        .0
        .keys()
        .filter(|point| map.is_low_point(point))
        .map(|point| map.basin_size(point))
        .collect::<Vec<_>>();

    basin_sizes.sort_unstable();

    basin_sizes.into_iter().rev().take(3).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 15);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 1134);
    }
}
