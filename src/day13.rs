use crate::day13::Instruction::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;
use std::hash::Hash;

const INSTRUCTION_PREFIX: &str = "fold along ";

enum Instruction {
    FoldAlongX(i32),
    FoldAlongY(i32),
}

impl From<&str> for Instruction {
    fn from(instruction: &str) -> Self {
        let value = instruction[INSTRUCTION_PREFIX.len() + 2..].parse().unwrap();

        match instruction.chars().nth(INSTRUCTION_PREFIX.len()).unwrap() {
            'x' => FoldAlongX(value),
            'y' => FoldAlongY(value),
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Vec2 {
    x: i32,
    y: i32,
}

type Point = Vec2;

impl From<&str> for Vec2 {
    fn from(vec2: &str) -> Self {
        let mut iter = vec2.split(',');

        Vec2 {
            x: iter.next().unwrap().parse().unwrap(),
            y: iter.next().unwrap().parse().unwrap(),
        }
    }
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> (HashSet<Point>, Vec<Instruction>) {
    let mut split = input.split("\n\n");

    (
        split.next().unwrap().lines().map(|l| l.into()).collect(),
        split.next().unwrap().lines().map(|l| l.into()).collect(),
    )
}

fn fold_along_axis(points: &HashSet<Point>, instruction: &Instruction) -> HashSet<Point> {
    points
        .iter()
        .map(|point| match instruction {
            FoldAlongX(x) => Point {
                x: x - (point.x - x).abs(),
                y: point.y,
            },
            FoldAlongY(y) => Point {
                x: point.x,
                y: y - (point.y - y).abs(),
            },
        })
        .collect::<HashSet<_>>()
}

#[aoc(day13, part1)]
fn part1((points, instructions): &(HashSet<Point>, Vec<Instruction>)) -> usize {
    fold_along_axis(points, &instructions[0]).len()
}

#[aoc(day13, part2)]
fn part2((points, instructions): &(HashSet<Point>, Vec<Instruction>)) -> String {
    let points = instructions
        .iter()
        .fold(points.to_owned(), |points, instruction| {
            fold_along_axis(&points, instruction)
        });

    let mut result = String::from("");

    for y in 0..=points.iter().map(|point| point.y).max().unwrap() {
        result.push('\n');
        for x in 0..=points.iter().map(|point| point.x).max().unwrap() {
            if points.contains(&Point { x, y }) {
                result.push('#');
            } else {
                result.push('.');
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 17);
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse_input(TEST_INPUT)),
            r"
#####
#...#
#...#
#...#
#####"
        );
    }
}
