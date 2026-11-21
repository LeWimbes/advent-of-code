use std::collections::{HashMap, HashSet};

use aoc_utils::Solution;
use regex::Regex;

struct Day03;

impl Solution for Day03 {
    type Input<'a> = HashMap<(usize, usize), Vec<u32>>;
    type Output1 = u32;
    type Output2 = u32;

    fn process(input: &str) -> Self::Input<'_> {
        let re = Regex::new(r"\d+").unwrap();
        let non_chars = HashSet::from(['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

        let mut chars: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

        let lines: Vec<&str> = input.lines().collect();
        let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
        lines.iter().enumerate().for_each(|(row, line)| {
            for m in re.find_iter(line) {
                let value: u32 = m.as_str().parse().unwrap();

                // Check all cells adjacent to the number, staying within the grid
                for (r, cells) in grid
                    .iter()
                    .enumerate()
                    .take(row + 2)
                    .skip(row.saturating_sub(1))
                {
                    for (c, cell) in cells
                        .iter()
                        .enumerate()
                        .take(m.end() + 1)
                        .skip(m.start().saturating_sub(1))
                    {
                        if !non_chars.contains(cell) {
                            chars.entry((r, c)).or_default().push(value);
                        }
                    }
                }
            }
        });

        chars
    }

    fn part1(chars: &Self::Input<'_>) -> Self::Output1 {
        chars.values().map(|value| value.iter().sum::<u32>()).sum()
    }

    fn part2(chars: &Self::Input<'_>) -> Self::Output2 {
        chars
            .values()
            .filter_map(|value| {
                if value.len() == 2 {
                    Some(value.iter().product::<u32>())
                } else {
                    None
                }
            })
            .sum()
    }
}

aoc_utils::run!(2023, 3, Day03);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 3, Day03, examples = [1]);

    #[rstest]
    fn part1_test(data1: <Day03 as Solution>::Input<'_>) {
        assert_eq!(4361, Day03::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day03 as Solution>::Input<'_>) {
        assert_eq!(467_835, Day03::part2(&data1));
    }
}
