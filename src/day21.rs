use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

const PRACTICE_GAME_WINNING_SCORE: u64 = 1_000;
const DIRAC_GAME_WINNING_SCORE: u64 = 21;
const DETERMINISTIC_DIE_FACES: u64 = 100;
const DIRAC_DIE_FACES: u64 = 3;
const TRACK_LEN: u64 = 10;
const MAX_TURNS: usize = 10; //Each turn adds at least 1 to the score, but every other turn adds at least 1 + 3 = 4.

#[aoc_generator(day21)]
fn parse_input(input: &str) -> (u64, u64) {
    let mut lines = input.lines();

    (
        lines
            .next()
            .unwrap()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as u64,
        lines
            .next()
            .unwrap()
            .chars()
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as u64,
    )
}

fn deterministic_turn(position: &mut u64, die: &mut u64, score: &mut u64, rolls: &mut u64) {
    let mut move_distance = 0;

    (0..3).for_each(|_| {
        *die = wrap(*die + 1, DETERMINISTIC_DIE_FACES);
        *rolls += 1;
        move_distance += *die;
    });

    *position = wrap(*position + move_distance, TRACK_LEN);

    *score += *position;
}

#[aoc(day21, part1)]
fn part1(starting_positions: &(u64, u64)) -> u64 {
    let (mut p1_position, mut p2_position) = *starting_positions;
    let (mut p1_score, mut p2_score) = (0, 0);
    let mut die = 0;
    let mut rolls = 0;

    loop {
        deterministic_turn(&mut p1_position, &mut die, &mut p1_score, &mut rolls);

        if p1_score >= PRACTICE_GAME_WINNING_SCORE {
            return p2_score * rolls;
        }

        deterministic_turn(&mut p2_position, &mut die, &mut p2_score, &mut rolls);

        if p2_score >= PRACTICE_GAME_WINNING_SCORE {
            return p1_score * rolls;
        }
    }
}

struct Node {
    position: u64,
    score: u64,
    turns: usize,
    universes: u64,
}

fn wrap(number: u64, radix: u64) -> u64 {
    (radix + number - 1) % radix + 1
}

fn universes_per_turn(
    starting_position: u64,
    universes_between: &HashMap<(u64, u64), u64>,
) -> ([u64; MAX_TURNS + 1], [u64; MAX_TURNS + 1]) {
    let mut stack = vec![Node {
        position: starting_position,
        score: 0,
        turns: 0,
        universes: 1,
    }];

    let mut winning_by_turn = [0u64; MAX_TURNS + 1];
    let mut non_winning_by_turn = [0u64; MAX_TURNS + 1];

    while !stack.is_empty() {
        let node = stack.pop().unwrap();

        if node.score >= DIRAC_GAME_WINNING_SCORE {
            winning_by_turn[node.turns] += node.universes;
        } else {
            non_winning_by_turn[node.turns] += node.universes;

            for move_distance in 3..=DIRAC_DIE_FACES * 3 {
                let position = wrap(node.position + move_distance, TRACK_LEN);
                let universes =
                    node.universes * *universes_between.get(&(node.position, position)).unwrap();

                stack.push(Node {
                    position,
                    score: node.score + position,
                    turns: node.turns + 1,
                    universes,
                })
            }
        }
    }

    (winning_by_turn, non_winning_by_turn)
}

#[aoc(day21, part2)]
fn part2((p1_position, p2_position): &(u64, u64)) -> u64 {
    let mut universes_between: HashMap<(u64, u64), u64> =
        HashMap::with_capacity((TRACK_LEN * (TRACK_LEN - 3)) as usize);

    for origin in 1..=TRACK_LEN {
        for first in 1..=DIRAC_DIE_FACES {
            for second in 1..=DIRAC_DIE_FACES {
                for third in 1..=DIRAC_DIE_FACES {
                    let destination = wrap(origin + first + second + third, TRACK_LEN);

                    *universes_between.entry((origin, destination)).or_insert(0) += 1;
                }
            }
        }
    }

    let (p1_winning_by_turn, p1_non_winning_by_turn) =
        universes_per_turn(*p1_position, &universes_between);

    let (p2_winning_by_turn, p2_non_winning_by_turn) =
        universes_per_turn(*p2_position, &universes_between);

    let mut p1_wins = 0;
    let mut p2_wins = 0;

    for turn in 3..=MAX_TURNS {
        p1_wins += p1_winning_by_turn[turn] * p2_non_winning_by_turn[turn - 1];
        p2_wins += p2_winning_by_turn[turn] * p1_non_winning_by_turn[turn];
    }

    p2_wins.max(p1_wins)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 739_785);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 444_356_092_776_315);
    }
}
