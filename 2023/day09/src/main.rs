use aoc_utils::Solution;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

struct Day09;

impl Solution for Day09 {
    type Input<'a> = Vec<Vec<i64>>;

    type Output1 = i64;

    type Output2 = i64;

    fn process(input: &str) -> Self::Input<'_> {
        input
            .lines()
            .map(|line| {
                line.split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part1(sequences: &Self::Input<'_>) -> Self::Output1 {
        sequences
            .par_iter()
            .map(|sequence| {
                let mut nums = sequence.clone();
                let mut last_nums: Vec<i64> = Vec::new();

                while nums.iter().any(|distance| distance != &0) {
                    last_nums.push(*nums.last().unwrap());
                    nums = nums
                        .iter()
                        .tuple_windows()
                        .map(|(left, right)| right - left)
                        .collect();
                }
                last_nums.iter().sum::<i64>()
            })
            .sum()
    }

    fn part2(sequences: &Self::Input<'_>) -> Self::Output2 {
        sequences
            .par_iter()
            .map(|sequence| {
                let mut nums = sequence.clone();
                let mut first_nums: Vec<i64> = Vec::new();

                while nums.iter().any(|distance| distance != &0) {
                    first_nums.push(*nums.first().unwrap());
                    nums = nums
                        .iter()
                        .tuple_windows()
                        .map(|(left, right)| right - left)
                        .collect();
                }

                first_nums.iter().rev().fold(0, |acc, num| num - acc)
            })
            .sum()
    }
}

aoc_utils::run!(2023, 9, Day09);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 9, Day09);

    #[rstest]
    fn part1_test(data1: <Day09 as Solution>::Input<'_>) {
        assert_eq!(114, Day09::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day09 as Solution>::Input<'_>) {
        assert_eq!(2, Day09::part2(&data1));
    }
}
