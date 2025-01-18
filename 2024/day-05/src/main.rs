use std::collections::{HashMap, HashSet};
use winnow::ascii::{dec_uint, multispace0, multispace1};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{PResult, Parser};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 5);

type ParsedInput = (Vec<(u32, u32)>, Vec<Vec<u32>>);
type ProcessedInput = (Vec<Vec<u32>>, HashMap<u32, HashSet<u32>>);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &mut &str) -> PResult<ParsedInput> {
    terminated(
        separated_pair(
            separated(0.., separated_pair(dec_uint, '|', dec_uint), multispace1),
            multispace1,
            separated::<_, Vec<u32>, _, _, _, _, _>(
                1..,
                separated(1.., dec_uint::<_, u32, _>, ','),
                multispace1,
            ),
        ),
        multispace0,
    )
    .parse_next(input)
}

fn process_input(input: &'static str) -> ProcessedInput {
    let parsed_input = parse_input
        .parse(input)
        .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"));

    let mut after_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (before, after) in &parsed_input.0 {
        after_map.entry(*after).or_default().insert(*before);
    }

    (parsed_input.1, after_map)
}

fn is_sorted(update: &[u32], after_map: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut after_pages = update[1..update.len()]
        .iter()
        .copied()
        .collect::<HashSet<_>>();

    for page in update {
        if let Some(before) = after_map.get(page) {
            if !after_pages.is_disjoint(before) {
                return false;
            }
        }
        after_pages.remove(page);
    }

    true
}

fn part1((updates, after_map): &ProcessedInput) -> u32 {
    updates
        .iter()
        .filter(|&update| is_sorted(update, after_map))
        .map(|update| update[update.len() / 2])
        .sum()
}

fn sort(update: &mut [u32], after_map: &HashMap<u32, HashSet<u32>>) {
    let pages = update.iter().copied().collect::<HashSet<_>>();

    let mut filtered_after_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    for page in update.iter() {
        if let Some(after) = after_map.get(page) {
            filtered_after_map.insert(*page, after.intersection(&pages).copied().collect());
        }
    }

    update.sort_by_key(|&page| {
        std::cmp::Reverse(filtered_after_map.get(&page).map_or(0, HashSet::len))
    });
}

fn part2((updates, after_map): &ProcessedInput) -> u32 {
    updates
        .iter()
        .filter(|&update| !is_sorted(update, after_map))
        .cloned()
        .map(|mut update| {
            sort(&mut update, after_map);
            update[update.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> ProcessedInput {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: ProcessedInput) {
        assert_eq!(part1(&data), 143);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2(&data), 123);
    }
}
