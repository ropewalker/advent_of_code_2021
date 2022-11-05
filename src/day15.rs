use aoc_runner_derive::{aoc, aoc_generator};
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

type Point = Vec2;

#[derive(Debug)]
struct RiskLevelMap {
    map: HashMap<Point, usize>,
    width: usize,
    height: usize,
}

impl From<&str> for RiskLevelMap {
    fn from(map_str: &str) -> Self {
        RiskLevelMap {
            map: map_str
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars().enumerate().map(move |(x, c)| {
                        (
                            Point {
                                x: x as i32,
                                y: y as i32,
                            },
                            c.to_digit(10).unwrap() as usize,
                        )
                    })
                })
                .collect(),
            width: map_str.lines().next().unwrap().chars().count(),
            height: map_str.lines().count(),
        }
    }
}

impl RiskLevelMap {
    fn neighbor_risks(&self, point: &Point, large_map_multiplier: usize) -> Vec<(Point, usize)> {
        [(-1, 0), (1, 0), (0, -1), (0, 1)]
            .into_iter()
            .map(|(x, y)| Point {
                x: point.x + x,
                y: point.y + y,
            })
            .filter(|p| {
                p.x >= 0
                    && p.y >= 0
                    && p.x < (self.width * large_map_multiplier) as i32
                    && p.y < (self.height * large_map_multiplier) as i32
            })
            .map(|p| {
                (
                    p,
                    (self
                        .map
                        .get(&Point {
                            x: p.x % self.width as i32,
                            y: p.y % self.height as i32,
                        })
                        .unwrap()
                        + p.x as usize / self.width
                        + p.y as usize / self.height
                        - 1)
                        % 9
                        + 1,
                )
            })
            .collect()
    }

    fn dijkstra_diagonal(&self, large_map_multiplier: usize) -> Option<usize> {
        const TOP_LEFT: Point = Point { x: 0, y: 0 };

        let mut dist: HashMap<Point, usize> = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(TOP_LEFT, 0);

        heap.push(State {
            cost: 0,
            position: TOP_LEFT,
        });

        while let Some(State { cost, position }) = heap.pop() {
            if position.x == (self.width * large_map_multiplier - 1) as i32
                && position.y == (self.height * large_map_multiplier - 1) as i32
            {
                return Some(cost);
            }

            if let Some(old_cost) = dist.get(&position) {
                if cost > *old_cost {
                    continue;
                }
            }

            for (point, risk) in self.neighbor_risks(&position, large_map_multiplier) {
                let next = State {
                    cost: cost + risk,
                    position: point,
                };

                if next.cost < *dist.get(&next.position).unwrap_or(&(next.cost + 1)) {
                    heap.push(next);
                    dist.insert(next.position, next.cost);
                }
            }
        }

        None
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Point,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| {
                (self.position.y + self.position.x).cmp(&(other.position.x + other.position.y))
            })
            .then_with(|| {
                (other.position.y - other.position.x)
                    .abs()
                    .cmp(&(self.position.x - self.position.y).abs())
            })
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day15)]
fn parse_input(input: &str) -> RiskLevelMap {
    input.into()
}

#[aoc(day15, part1)]
fn part1(risk_map: &RiskLevelMap) -> Option<usize> {
    risk_map.dijkstra_diagonal(1)
}

#[aoc(day15, part2)]
fn part2(risk_map: &RiskLevelMap) -> Option<usize> {
    risk_map.dijkstra_diagonal(5)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), Some(40));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), Some(315));
    }
}
