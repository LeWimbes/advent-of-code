use itertools::{Itertools, repeat_n};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

// After using brute-force for part1, this solution is inspired by https://www.reddit.com/r/adventofcode/comments/18ge41g/comment/kd18cl9/

fn main() {
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &str) -> Vec<(String, Vec<u64>)> {
    input.lines().map(|line| {
        let parts = line.split_once(" ").unwrap();

        let group_sizes = parts.1.split(",").map(|num| num.parse().unwrap()).collect();

        (parts.0.to_string(), group_sizes)
    }).collect()
}

fn unfold_and_prepare(springs: &Vec<(String, Vec<u64>)>, folds: usize) -> Vec<(Vec<char>, Vec<bool>)> {
    springs.par_iter().map(|(springs, group_sizes)| {
        let springs_string: Vec<char> = std::iter::once('.')
            .chain(repeat_n(springs, folds).join("?").chars())
            .chain(std::iter::once('.'))
            .collect();

        let new_group_sizes: Vec<_> = std::iter::once(false)
            .chain(repeat_n(group_sizes.iter(), folds).flatten()
                .map(|group_size| vec![true; *group_size as usize])
                .intersperse(vec![false]).flatten())
            .chain(std::iter::once(false))
            .collect();

        (springs_string, new_group_sizes)
    }).collect()
}

fn count_possible_arrangements(springs: &Vec<(Vec<char>, Vec<bool>)>) -> u64 {
    springs.par_iter().map(|(springs, groups)| {
        let n = springs.len();
        let m = groups.len();
        let mut dp = vec![vec![0u64; m + 1]; n + 1];
        dp[n][m] = 1;

        (0..n).rev().for_each(|i| {
            (m.saturating_sub(n - i)..m).rev().for_each(|j| {
                let mut damaged = false;
                let mut operational = false;

                match springs[i] {
                    '#' => damaged = true,
                    '.' => operational = true,
                    _ => {
                        operational = true;
                        damaged = true;
                    }
                }

                let mut sum = 0;
                if damaged && groups[j] {
                    sum += dp[i + 1][j + 1];
                } else if operational && !groups[j] {
                    sum += dp[i + 1][j + 1] + dp[i + 1][j];
                }
                dp[i][j] = sum;
            });
        });

        dp[0][0]
    }).sum()
}

fn part1(springs: &Vec<(String, Vec<u64>)>) -> u64 {
    count_possible_arrangements(&unfold_and_prepare(springs, 1))
}

fn part2(springs: &Vec<(String, Vec<u64>)>) -> u64 {
    count_possible_arrangements(&unfold_and_prepare(springs, 5))
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn data() -> Vec<(String, Vec<u64>)> {
        let input = include_str!("input_test.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<(String, Vec<u64>)>) {
        assert_eq!(part1(&data), 21);
    }

    #[rstest]
    fn part2_test(data: Vec<(String, Vec<u64>)>) {
        assert_eq!(part2(&data), 525152);
    }
}
