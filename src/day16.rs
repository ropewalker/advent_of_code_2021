use crate::day16::PacketBody::*;
use crate::day16::Token::*;
use aoc_runner_derive::{aoc, aoc_generator};

const LITERAL_VALUE_TYPE_ID: u64 = 4;
const VERSION_LEN: usize = 3;
const TYPE_ID_LEN: usize = 3;
const GROUP_LEN: usize = 5;
const TOTAL_BIT_LEN_LEN: usize = 15;
const SUB_PACKETS_NUM_LEN: usize = 11;

#[derive(Clone, Copy, Debug)]
enum Token {
    LiteralValue(u64),
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    EqualTo,
    ClosingBracket,
}

#[derive(Debug)]
struct Packet {
    version: u64,
    packet_type: Token,
}

#[derive(Eq, PartialEq)]
enum PacketBody {
    Bits(usize),
    SubPackets(usize),
}

#[aoc_generator(day16)]
fn parse_input(input: &str) -> Vec<Packet> {
    let packet_string = input
        .chars()
        .map(|c| c.to_digit(16).unwrap())
        .map(|d| format!("{:04b}", d))
        .fold(String::new(), |acc, s| acc + s.as_str());

    let mut index = 0;
    let mut parsing_stack = vec![SubPackets(1)];
    let mut packet_stack = Vec::new();

    while !parsing_stack.is_empty() {
        let packet_body = parsing_stack.pop().unwrap();

        if packet_body == Bits(0) || packet_body == SubPackets(0) {
            packet_stack.push(Packet {
                version: 0,
                packet_type: ClosingBracket,
            });

            continue;
        }

        let mut bits_used = 0;

        let version = u64::from_str_radix(&packet_string[index..index + VERSION_LEN], 2).unwrap();

        index += VERSION_LEN;
        bits_used += VERSION_LEN;

        let type_id = u64::from_str_radix(&packet_string[index..index + TYPE_ID_LEN], 2).unwrap();

        index += TYPE_ID_LEN;
        bits_used += TYPE_ID_LEN;

        if type_id == LITERAL_VALUE_TYPE_ID {
            let mut value = String::new();

            loop {
                let group_prefix = packet_string
                    .chars()
                    .nth(index)
                    .unwrap()
                    .to_digit(2)
                    .unwrap();

                value.push_str(&packet_string[index + 1..index + GROUP_LEN]);

                index += GROUP_LEN;
                bits_used += GROUP_LEN;

                if group_prefix == 0 {
                    break;
                }
            }

            packet_stack.push(Packet {
                version,
                packet_type: Token::LiteralValue(u64::from_str_radix(value.as_str(), 2).unwrap()),
            });

            for packet_body in parsing_stack.iter_mut() {
                if let Bits(bits_num) = packet_body {
                    *packet_body = Bits(*bits_num - bits_used);
                }
            }

            match packet_body {
                Bits(bits_num) => {
                    parsing_stack.push(Bits(bits_num - bits_used));
                }
                SubPackets(sub_packets_num) => {
                    parsing_stack.push(SubPackets(sub_packets_num - 1));
                }
            }
        } else {
            packet_stack.push(Packet {
                version,
                packet_type: match type_id {
                    0 => Sum,
                    1 => Product,
                    2 => Minimum,
                    3 => Maximum,
                    5 => GreaterThan,
                    6 => LessThan,
                    7 => EqualTo,
                    _ => unreachable!(),
                },
            });

            let length_type_id = packet_string
                .chars()
                .nth(index)
                .unwrap()
                .to_digit(2)
                .unwrap();

            index += 1;
            bits_used += 1;

            let shift = match length_type_id {
                0 => TOTAL_BIT_LEN_LEN,
                1 => SUB_PACKETS_NUM_LEN,
                _ => unreachable!(),
            };

            let value = usize::from_str_radix(&packet_string[index..index + shift], 2).unwrap();

            index += shift;
            bits_used += shift;

            for packet_body in parsing_stack.iter_mut() {
                if let Bits(bits_num) = packet_body {
                    *packet_body = Bits(*bits_num - bits_used);
                }
            }

            match packet_body {
                Bits(bits_num) => {
                    parsing_stack.push(Bits(bits_num - bits_used));
                }
                SubPackets(sub_packets_num) => {
                    parsing_stack.push(SubPackets(sub_packets_num - 1));
                }
            }

            match length_type_id {
                0 => parsing_stack.push(Bits(value)),
                1 => parsing_stack.push(SubPackets(value)),
                _ => unreachable!(),
            };
        }
    }

    packet_stack.pop();

    packet_stack
}

#[aoc(day16, part1)]
fn part1(packet_stack: &[Packet]) -> u64 {
    packet_stack.iter().map(|packet| packet.version).sum()
}

#[aoc(day16, part2)]
fn part2(packets: &[Packet]) -> Option<u64> {
    let mut tokens: Vec<Token> = Vec::new();

    for packet in packets.iter() {
        match packet.packet_type {
            ClosingBracket => {
                let mut token = tokens.pop().unwrap();
                let mut operands = Vec::new();

                while let LiteralValue(x) = token {
                    operands.push(x);
                    token = tokens.pop().unwrap();
                }

                match token {
                    Sum => tokens.push(LiteralValue(operands.into_iter().sum())),
                    Product => tokens.push(LiteralValue(operands.into_iter().product())),
                    Minimum => tokens.push(LiteralValue(operands.into_iter().min().unwrap())),
                    Maximum => tokens.push(LiteralValue(operands.into_iter().max().unwrap())),
                    GreaterThan => tokens.push(LiteralValue(
                        (operands.pop().unwrap() > operands.pop().unwrap()) as u64,
                    )),
                    LessThan => tokens.push(LiteralValue(
                        (operands.pop().unwrap() < operands.pop().unwrap()) as u64,
                    )),
                    EqualTo => tokens.push(LiteralValue(
                        (operands.pop().unwrap() == operands.pop().unwrap()) as u64,
                    )),
                    _ => unreachable!(),
                }
            }
            operator => {
                tokens.push(operator);
            }
        }
    }

    if let LiteralValue(x) = tokens.into_iter().next().unwrap() {
        Some(x)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_1: &str = r"8A004A801A8002F478";
    static TEST_INPUT_2: &str = r"620080001611562C8802118E34";
    static TEST_INPUT_3: &str = r"C0015000016115A2E0802F182340";
    static TEST_INPUT_4: &str = r"A0016C880162017C3686B18A3D4780";
    static TEST_INPUT_5: &str = r"C200B40A82";
    static TEST_INPUT_6: &str = r"04005AC33890";
    static TEST_INPUT_7: &str = r"880086C3E88112";
    static TEST_INPUT_8: &str = r"CE00C43D881120";
    static TEST_INPUT_9: &str = r"D8005AC2A8F0";
    static TEST_INPUT_10: &str = r"F600BC2D8F";
    static TEST_INPUT_11: &str = r"9C005AC2F8F0";
    static TEST_INPUT_12: &str = r"9C0141080250320F1802104A08";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT_1)), 16);
        assert_eq!(part1(&parse_input(TEST_INPUT_2)), 12);
        assert_eq!(part1(&parse_input(TEST_INPUT_3)), 23);
        assert_eq!(part1(&parse_input(TEST_INPUT_4)), 31);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT_5)), Some(3));
        assert_eq!(part2(&parse_input(TEST_INPUT_6)), Some(54));
        assert_eq!(part2(&parse_input(TEST_INPUT_7)), Some(7));
        assert_eq!(part2(&parse_input(TEST_INPUT_8)), Some(9));
        assert_eq!(part2(&parse_input(TEST_INPUT_9)), Some(1));
        assert_eq!(part2(&parse_input(TEST_INPUT_10)), Some(0));
        assert_eq!(part2(&parse_input(TEST_INPUT_11)), Some(0));
        assert_eq!(part2(&parse_input(TEST_INPUT_12)), Some(1));
    }
}
