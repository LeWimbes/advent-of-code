use aoc_utils::Solution;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

struct Day12;

impl Solution for Day12 {
    type Input<'a> = Vec<(Vec<Spring>, Vec<bool>)>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
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

                let broken = Itertools::intersperse(
                    parts.1.split(',').map(|num| {
                        let group_size: usize = num.parse().unwrap();
                        std::iter::repeat_n(true, group_size)
                    }),
                    std::iter::repeat_n(false, 1),
                )
                .flatten()
                .collect();

                (springs, broken)
            })
            .collect()
    }

    fn part1(springs: &Self::Input<'_>) -> Self::Output1 {
        count_possible_arrangements(&unfold_and_prepare(springs, 1))
    }

    fn part2(springs: &Self::Input<'_>) -> Self::Output2 {
        count_possible_arrangements(&unfold_and_prepare(springs, 5))
    }
}

aoc_utils::run!(2023, 12, Day12);

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

fn unfold_and_prepare<'a>(
    springs: &<Day12 as Solution>::Input<'a>,
    folds: usize,
) -> <Day12 as Solution>::Input<'a> {
    springs
        .par_iter()
        .map(|(springs, broken)| {
            let unfolded_springs = std::iter::once(Spring::Operational)
                .chain(
                    Itertools::intersperse(
                        std::iter::repeat_n(springs.iter().copied(), folds),
                        [Spring::Unknown].iter().copied(),
                    )
                    .flatten(),
                )
                .chain(std::iter::once(Spring::Operational))
                .collect();

            let unfolded_broken = std::iter::once(false)
                .chain(
                    Itertools::intersperse(
                        std::iter::repeat_n(broken.iter().copied(), folds),
                        [false].iter().copied(),
                    )
                    .flatten(),
                )
                .chain(std::iter::once(false))
                .collect();

            (unfolded_springs, unfolded_broken)
        })
        .collect()
}

fn count_possible_arrangements(springs: &<Day12 as Solution>::Input<'_>) -> u64 {
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 12, Day12);

    #[rstest]
    fn part1_test(data1: <Day12 as Solution>::Input<'_>) {
        assert_eq!(21, Day12::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day12 as Solution>::Input<'_>) {
        assert_eq!(525_152, Day12::part2(&data1));
    }
}
