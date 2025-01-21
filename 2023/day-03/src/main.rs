use std::collections::{HashMap, HashSet};

use regex::Regex;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 3);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &'static str) -> HashMap<(usize, usize), Vec<u32>> {
    let re = Regex::new(r"\d+").unwrap();
    let non_chars = HashSet::from(['.', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9']);

    let mut chars: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    let lines: Vec<&str> = input.lines().collect();
    let grid: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    lines.iter().enumerate().for_each(|(row, line)| {
        for m in re.find_iter(line) {
            let value: u32 = m.as_str().parse().unwrap();

            for r in row.saturating_sub(1)..(row + 2).min(grid.len()) {
                for c in m.start().saturating_sub(1)..(m.end() + 1).min(grid[row].len()) {
                    if !non_chars.contains(&grid[r][c]) {
                        chars.entry((r, c)).or_default().push(value);
                    }
                }
            }
        }
    });

    chars
}

fn part1(chars: &HashMap<(usize, usize), Vec<u32>>) -> u32 {
    chars
        .iter()
        .map(|(_key, value)| value.iter().sum::<u32>())
        .sum()
}

fn part2(chars: &HashMap<(usize, usize), Vec<u32>>) -> u32 {
    chars
        .iter()
        .filter_map(|(_key, value)| {
            if value.len() == 2 {
                Some(value.iter().product::<u32>())
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> HashMap<(usize, usize), Vec<u32>> {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: HashMap<(usize, usize), Vec<u32>>) {
        assert_eq!(part1(&data), 4361);
    }

    #[rstest]
    fn part2_test(data: HashMap<(usize, usize), Vec<u32>>) {
        assert_eq!(part2(&data), 467_835);
    }
}
