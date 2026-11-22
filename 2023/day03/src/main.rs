use std::collections::{HashMap, HashSet};

use aoc_utils::Solution;
use aoc_utils::grid::{Grid, Pos};
use regex::Regex;

struct Day03;

impl Solution for Day03 {
    type Input<'a> = HashMap<Pos, Vec<u32>>;
    type Output1 = u32;
    type Output2 = u32;

    fn process(input: &str) -> Self::Input<'_> {
        let re = Regex::new(r"\d+").unwrap();
        let grid = Grid::parse(input, |c| c);
        let is_symbol = |c: char| c != '.' && !c.is_ascii_digit();

        let mut symbols: HashMap<Pos, Vec<u32>> = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for m in re.find_iter(line) {
                let value: u32 = m.as_str().parse().unwrap();

                // The symbols adjacent to any digit of the number, each only once
                let adjacent: HashSet<Pos> = (m.start()..m.end())
                    .flat_map(|x| grid.neighbors8(Pos::new(x, y)))
                    .filter(|&pos| is_symbol(grid[pos]))
                    .collect();

                for pos in adjacent {
                    symbols.entry(pos).or_default().push(value);
                }
            }
        }

        symbols
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
