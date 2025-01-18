use std::collections::HashMap;
use winnow::ascii::{dec_uint, multispace0, multispace1, space1};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{PResult, Parser};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 1);

type ParsedInput = Vec<(u32, u32)>;
type ProcessedInput = (Vec<u32>, Vec<u32>);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &mut &str) -> PResult<ParsedInput> {
    terminated(
        separated(0.., separated_pair(dec_uint, space1, dec_uint), multispace1),
        multispace0,
    )
    .parse_next(input)
}

fn process_input(input: &'static str) -> ProcessedInput {
    parse_input
        .parse(input)
        .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
        .into_iter()
        .unzip()
}

fn part1(lists: &ProcessedInput) -> u32 {
    let mut list0 = lists.0.clone();
    let mut list1 = lists.1.clone();
    list0.sort_unstable();
    list1.sort_unstable();

    list0
        .into_iter()
        .zip(list1)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn part2(lists: &ProcessedInput) -> u32 {
    let mut frequency_map: HashMap<u32, u32> = HashMap::new();

    for &num in &lists.1 {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    lists
        .0
        .iter()
        .map(|&num| num * *frequency_map.entry(num).or_insert(0))
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
        assert_eq!(part1(&data), 11);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2(&data), 31);
    }
}
