use crate::day04::BoardState::*;
use crate::day04::CellState::*;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Clone, PartialEq)]
enum CellState {
    Unmarked,
    Marked,
}

#[derive(Debug, Clone)]
struct Position {
    row: usize,
    column: usize,
}

#[derive(Debug, Clone)]
struct Cell {
    number: u8,
    position: Position,
    state: CellState,
}

#[derive(Debug, Clone, PartialEq)]
enum BoardState {
    Playing,
    Won,
}

#[derive(Debug, Clone)]
struct Board {
    cells: Vec<Cell>,
    state: BoardState,
}

impl Board {
    fn make_turn(&mut self, number: &u8) -> BoardState {
        if let Some(position) = self.mark_number(number) {
            self.update_state(&position);
        }

        self.state.to_owned()
    }

    fn mark_number(&mut self, number: &u8) -> Option<Position> {
        self.cells
            .iter_mut()
            .filter(|cell| cell.number == *number)
            .map(|cell| {
                cell.state = Marked;
                cell.position.to_owned()
            })
            .next()
    }

    fn update_state(&mut self, position: &Position) {
        if !self
            .cells
            .iter()
            .any(|c| c.position.column == position.column && c.state == Unmarked)
            || !self
                .cells
                .iter()
                .any(|c| c.position.row == position.row && c.state == Unmarked)
        {
            self.state = Won;
        }
    }

    fn calculate_score(&self, last_number: &u8) -> u32 {
        self.cells
            .iter()
            .filter(|&c| c.state == Unmarked)
            .map(|c| c.number as u32)
            .sum::<u32>()
            * *last_number as u32
    }
}

type Order = Vec<u8>;

fn parse_order(order_str: &str) -> Order {
    order_str
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect()
}

fn parse_board(board_str: &str) -> Board {
    Board {
        cells: board_str
            .lines()
            .enumerate()
            .map(|(row_num, row)| {
                row.split_whitespace()
                    .enumerate()
                    .map(|(column_num, n)| Cell {
                        number: n.parse().unwrap(),
                        position: Position {
                            row: row_num,
                            column: column_num,
                        },
                        state: Unmarked,
                    })
                    .collect::<Vec<Cell>>()
            })
            .flatten()
            .collect(),
        state: Playing,
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> (Order, Vec<Board>) {
    let mut split = input.split("\n\n");

    let order = parse_order(split.next().unwrap());
    let boards = split.map(parse_board).collect();

    (order, boards)
}

#[aoc(day4, part1)]
fn part1((order, boards): &(Order, Vec<Board>)) -> Option<u32> {
    let mut boards = (*boards).clone();

    for number in order {
        for board in boards.iter_mut() {
            if board.make_turn(number) == Won {
                return Some(board.calculate_score(number));
            }
        }
    }

    None
}

#[aoc(day4, part2)]
fn part2((order, boards): &(Order, Vec<Board>)) -> Option<u32> {
    let mut boards = (*boards).clone();
    let mut playing_count = boards.len();

    for number in order {
        for board in boards.iter_mut() {
            if board.state == Won {
                continue;
            }

            if board.make_turn(number) == Won {
                playing_count -= 1;

                if playing_count == 0 {
                    return Some(board.calculate_score(number));
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), Some(4512));
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), Some(1924));
    }
}
