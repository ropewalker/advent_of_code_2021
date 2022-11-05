use crate::day25::Cucumber::{EastFacing, SouthFacing};
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::ops::Add;

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
enum Cucumber {
    EastFacing,
    SouthFacing,
}

impl From<char> for Cucumber {
    fn from(cucumber: char) -> Self {
        match cucumber {
            '>' => EastFacing,
            'v' => SouthFacing,
            _ => unreachable!(),
        }
    }
}

impl Debug for Cucumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                EastFacing => ">",
                SouthFacing => "v",
            }
        )
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Vec2 {
    x: usize,
    y: usize,
}

type Position = Vec2;

impl From<(usize, usize)> for Vec2 {
    fn from((x, y): (usize, usize)) -> Self {
        Self { x, y }
    }
}

impl From<Vec2> for (usize, usize) {
    fn from(vec2: Vec2) -> Self {
        (vec2.x, vec2.y)
    }
}

impl From<Cucumber> for Vec2 {
    fn from(cucumber: Cucumber) -> Self {
        match cucumber {
            EastFacing => (1, 0).into(),
            SouthFacing => (0, 1).into(),
        }
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

#[derive(Clone)]
struct Map {
    cucumbers_positions: HashMap<Position, Cucumber>,
    width: usize,
    height: usize,
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            writeln!(f)?;

            for x in 0..self.width {
                if self.cucumbers_positions.contains_key(&((x, y).into())) {
                    write!(
                        f,
                        "{:?}",
                        self.cucumbers_positions.get(&((x, y).into())).unwrap()
                    )?;
                } else {
                    write!(f, ".")?;
                };
            }
        }

        Ok(())
    }
}

impl From<&str> for Map {
    fn from(map: &str) -> Self {
        Map {
            cucumbers_positions: map
                .lines()
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_, c)| *c != '.')
                        .map(move |(x, c)| ((x, y).into(), c.into()))
                })
                .collect(),
            width: map.lines().next().unwrap().len(),
            height: map.lines().count(),
        }
    }
}

impl Map {
    fn stabilize(&mut self) -> usize {
        let mut steps = 1;

        while self.step() {
            steps += 1;
        }

        steps
    }

    fn move_cucumbers(&mut self, cucumber_type: Cucumber) -> bool {
        let mut moved = false;

        let mut new_cucumbers_positions = HashMap::with_capacity(self.cucumbers_positions.len());

        for (position, cucumber) in self
            .cucumbers_positions
            .iter()
            .filter(|(_, cucumber)| **cucumber != cucumber_type)
        {
            new_cucumbers_positions.insert(*position, cucumber.to_owned());
        }

        for (position, cucumber) in self
            .cucumbers_positions
            .iter()
            .filter(|(_, cucumber)| **cucumber == cucumber_type)
        {
            let mut new_position = *position + cucumber_type.into();

            if cucumber_type == EastFacing {
                new_position.x %= self.width;
            } else {
                new_position.y %= self.height;
            }

            if !self.cucumbers_positions.contains_key(&new_position) {
                new_cucumbers_positions.insert(new_position, cucumber.to_owned());
                moved = true;
            } else {
                new_cucumbers_positions.insert(*position, cucumber.to_owned());
            }
        }

        self.cucumbers_positions = new_cucumbers_positions;

        moved
    }

    fn step(&mut self) -> bool {
        let east_facing_moved = self.move_cucumbers(EastFacing);
        let south_facing_moved = self.move_cucumbers(SouthFacing);

        east_facing_moved || south_facing_moved
    }
}

#[aoc_generator(day25)]
fn parse_input(input: &str) -> Map {
    input.into()
}

#[aoc(day25, part1)]
fn part1(map: &Map) -> usize {
    let mut map = (*map).to_owned();

    map.stabilize()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 58);
    }
}
