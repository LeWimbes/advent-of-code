use std::ops::Range;

use aoc_utils::Solution;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;

struct Day05;

impl Solution for Day05 {
    type Input<'a> = (Vec<u64>, Vec<Mapping>);
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        let mut blocks_it = input.split("\n\n");
        let seeds: Vec<u64> = blocks_it
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        let mappings: Vec<Mapping> = blocks_it
            .map(|block| {
                let mappings = block
                    .lines()
                    .skip(1)
                    .map(|mapping| {
                        let nums: Vec<u64> = mapping
                            .split_whitespace()
                            .map(|num| num.parse().unwrap())
                            .collect();
                        (nums[1]..nums[1] + nums[2], nums[0]..nums[0] + nums[2])
                    })
                    .collect();
                Mapping::from(mappings)
            })
            .collect();

        (seeds, mappings)
    }

    fn part1((seeds, mappings): &Self::Input<'_>) -> Self::Output1 {
        seeds
            .iter()
            .map(|seed| {
                mappings
                    .iter()
                    .fold(*seed, |value, mapping| mapping.map(value))
            })
            .min()
            .unwrap()
    }

    fn part2((seeds, mappings): &Self::Input<'_>) -> Self::Output2 {
        seeds
            .par_chunks_exact(2)
            .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
            .map(|seed| {
                mappings
                    .iter()
                    .fold(seed, |value, mapping| mapping.map(value))
            })
            .min()
            .unwrap()
    }
}

aoc_utils::run!(2023, 5, Day05);

struct Mapping {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl Mapping {
    pub fn from(mappings: Vec<(Range<u64>, Range<u64>)>) -> Self {
        Mapping { mappings }
    }

    pub fn map(&self, value: u64) -> u64 {
        for mapping in &self.mappings {
            if mapping.0.contains(&value) {
                let offset = value - mapping.0.start;
                return mapping.1.start + offset;
            }
        }
        value
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 5, Day05);

    #[rstest]
    fn part1_test(data1: <Day05 as Solution>::Input<'_>) {
        assert_eq!(35, Day05::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day05 as Solution>::Input<'_>) {
        assert_eq!(46, Day05::part2(&data1));
    }
}
