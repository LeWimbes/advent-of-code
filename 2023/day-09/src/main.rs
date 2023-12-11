use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &str) -> Vec<Vec<i64>> {
    input.lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect()
        }).collect()
}

fn part1(sequences: &Vec<Vec<i64>>) -> i64 {
    sequences.par_iter().map(|sequence| {
        let mut nums = sequence.clone();
        let mut last_nums: Vec<i64> = Vec::new();

        while nums.iter().any(|distance| distance != &0) {
            last_nums.push(nums.last().unwrap().clone());
            nums = nums.iter()
                .tuple_windows()
                .map(|(left, right)| right - left)
                .collect();
        }
        last_nums.iter().fold(0, |acc, num| acc + num)
    }).sum()
}

fn part2(sequences: &Vec<Vec<i64>>) -> i64 {
    sequences.par_iter().map(|sequence| {
        let mut nums = sequence.clone();
        let mut first_nums: Vec<i64> = Vec::new();

        while nums.iter().any(|distance| distance != &0) {
            first_nums.push(nums.first().unwrap().clone());
            nums = nums.iter()
                .tuple_windows()
                .map(|(left, right)| right - left)
                .collect();
        }

        first_nums.iter().rev().fold(0, |acc, num| num - acc)
    }).sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn data() -> Vec<Vec<i64>> {
        let input = include_str!("input_test.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<Vec<i64>>) {
        assert_eq!(part1(&data), 114);
    }

    #[rstest]
    fn part2_test(data: Vec<Vec<i64>>) {
        assert_eq!(part2(&data), 2);
    }
}
