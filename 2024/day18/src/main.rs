use aoc_utils::Solution;
use std::cmp::Ordering;
use std::collections::VecDeque;
use winnow::ascii::{dec_uint, multispace0, multispace1};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{Parser, Result};

struct Day18;

impl Solution for Day18 {
    type Input<'a> = Vec<(usize, usize)>;
    type Output1 = u64;
    type Output2 = String;

    fn process(input: &str) -> Self::Input<'_> {
        parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
    }

    fn part1(bytes: &Self::Input<'_>) -> Self::Output1 {
        part1_parameterized(bytes, 71, 1024)
    }

    fn part2(bytes: &Self::Input<'_>) -> Self::Output2 {
        part2_parameterized(bytes, 71)
    }
}

aoc_utils::run!(2024, 18, Day18);

fn parse_input<'a>(input: &mut &str) -> Result<<Day18 as Solution>::Input<'a>> {
    terminated(
        separated(1.., separated_pair(dec_uint, ',', dec_uint), multispace1),
        multispace0,
    )
    .parse_next(input)
}

fn bfs(bytes: &<Day18 as Solution>::Input<'_>, size: usize, byte_count: usize) -> Option<u64> {
    let max = size - 1;

    let mut map = vec![vec![true; size]; size];

    for byte in bytes.iter().take(byte_count) {
        map[byte.1][byte.0] = false;
    }

    let mut visited = vec![vec![false; size]; size];

    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));

    while let Some(((x, y), steps)) = queue.pop_front() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;

        if x == max && y == max {
            return Some(steps);
        }

        if x > 0 && map[y][x - 1] {
            queue.push_back(((x - 1, y), steps + 1));
        }
        if y > 0 && map[y - 1][x] {
            queue.push_back(((x, y - 1), steps + 1));
        }
        if x < max && map[y][x + 1] {
            queue.push_back(((x + 1, y), steps + 1));
        }
        if y < max && map[y + 1][x] {
            queue.push_back(((x, y + 1), steps + 1));
        }
    }

    None
}

fn part1_parameterized(
    bytes: &<Day18 as Solution>::Input<'_>,
    size: usize,
    byte_count: usize,
) -> u64 {
    bfs(bytes, size, byte_count).unwrap()
}

fn part2_parameterized(bytes: &<Day18 as Solution>::Input<'_>, size: usize) -> String {
    let limit = (0..bytes.len()).collect::<Vec<_>>().binary_search_by(|i| {
        if bfs(bytes, size, *i).is_some() {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });

    let pos = limit.unwrap_err();

    format!("{},{}", bytes[pos - 1].0, bytes[pos - 1].1)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // Note: The automatic test input fetching retrieves incorrect data.
    aoc_utils::solution_tests!(2024, 18, Day18);

    #[rstest]
    fn part1_test(data1: <Day18 as Solution>::Input<'_>) {
        assert_eq!(22, part1_parameterized(&data1, 7, 12));
    }

    #[rstest]
    fn part2_test(data1: <Day18 as Solution>::Input<'_>) {
        assert_eq!("6,1", part2_parameterized(&data1, 7));
    }
}
