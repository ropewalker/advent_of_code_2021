use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

type PolymerTemplate = Vec<char>;
type Element = char;

#[aoc_generator(day14)]
fn parse_input(input: &str) -> (PolymerTemplate, HashMap<(Element, Element), Element>) {
    let mut split = input.split("\n\n");

    (
        split.next().unwrap().chars().collect(),
        split
            .next()
            .unwrap()
            .lines()
            .map(|l| {
                let mut chars = l.chars();
                (
                    (chars.next().unwrap(), chars.next().unwrap()),
                    chars.last().unwrap(),
                )
            })
            .collect(),
    )
}

fn count_elements_and_pairs(
    polymer_template: &[Element],
) -> (HashMap<Element, usize>, HashMap<(Element, Element), usize>) {
    let mut element_counts: HashMap<Element, usize> = HashMap::new();
    let mut pair_counts: HashMap<(Element, Element), usize> = HashMap::new();

    polymer_template.windows(2).for_each(|pair| {
        element_counts
            .entry(pair[0])
            .and_modify(|count| *count += 1)
            .or_insert(1);
        pair_counts
            .entry((pair[0], pair[1]))
            .and_modify(|count| *count += 1)
            .or_insert(1);
    });

    element_counts
        .entry(*polymer_template.last().unwrap())
        .and_modify(|count| *count += 1)
        .or_insert(1);

    (element_counts, pair_counts)
}

fn make_insertion_step(
    pair_counts: &mut HashMap<(Element, Element), usize>,
    element_counts: &mut HashMap<Element, usize>,
    pair_insertion_rules: &HashMap<(Element, Element), Element>,
) {
    let mut new_pair_counts = pair_counts.clone();

    for (pair, inserted_element) in pair_insertion_rules {
        if let Some(pair_count) = pair_counts.get(pair) {
            let count = pair_count;

            new_pair_counts.entry(*pair).and_modify(|e| *e -= *count);

            element_counts
                .entry(*inserted_element)
                .and_modify(|e| *e += *count)
                .or_insert(*count);

            new_pair_counts
                .entry((pair.0, *inserted_element))
                .and_modify(|e| *e += *count)
                .or_insert(*count);
            new_pair_counts
                .entry((*inserted_element, pair.1))
                .and_modify(|e| *e += *count)
                .or_insert(*count);
        }
    }

    *pair_counts = new_pair_counts;
}

fn make_n_steps(
    polymer_template: &[Element],
    pair_insertion_rules: &HashMap<(Element, Element), Element>,
    steps: usize,
) -> usize {
    let (mut element_counts, mut pair_counts) = count_elements_and_pairs(polymer_template);

    (0..steps).for_each(|_| {
        make_insertion_step(&mut pair_counts, &mut element_counts, pair_insertion_rules)
    });

    element_counts.values().max().unwrap() - element_counts.values().min().unwrap()
}

#[aoc(day14, part1)]
fn part1(
    (polymer_template, pair_insertion_rules): &(
        PolymerTemplate,
        HashMap<(Element, Element), Element>,
    ),
) -> usize {
    make_n_steps(polymer_template, pair_insertion_rules, 10)
}

#[aoc(day14, part2)]
fn part2(
    (polymer_template, pair_insertion_rules): &(
        PolymerTemplate,
        HashMap<(Element, Element), Element>,
    ),
) -> usize {
    make_n_steps(polymer_template, pair_insertion_rules, 40)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = r"NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 1_588);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 2_188_189_693_529);
    }
}
