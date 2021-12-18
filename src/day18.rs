use crate::day18::Token::*;
use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;
use std::fmt::{Display, Formatter};
use std::ops::{Add, AddAssign};

#[derive(Clone)]
enum Token {
    LeftBracket,
    RightBracket,
    Comma,
    Number(u32),
}

impl From<char> for Token {
    fn from(c: char) -> Self {
        match c {
            '[' => LeftBracket,
            ']' => RightBracket,
            ',' => Comma,
            _ => Number(c.to_digit(10).unwrap()),
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LeftBracket => write!(f, "["),
            RightBracket => write!(f, "]"),
            Comma => write!(f, ","),
            Number(d) => write!(f, "{}", d),
        }
    }
}

#[derive(Clone)]
struct SnailfishNumber(VecDeque<Token>);

impl From<&str> for SnailfishNumber {
    fn from(snailfish_number_str: &str) -> Self {
        SnailfishNumber(snailfish_number_str.chars().map(|c| c.into()).collect())
    }
}

impl Display for SnailfishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .fold(String::with_capacity(self.0.len()), |acc, f| {
                    acc + f.to_string().as_str()
                })
        )
    }
}

impl SnailfishNumber {
    fn explode(&self, with_split: bool) -> Self {
        let mut left_part = VecDeque::with_capacity(self.0.capacity());
        let mut right_part = self.0.to_owned();
        let mut nesting_level = 0usize;

        while !right_part.is_empty() {
            let token = right_part.pop_front().unwrap();

            match token {
                LeftBracket => {
                    if nesting_level < 4 {
                        left_part.push_back(token);
                        nesting_level += 1;
                    } else {
                        let left_number = right_part.pop_front().unwrap();
                        let _comma = right_part.pop_front().unwrap();
                        let right_number = right_part.pop_front().unwrap();
                        let _right_bracket = right_part.pop_front().unwrap();

                        let mut middle_part = VecDeque::new();

                        while !right_part.is_empty() {
                            let right_token = right_part.pop_front().unwrap();

                            if let Number(rhs) = right_token {
                                if let Number(lhs) = right_number {
                                    right_part.push_front(Number(lhs + rhs));
                                    break;
                                }
                            }

                            middle_part.push_back(right_token);
                        }

                        while !middle_part.is_empty() {
                            right_part.push_front(middle_part.pop_back().unwrap());
                        }

                        let mut middle_to_left = true;

                        while !left_part.is_empty() {
                            let left_token = left_part.pop_back().unwrap();

                            if let Number(lhs) = left_token {
                                if let Number(rhs) = left_number {
                                    let sum = lhs + rhs;

                                    if sum < 10 || !with_split {
                                        left_part.push_back(Number(sum));
                                    } else {
                                        middle_part.push_front(Number(sum));
                                        middle_to_left = false;
                                    }

                                    break;
                                }
                            }

                            middle_part.push_front(left_token);
                        }

                        if middle_to_left {
                            left_part.append(&mut middle_part);

                            left_part.push_back(Number(0));
                        } else {
                            right_part.push_front(Number(0));

                            while !middle_part.is_empty() {
                                let token = middle_part.pop_back().unwrap();

                                match token {
                                    LeftBracket => {
                                        nesting_level -= 1;
                                    }
                                    RightBracket => {
                                        nesting_level += 1;
                                    }
                                    _ => {}
                                }

                                right_part.push_front(token);
                            }
                        }
                    }
                }
                RightBracket => {
                    left_part.push_back(token);
                    nesting_level -= 1;
                }
                Comma => {
                    left_part.push_back(token);
                }
                Number(x) => {
                    if x < 10 || !with_split {
                        left_part.push_back(token);
                    } else {
                        let left = x / 2;
                        let right = x - left;

                        right_part.push_front(RightBracket);
                        right_part.push_front(Number(right));
                        right_part.push_front(Comma);
                        right_part.push_front(Number(left));
                        right_part.push_front(LeftBracket);
                    }
                }
            }
        }

        Self(left_part)
    }

    fn reduce(&self) -> Self {
        self.explode(false).explode(true)
    }

    fn magnitude(&self) -> u32 {
        let mut stack: Vec<u32> = Vec::new();

        for token in &self.0 {
            match token {
                RightBracket => {
                    let rhs = stack.pop().unwrap();
                    let lhs = stack.pop().unwrap();
                    stack.push(rhs * 2 + lhs * 3);
                }
                Number(x) => {
                    stack.push(*x);
                }
                _ => {}
            }
        }

        stack.pop().unwrap()
    }
}

impl Add<&SnailfishNumber> for &SnailfishNumber {
    type Output = SnailfishNumber;

    fn add(self, rhs: &SnailfishNumber) -> SnailfishNumber {
        let mut result = self.0.to_owned();

        result.push_front(LeftBracket);
        result.push_back(Comma);
        result.append(&mut rhs.0.to_owned());
        result.push_back(RightBracket);

        SnailfishNumber(result).reduce()
    }
}

impl AddAssign<&SnailfishNumber> for SnailfishNumber {
    fn add_assign(&mut self, rhs: &SnailfishNumber) {
        *self = &*self + rhs
    }
}

#[aoc_generator(day18)]
fn parse_input(input: &str) -> Vec<SnailfishNumber> {
    input.lines().map(|l| l.into()).collect()
}

fn final_sum(math_homework: &[SnailfishNumber]) -> SnailfishNumber {
    let mut result = math_homework.first().unwrap().to_owned();

    for snailfish_number in math_homework.iter().skip(1) {
        result += snailfish_number;
    }

    result
}

#[aoc(day18, part1)]
fn part1(math_homework: &[SnailfishNumber]) -> u32 {
    final_sum(math_homework).magnitude()
}

#[aoc(day18, part2)]
fn part2(math_homework: &[SnailfishNumber]) -> u32 {
    let mut result = 0;

    for (i, lhs) in math_homework.iter().enumerate() {
        for rhs in math_homework.iter().skip(i) {
            result = result
                .max((lhs + rhs).magnitude())
                .max((rhs + lhs).magnitude());
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXPLODE_EXAMPLE_INPUT_1: &str = "[[[[[9,8],1],2],3],4]";
    static EXPLODE_EXAMPLE_RESULT_1: &str = "[[[[0,9],2],3],4]";

    static EXPLODE_EXAMPLE_INPUT_2: &str = "[7,[6,[5,[4,[3,2]]]]]";
    static EXPLODE_EXAMPLE_RESULT_2: &str = "[7,[6,[5,[7,0]]]]";

    static EXPLODE_EXAMPLE_INPUT_3: &str = "[[6,[5,[4,[3,2]]]],1]";
    static EXPLODE_EXAMPLE_RESULT_3: &str = "[[6,[5,[7,0]]],3]";

    static EXPLODE_EXAMPLE_INPUT_4: &str = "[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]";
    static EXPLODE_EXAMPLE_RESULT_4: &str = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";

    static EXPLODE_EXAMPLE_INPUT_5: &str = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
    static EXPLODE_EXAMPLE_RESULT_5: &str = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";

    static SUM_EXAMPLE_LHS: &str = "[[[[4,3],4],4],[7,[[8,4],9]]]";
    static SUM_EXAMPLE_RHS: &str = "[1,1]";
    static SUM_EXAMPLE_RESULT: &str = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";

    static FINAL_SUM_TEST_INPUT_1: &str = "[1,1]
[2,2]
[3,3]
[4,4]";
    static FINAL_SUM_TEST_RESULT_1: &str = "[[[[1,1],[2,2]],[3,3]],[4,4]]";

    static FINAL_SUM_TEST_INPUT_2: &str = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]";
    static FINAL_SUM_TEST_RESULT_2: &str = "[[[[3,0],[5,3]],[4,4]],[5,5]]";

    static FINAL_SUM_TEST_INPUT_3: &str = "[1,1]
[2,2]
[3,3]
[4,4]
[5,5]
[6,6]";
    static FINAL_SUM_TEST_RESULT_3: &str = "[[[[5,0],[7,4]],[5,5]],[6,6]]";

    static FINAL_SUM_TEST_INPUT_4: &str = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";
    static FINAL_SUM_TEST_RESULT_4: &str = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

    static MAGNITUDE_EXAMPLE_1: &str = "[9,1]";
    static MAGNITUDE_EXAMPLE_2: &str = "[1,9]";
    static MAGNITUDE_EXAMPLE_3: &str = "[[9,1],[1,9]]";
    static MAGNITUDE_EXAMPLE_4: &str = "[[1,2],[[3,4],5]]";
    static MAGNITUDE_EXAMPLE_5: &str = "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]";
    static MAGNITUDE_EXAMPLE_6: &str = "[[[[1,1],[2,2]],[3,3]],[4,4]]";
    static MAGNITUDE_EXAMPLE_7: &str = "[[[[3,0],[5,3]],[4,4]],[5,5]]";
    static MAGNITUDE_EXAMPLE_8: &str = "[[[[5,0],[7,4]],[5,5]],[6,6]]";
    static MAGNITUDE_EXAMPLE_9: &str = "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]";

    static TEST_INPUT: &str = r"[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";

    #[test]
    fn explode_example() {
        assert_eq!(
            SnailfishNumber::from(EXPLODE_EXAMPLE_INPUT_1)
                .reduce()
                .to_string(),
            EXPLODE_EXAMPLE_RESULT_1.to_string()
        );
        assert_eq!(
            SnailfishNumber::from(EXPLODE_EXAMPLE_INPUT_2)
                .reduce()
                .to_string(),
            EXPLODE_EXAMPLE_RESULT_2.to_string()
        );
        assert_eq!(
            SnailfishNumber::from(EXPLODE_EXAMPLE_INPUT_3)
                .reduce()
                .to_string(),
            EXPLODE_EXAMPLE_RESULT_3.to_string()
        );
        assert_eq!(
            SnailfishNumber::from(EXPLODE_EXAMPLE_INPUT_4)
                .reduce()
                .to_string(),
            EXPLODE_EXAMPLE_RESULT_4.to_string()
        );
        assert_eq!(
            SnailfishNumber::from(EXPLODE_EXAMPLE_INPUT_5)
                .reduce()
                .to_string(),
            EXPLODE_EXAMPLE_RESULT_5.to_string()
        );
    }

    #[test]
    fn sum_example() {
        assert_eq!(
            (&SnailfishNumber::from(SUM_EXAMPLE_LHS) + &SnailfishNumber::from(SUM_EXAMPLE_RHS))
                .to_string(),
            SUM_EXAMPLE_RESULT.to_string()
        );
    }

    #[test]
    fn final_sum_example() {
        assert_eq!(
            final_sum(&parse_input(FINAL_SUM_TEST_INPUT_1)).to_string(),
            FINAL_SUM_TEST_RESULT_1.to_string()
        );

        assert_eq!(
            final_sum(&parse_input(FINAL_SUM_TEST_INPUT_2)).to_string(),
            FINAL_SUM_TEST_RESULT_2.to_string()
        );

        assert_eq!(
            final_sum(&parse_input(FINAL_SUM_TEST_INPUT_3)).to_string(),
            FINAL_SUM_TEST_RESULT_3.to_string()
        );

        assert_eq!(
            final_sum(&parse_input(FINAL_SUM_TEST_INPUT_4)).to_string(),
            FINAL_SUM_TEST_RESULT_4.to_string()
        );
    }

    #[test]
    fn magnitude_example() {
        assert_eq!(SnailfishNumber::from(MAGNITUDE_EXAMPLE_1).magnitude(), 29);
        assert_eq!(SnailfishNumber::from(MAGNITUDE_EXAMPLE_2).magnitude(), 21);
        assert_eq!(SnailfishNumber::from(MAGNITUDE_EXAMPLE_3).magnitude(), 129);
        assert_eq!(SnailfishNumber::from(MAGNITUDE_EXAMPLE_4).magnitude(), 143);
        assert_eq!(
            SnailfishNumber::from(MAGNITUDE_EXAMPLE_5).magnitude(),
            1_384
        );
        assert_eq!(SnailfishNumber::from(MAGNITUDE_EXAMPLE_6).magnitude(), 445);
        assert_eq!(SnailfishNumber::from(MAGNITUDE_EXAMPLE_7).magnitude(), 791);
        assert_eq!(
            SnailfishNumber::from(MAGNITUDE_EXAMPLE_8).magnitude(),
            1_137
        );
        assert_eq!(
            SnailfishNumber::from(MAGNITUDE_EXAMPLE_9).magnitude(),
            3_488
        );
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 4_140);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 3_993);
    }
}
