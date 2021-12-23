use crate::day23::Amphipod::*;
use aoc_runner_derive::aoc;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
enum Amphipod {
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl From<char> for Amphipod {
    fn from(c: char) -> Self {
        match c {
            'A' => Amber,
            'B' => Bronze,
            'C' => Copper,
            'D' => Desert,
            _ => unreachable!(),
        }
    }
}

impl Amphipod {
    fn energy_by_step(&self) -> usize {
        match self {
            Amber => 1,
            Bronze => 10,
            Copper => 100,
            Desert => 1000,
        }
    }

    fn room_index(&self) -> usize {
        match self {
            Amber => 0,
            Bronze => 1,
            Copper => 2,
            Desert => 3,
        }
    }
}

type SideRoom = Vec<Amphipod>;

type SideRooms = [SideRoom; 4];
type Hallway = [Option<Amphipod>; 11];

fn parse_input(diagram: &str) -> SideRooms {
    let mut amphipods: HashMap<(usize, usize), Amphipod> = HashMap::new();

    diagram.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            'A' | 'B' | 'C' | 'D' => {
                amphipods.insert((x, y), c.into());
            }
            _ => {}
        })
    });

    let diagram_height = diagram.lines().count();

    let mut side_rooms = [
        SideRoom::with_capacity(diagram_height - 3),
        SideRoom::with_capacity(diagram_height - 3),
        SideRoom::with_capacity(diagram_height - 3),
        SideRoom::with_capacity(diagram_height - 3),
    ];

    for x in [3, 5, 7, 9] {
        for y in (2..diagram_height - 1).rev() {
            side_rooms[(x - 3) / 2].push(amphipods.remove(&(x, y)).unwrap());
        }
    }

    side_rooms
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Configuration {
    side_rooms: SideRooms,
    hallway: Hallway,
    room_capacity: usize,
}

fn room_index_to_hallway_index(room_index: usize) -> usize {
    (room_index + 1) * 2
}

impl Configuration {
    fn is_final(&self) -> bool {
        self.hallway.iter().all(|amphipod| amphipod.is_none())
            && self
                .side_rooms
                .iter()
                .enumerate()
                .all(|(index, side_room)| side_room.iter().all(|a| a.room_index() == index))
    }

    fn adjacent_configurations(&self) -> Vec<(Configuration, usize)> {
        let mut configurations_with_cost = Vec::new();

        'hallway: for (hallway_index, amphipod) in self
            .hallway
            .iter()
            .enumerate()
            .filter(|(_, amphipod)| amphipod.is_some())
            .map(|(i, amphipod)| (i, amphipod.unwrap()))
        {
            let target_room_index = amphipod.room_index();

            if self.side_rooms[target_room_index]
                .iter()
                .any(|a| a.room_index() != target_room_index)
            {
                continue;
            }

            let target_hallway_index = room_index_to_hallway_index(target_room_index);

            for i in
                hallway_index.min(target_hallway_index)..=hallway_index.max(target_hallway_index)
            {
                if i != hallway_index && self.hallway[i].is_some() {
                    continue 'hallway;
                }
            }

            let steps = target_hallway_index.max(hallway_index)
                - target_hallway_index.min(hallway_index)
                + self.room_capacity
                - self.side_rooms[target_room_index].len();

            let cost = steps * amphipod.energy_by_step();

            let mut new_configuration = self.clone();
            new_configuration.hallway[hallway_index] = None;
            new_configuration.side_rooms[target_room_index].push(amphipod);

            configurations_with_cost.push((new_configuration, cost));
        }

        for room_index in 0..4 {
            if self.side_rooms[room_index].is_empty() {
                continue;
            }

            if self.side_rooms[room_index]
                .iter()
                .all(|a| a.room_index() == room_index)
            {
                continue;
            }

            let hallway_index = room_index_to_hallway_index(room_index);

            for i in (0..hallway_index).rev() {
                if self.hallway[i].is_some() {
                    break;
                }

                if [2, 4, 6, 8].contains(&i) {
                    continue;
                }

                let mut new_configuration = self.clone();
                let amphipod = new_configuration.side_rooms[room_index].pop().unwrap();
                new_configuration.hallway[i] = Some(amphipod);

                let steps = hallway_index - i + new_configuration.room_capacity
                    - new_configuration.side_rooms[room_index].len();
                let cost = steps * amphipod.energy_by_step();

                configurations_with_cost.push((new_configuration, cost));
            }

            for i in hallway_index + 1..11 {
                if self.hallway[i].is_some() {
                    break;
                }

                if [2, 4, 6, 8].contains(&i) {
                    continue;
                }

                let mut new_configuration = self.clone();
                let amphipod = new_configuration.side_rooms[room_index].pop().unwrap();
                new_configuration.hallway[i] = Some(amphipod);

                let steps = i - hallway_index + new_configuration.room_capacity
                    - new_configuration.side_rooms[room_index].len();
                let cost = steps * amphipod.energy_by_step();

                configurations_with_cost.push((new_configuration, cost));
            }
        }

        configurations_with_cost
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    configuration: Configuration,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

fn organization_cost(side_rooms: &SideRooms) -> Option<usize> {
    let configuration = Configuration {
        side_rooms: side_rooms.to_owned(),
        hallway: [None; 11],
        room_capacity: side_rooms[0].len(),
    };

    let mut heap = BinaryHeap::new();
    let mut energy_to: HashMap<Configuration, usize> = HashMap::new();

    energy_to.insert(configuration.to_owned(), 0);

    heap.push(State {
        cost: 0,
        configuration,
    });

    while let Some(State {
        cost,
        configuration,
    }) = heap.pop()
    {
        if configuration.is_final() {
            return Some(cost);
        }

        if let Some(old_cost) = energy_to.get(&configuration) {
            if cost > *old_cost {
                continue;
            }
        }

        for (next_configuration, extra_cost) in configuration.adjacent_configurations() {
            let next = State {
                cost: cost + extra_cost,
                configuration: next_configuration,
            };

            if next.cost
                < *energy_to
                    .get(&next.configuration)
                    .unwrap_or(&(next.cost + 1))
            {
                energy_to.insert(next.configuration.clone(), next.cost);
                heap.push(next);
            }
        }
    }

    None
}

#[aoc(day23, part1)]
fn part1(diagram: &str) -> Option<usize> {
    let side_rooms = parse_input(diagram);
    organization_cost(&side_rooms)
}

fn unfold(diagram: &str) -> String {
    let mut lines = diagram.lines();
    let mut unfolded_diagram = String::new();

    (0..3).for_each(|_| {
        unfolded_diagram.push_str(lines.next().unwrap());
        unfolded_diagram.push('\n');
    });

    unfolded_diagram.push_str(
        r"  #D#C#B#A#
  #D#B#A#C#",
    );

    lines.for_each(|l| {
        unfolded_diagram.push('\n');
        unfolded_diagram.push_str(l);
    });

    unfolded_diagram
}

#[aoc(day23, part2)]
fn part2(diagram: &str) -> Option<usize> {
    let side_rooms = parse_input(&unfold(diagram));
    organization_cost(&side_rooms)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    #[test]
    fn part1_example() {
        assert_eq!(part1(TEST_INPUT), Some(12_521));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(TEST_INPUT), Some(44_169));
    }
}
