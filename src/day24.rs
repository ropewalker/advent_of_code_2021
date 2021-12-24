use crate::day24::ModelNumberType::{Largest, Smallest};
use aoc_runner_derive::{aoc, aoc_generator};

const NUM_OF_INPUTS: usize = 14;
const BLOCK_LEN: usize = 18;
const DIV_Z_POSITION: usize = 4;
const ADD_X_POSITION: usize = 5;
const ADD_Y_POSITION: usize = 15;

#[derive(Debug)]
struct Block {
    div_z: i64,
    add_x: i64,
    add_y: i64,
}

#[aoc_generator(day24)]
fn parse_input(input: &str) -> Vec<Block> {
    let mut blocks = Vec::with_capacity(NUM_OF_INPUTS);
    let instructions = input.lines().collect::<Vec<_>>();

    for i in 0..NUM_OF_INPUTS {
        blocks.push(Block {
            div_z: instructions[i * BLOCK_LEN + DIV_Z_POSITION]
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            add_x: instructions[i * BLOCK_LEN + ADD_X_POSITION]
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
            add_y: instructions[i * BLOCK_LEN + ADD_Y_POSITION]
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap(),
        })
    }

    blocks
}

struct Node {
    index: usize,
    z_input: i64,
    total_input: i64,
}

enum ModelNumberType {
    Largest,
    Smallest,
}

fn find_model_number_by_type(blocks: &[Block], model_number_type: ModelNumberType) -> Option<i64> {
    let mut nodes = vec![Node {
        index: 0,
        z_input: 0,
        total_input: 0,
    }];

    while !nodes.is_empty() {
        let node = nodes.pop().unwrap();

        if node.index == NUM_OF_INPUTS {
            if node.z_input == 0 {
                return Some(node.total_input);
            } else {
                continue;
            }
        }

        let block = blocks.get(node.index).unwrap();

        for w in match model_number_type {
            ModelNumberType::Largest => (1..=9).collect::<Vec<_>>(),
            ModelNumberType::Smallest => (1..=9).collect::<Vec<_>>(),
        } {
            if block.div_z == 26 && node.z_input % 26 + block.add_x != w {
                continue;
            }

            let z_output = if node.z_input % 26 + block.add_x == w {
                node.z_input / block.div_z
            } else {
                node.z_input / block.div_z * 26 + w + block.add_y
            };

            nodes.push(Node {
                index: node.index + 1,
                z_input: z_output,
                total_input: node.total_input * 10 + w,
            })
        }
    }

    None
}

#[aoc(day24, part1)]
fn part1(blocks: &[Block]) -> Option<i64> {
    find_model_number_by_type(blocks, Largest)
}

#[aoc(day24, part2)]
fn part2(blocks: &[Block]) -> Option<i64> {
    find_model_number_by_type(blocks, Smallest)
}
