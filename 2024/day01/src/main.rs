use aoc_utils::Solution;
use std::collections::HashMap;
use winnow::ascii::{dec_uint, multispace0, multispace1, space1};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{Parser, Result};

struct Day01;

impl Solution for Day01 {
    type Input<'a> = (Vec<u32>, Vec<u32>);
    type Output1 = u32;
    type Output2 = u32;

    fn process(input: &str) -> Self::Input<'_> {
        parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
            .into_iter()
            .unzip()
    }

    fn part1(lists: &Self::Input<'_>) -> Self::Output1 {
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

    fn part2(lists: &Self::Input<'_>) -> Self::Output2 {
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
}

aoc_utils::run!(2024, 1, Day01);

fn parse_input(input: &mut &str) -> Result<Vec<(u32, u32)>> {
    terminated(
        separated(0.., separated_pair(dec_uint, space1, dec_uint), multispace1),
        multispace0,
    )
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 1, Day01);

    #[rstest]
    fn part1_test(data1: <Day01 as Solution>::Input<'_>) {
        assert_eq!(11, Day01::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day01 as Solution>::Input<'_>) {
        assert_eq!(31, Day01::part2(&data1));
    }
}
