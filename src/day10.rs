use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<String> {
    input.lines().map(|l| l.to_owned()).collect()
}

fn is_closing_bracket(c: char) -> bool {
    matches!(c, ')' | ']' | '}' | '>')
}

fn matching_closing_bracket(c: char) -> Option<char> {
    match c {
        '(' => Some(')'),
        '[' => Some(']'),
        '{' => Some('}'),
        '<' => Some('>'),
        _ => None,
    }
}

fn matching_opening_bracket(c: char) -> Option<char> {
    match c {
        ')' => Some('('),
        ']' => Some('['),
        '}' => Some('{'),
        '>' => Some('<'),
        _ => None,
    }
}

fn syntax_error_score(c: char) -> u64 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1_197,
        '>' => 25_137,
        _ => 0,
    }
}

fn autocomplete_score(c: char) -> u64 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => 0,
    }
}

#[aoc(day10, part1)]
fn part1(lines: &[String]) -> u64 {
    let mut result = 0;

    for line in lines {
        let mut stack = Vec::new();

        for c in line.chars() {
            if is_closing_bracket(c) {
                if stack.pop() != matching_opening_bracket(c) {
                    result += syntax_error_score(c);
                    break;
                }
            } else {
                stack.push(c);
            }
        }
    }

    result
}

#[aoc(day10, part2)]
fn part2(lines: &[String]) -> u64 {
    let mut total_scores = Vec::new();

    'next_line: for line in lines {
        let mut stack = Vec::new();

        for c in line.chars() {
            if is_closing_bracket(c) {
                if stack.pop() != matching_opening_bracket(c) {
                    continue 'next_line;
                }
            } else {
                stack.push(c);
            }
        }

        let mut total_score = 0;

        while !stack.is_empty() {
            let c = matching_closing_bracket(stack.pop().unwrap()).unwrap();

            total_score *= 5;
            total_score += autocomplete_score(c);
        }

        total_scores.push(total_score);
    }

    total_scores.sort_unstable();

    total_scores[total_scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 26_397);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 288_957);
    }
}
