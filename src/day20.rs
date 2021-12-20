use crate::day20::Pixel::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::ops::Add;

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Pixel {
    Light,
    Dark,
}

impl From<char> for Pixel {
    fn from(c: char) -> Self {
        match c {
            '#' => Light,
            '.' => Dark,
            _ => unreachable!(),
        }
    }
}

impl From<Pixel> for usize {
    fn from(pixel: Pixel) -> Self {
        match pixel {
            Light => 1,
            Dark => 0,
        }
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

type Position = Vec2;

impl From<(i32, i32)> for Vec2 {
    fn from((x, y): (i32, i32)) -> Self {
        Self { x, y }
    }
}

impl From<Vec2> for (i32, i32) {
    fn from(vec2: Vec2) -> Self {
        (vec2.x, vec2.y)
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Image {
    pixels: HashMap<Position, Pixel>,
    top_left: Position,
    bottom_right: Position,
}

impl From<&str> for Image {
    fn from(image_str: &str) -> Self {
        Image {
            pixels: image_str
                .lines()
                .enumerate()
                .map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .map(move |(x, c)| ((x as i32, y as i32).into(), c.into()))
                })
                .flatten()
                .collect(),
            top_left: (0, 0).into(),
            bottom_right: (
                image_str.lines().next().unwrap().len() as i32,
                image_str.lines().count() as i32,
            )
                .into(),
        }
    }
}

#[derive(Clone, Debug)]
struct Processor {
    image_enhancement_algorithm: [Pixel; 512],
    input_image: Image,
    outside_pixels_kind: Pixel,
}

impl From<&str> for Processor {
    fn from(processor_str: &str) -> Self {
        let mut input_split = processor_str.split("\n\n");
        let image_enhancement_algorithm = input_split
            .next()
            .unwrap()
            .chars()
            .filter(|c| *c == '#' || *c == '.')
            .map(|c| c.into())
            .collect::<Vec<Pixel>>();

        Processor {
            image_enhancement_algorithm: image_enhancement_algorithm.try_into().unwrap(),
            input_image: input_split.next().unwrap().into(),
            outside_pixels_kind: Dark,
        }
    }
}

const DARK_FRAGMENT_BINARY: usize = 0b000000000;
const LIGHT_FRAGMENT_BINARY: usize = 0b111111111;

impl Processor {
    fn apply_algorithm_to_pixel(&self, pixel_position: Position) -> Pixel {
        *self
            .image_enhancement_algorithm
            .get(self.pixel_to_binary(&pixel_position))
            .unwrap()
    }

    fn apply_algorithm(&mut self) {
        let mut new_image = Image {
            pixels: self.input_image.pixels.to_owned(),
            top_left: self.input_image.top_left + (-1, -1).into(),
            bottom_right: self.input_image.bottom_right + (1, 1).into(),
        };

        let new_outside_pixels_kind = match self.outside_pixels_kind {
            Light => self.image_enhancement_algorithm[LIGHT_FRAGMENT_BINARY],
            Dark => self.image_enhancement_algorithm[DARK_FRAGMENT_BINARY],
        };

        for x in new_image.top_left.x..=new_image.bottom_right.x {
            for y in new_image.top_left.y..=new_image.bottom_right.y {
                let pixel = new_image
                    .pixels
                    .entry(Position { x, y })
                    .or_insert(new_outside_pixels_kind);
                *pixel = self.apply_algorithm_to_pixel(Position { x, y });
            }
        }

        self.input_image = new_image;
        self.outside_pixels_kind = new_outside_pixels_kind;
    }

    fn apply_algorithm_n_times(mut self, n: usize) -> Self {
        (0..n).for_each(|_| self.apply_algorithm());
        self
    }

    fn count_pixels(&self, pixels_kind: Pixel) -> usize {
        self.input_image
            .pixels
            .values()
            .filter(|pixel| **pixel == pixels_kind)
            .count()
    }

    fn pixel_to_binary(&self, pixel_position: &Position) -> usize {
        let mut result = 0;

        for y in -1..=1 {
            for x in -1..=1 {
                let pixel = self
                    .input_image
                    .pixels
                    .get(&(*pixel_position + (x, y).into()))
                    .unwrap_or(&self.outside_pixels_kind);

                result = result * 2 + usize::from(*pixel);
            }
        }

        result
    }
}

#[aoc_generator(day20)]
fn parse_input(input: &str) -> Processor {
    input.into()
}

#[aoc(day20, part1)]
fn part1(processor: &Processor) -> usize {
    (*processor)
        .to_owned()
        .apply_algorithm_n_times(2)
        .count_pixels(Light)
}

#[aoc(day20, part2)]
fn part2(processor: &Processor) -> usize {
    (*processor)
        .to_owned()
        .apply_algorithm_n_times(50)
        .count_pixels(Light)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##
#..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###
.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.
.#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....
.#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..
...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....
..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 3_351);
    }
}
