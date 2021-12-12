use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

type CaveSystem = HashMap<String, HashSet<String>>;

const START: &str = "start";
const END: &str = "end";

#[aoc_generator(day12)]
fn parse_input(input: &str) -> CaveSystem {
    let mut cave_system = CaveSystem::new();

    for line in input.lines() {
        let mut split = line.split('-');

        let cave_a = split.next().unwrap();
        let cave_b = split.next().unwrap();

        let cave_a_connections = cave_system
            .entry(cave_a.to_owned())
            .or_insert_with(HashSet::new);
        cave_a_connections.insert(cave_b.to_owned());

        let cave_b_connections = cave_system
            .entry(cave_b.to_owned())
            .or_insert_with(HashSet::new);
        cave_b_connections.insert(cave_a.to_owned());
    }

    cave_system
}

fn is_small(cave: &str) -> bool {
    cave.to_ascii_lowercase() == *cave
}

fn count_simple_paths_to_end(
    cave_system: &CaveSystem,
    current_cave: &str,
    visited_small_caves: &mut HashSet<String>,
) -> usize {
    if current_cave == END {
        visited_small_caves.remove(current_cave);
        return 1;
    }

    let mut count = 0;

    if let Some(connected_caves) = cave_system.get(current_cave) {
        for connected_cave in connected_caves {
            if is_small(connected_cave) {
                if visited_small_caves.contains(connected_cave) {
                    continue;
                } else {
                    visited_small_caves.insert(connected_cave.to_owned());
                }
            }

            count += count_simple_paths_to_end(cave_system, connected_cave, visited_small_caves);

            visited_small_caves.remove(connected_cave);
        }
    }

    count
}

#[aoc(day12, part1)]
fn part1(cave_system: &CaveSystem) -> usize {
    let mut visited_small_caves = [START.to_owned()].iter().cloned().collect();

    count_simple_paths_to_end(cave_system, &(START.to_owned()), &mut visited_small_caves)
}

fn count_complex_paths_to_end(
    cave_system: &CaveSystem,
    current_cave: &str,
    visited_small_caves: &mut HashSet<String>,
    twice_visited_small_cave: &mut Option<String>,
) -> usize {
    if current_cave == END {
        visited_small_caves.remove(current_cave);
        return 1;
    }

    let mut count = 0;

    if let Some(connected_caves) = cave_system.get(current_cave) {
        for connected_cave in connected_caves {
            if is_small(connected_cave) {
                if !visited_small_caves.contains(connected_cave) {
                    visited_small_caves.insert(connected_cave.to_owned());
                } else if twice_visited_small_cave.is_none()
                    && connected_cave != START
                    && connected_cave != END
                {
                    *twice_visited_small_cave = Some(connected_cave.to_owned());
                } else {
                    continue;
                }
            }

            count += count_complex_paths_to_end(
                cave_system,
                connected_cave,
                visited_small_caves,
                twice_visited_small_cave,
            );

            if *twice_visited_small_cave == Some(connected_cave.to_owned()) {
                *twice_visited_small_cave = None
            } else {
                visited_small_caves.remove(connected_cave);
            }
        }
    }

    count
}

#[aoc(day12, part2)]
fn part2(cave_system: &CaveSystem) -> usize {
    let mut visited_caves = [START.to_owned()].iter().cloned().collect();
    let mut twice_visited_small_cave: Option<String> = None;

    count_complex_paths_to_end(
        cave_system,
        &(START.to_owned()),
        &mut visited_caves,
        &mut twice_visited_small_cave,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"start-A
start-b
A-c
A-b
b-d
A-end
b-end";

    static TEST_INPUT_2: &str = r"dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

    static TEST_INPUT_3: &str = r"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 10);
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 19);
        assert_eq!(part1(&parse_input(TEST_INPUT_3)), 226);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT_1)), 36);
        assert_eq!(part2(&parse_input(TEST_INPUT_2)), 103);
        assert_eq!(part2(&parse_input(TEST_INPUT_3)), 3_509);
    }
}
