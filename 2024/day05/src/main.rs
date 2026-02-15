use aoc_utils::Solution;
use std::collections::{HashMap, HashSet};
use winnow::ascii::{dec_uint, multispace0, multispace1};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{Parser, Result};

struct Day05;

impl Solution for Day05 {
    type Input<'a> = (Vec<Vec<u32>>, HashMap<u32, HashSet<u32>>);
    type Output1 = u32;
    type Output2 = u32;

    fn process(input: &str) -> Self::Input<'_> {
        let parsed_input = parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"));
        let mut after_map: HashMap<u32, HashSet<u32>> = HashMap::new();
        for (before, after) in &parsed_input.0 {
            after_map.entry(*after).or_default().insert(*before);
        }

        (parsed_input.1, after_map)
    }

    fn part1((updates, after_map): &Self::Input<'_>) -> Self::Output1 {
        updates
            .iter()
            .filter(|&update| is_sorted(update, after_map))
            .map(|update| update[update.len() / 2])
            .sum()
    }

    fn part2((updates, after_map): &Self::Input<'_>) -> Self::Output2 {
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
}

aoc_utils::run!(2024, 5, Day05);

type ParsedInput = (Vec<(u32, u32)>, Vec<Vec<u32>>);
fn parse_input(input: &mut &str) -> Result<ParsedInput> {
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

fn is_sorted(update: &[u32], after_map: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut after_pages = update[1..update.len()]
        .iter()
        .copied()
        .collect::<HashSet<_>>();

    for page in update {
        if let Some(before) = after_map.get(page)
            && !after_pages.is_disjoint(before)
        {
            return false;
        }
        after_pages.remove(page);
    }

    true
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 5, Day05);

    #[rstest]
    fn part1_test(data1: <Day05 as Solution>::Input<'_>) {
        assert_eq!(143, Day05::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day05 as Solution>::Input<'_>) {
        assert_eq!(123, Day05::part2(&data1));
    }
}
