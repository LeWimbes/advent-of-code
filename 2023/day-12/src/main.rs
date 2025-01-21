use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 12);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

#[derive(Copy, Clone)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl Spring {
    fn is_operational(self) -> bool {
        matches!(self, Spring::Operational | Spring::Unknown)
    }

    fn is_damaged(self) -> bool {
        matches!(self, Spring::Damaged | Spring::Unknown)
    }
}

fn process_input(input: &'static str) -> Vec<(Vec<Spring>, Vec<bool>)> {
    input
        .lines()
        .map(|line| {
            let parts = line.split_once(' ').unwrap();

            let springs = parts
                .0
                .chars()
                .map(|c| match c {
                    '.' => Spring::Operational,
                    '#' => Spring::Damaged,
                    '?' => Spring::Unknown,
                    _ => panic!("Unknown spring '{c}'"),
                })
                .collect();

            let broken = parts
                .1
                .split(',')
                .map(|num| {
                    let group_size: usize = num.parse().unwrap();
                    std::iter::repeat(true).take(group_size)
                })
                .intersperse(std::iter::repeat(false).take(1))
                .flatten()
                .collect();

            (springs, broken)
        })
        .collect()
}

fn unfold_and_prepare(
    springs: &Vec<(Vec<Spring>, Vec<bool>)>,
    folds: usize,
) -> Vec<(Vec<Spring>, Vec<bool>)> {
    springs
        .par_iter()
        .map(|(springs, broken)| {
            let unfolded_springs = std::iter::once(Spring::Operational)
                .chain(
                    std::iter::repeat(springs.iter().copied())
                        .take(folds)
                        .intersperse([Spring::Unknown].iter().copied())
                        .flatten(),
                )
                .chain(std::iter::once(Spring::Operational))
                .collect();

            let unfolded_broken = std::iter::once(false)
                .chain(
                    std::iter::repeat(broken.iter().copied())
                        .take(folds)
                        .intersperse([false].iter().copied())
                        .flatten(),
                )
                .chain(std::iter::once(false))
                .collect();

            (unfolded_springs, unfolded_broken)
        })
        .collect()
}

fn count_possible_arrangements(springs: &Vec<(Vec<Spring>, Vec<bool>)>) -> u64 {
    springs
        .par_iter()
        .map(|(springs, broken)| {
            let n = springs.len();
            let m = broken.len();

            // [i][j] represents the number of valid configurations for springs[i..n] and broken[j..m]
            let mut possible_arrangements = vec![vec![0u64; m + 1]; n + 1];
            possible_arrangements[n][m] = 1;

            for i in (0..n).rev() {
                let spring = &springs[i];

                for j in (m.saturating_sub(n - i)..m).rev() {
                    possible_arrangements[i][j] = if spring.is_damaged() && broken[j] {
                        // The current spring is damaged, and the broken sequence expects a damaged spring.
                        // Move to the next spring and the next expectation.
                        possible_arrangements[i + 1][j + 1]
                    } else if spring.is_operational() && !broken[j] {
                        // The current spring is operational, and the broken sequence expects an operational spring.
                        // There are two options:
                        // - The next spring is part of the same group (operational).
                        // - The next spring starts a new group (broken).
                        possible_arrangements[i + 1][j + 1] + possible_arrangements[i + 1][j]
                    } else {
                        // The current spring and the expectation do not match.
                        0
                    };
                }
            }

            possible_arrangements[0][0]
        })
        .sum()
}

fn part1(springs: &Vec<(Vec<Spring>, Vec<bool>)>) -> u64 {
    count_possible_arrangements(&unfold_and_prepare(springs, 1))
}

fn part2(springs: &Vec<(Vec<Spring>, Vec<bool>)>) -> u64 {
    count_possible_arrangements(&unfold_and_prepare(springs, 5))
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> Vec<(Vec<Spring>, Vec<bool>)> {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<(Vec<Spring>, Vec<bool>)>) {
        assert_eq!(part1(&data), 21);
    }

    #[rstest]
    fn part2_test(data: Vec<(Vec<Spring>, Vec<bool>)>) {
        assert_eq!(part2(&data), 525_152);
    }
}
