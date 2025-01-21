use std::ops::Range;

use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 5);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

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

fn process_input(input: &'static str) -> (Vec<u64>, Vec<Mapping>) {
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

fn part1(seeds_mappings: &(Vec<u64>, Vec<Mapping>)) -> u64 {
    seeds_mappings
        .0
        .iter()
        .map(|seed| {
            seeds_mappings
                .1
                .iter()
                .fold(*seed, |value, mapping| mapping.map(value))
        })
        .min()
        .unwrap()
}

fn part2(seeds_mappings: &(Vec<u64>, Vec<Mapping>)) -> u64 {
    seeds_mappings
        .0
        .par_chunks_exact(2)
        .flat_map(|chunk| chunk[0]..chunk[0] + chunk[1])
        .map(|seed| {
            seeds_mappings
                .1
                .iter()
                .fold(seed, |value, mapping| mapping.map(value))
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> (Vec<u64>, Vec<Mapping>) {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: (Vec<u64>, Vec<Mapping>)) {
        assert_eq!(part1(&data), 35);
    }

    #[rstest]
    fn part2_test(data: (Vec<u64>, Vec<Mapping>)) {
        assert_eq!(part2(&data), 46);
    }
}
