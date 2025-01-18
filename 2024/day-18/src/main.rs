use std::cmp::Ordering;
use std::collections::VecDeque;
use winnow::ascii::{dec_uint, multispace0, multispace1};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{PResult, Parser};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 18);

type ParsedInput = Vec<(usize, usize)>;
type ProcessedInput = Vec<(usize, usize)>;

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &mut &str) -> PResult<ParsedInput> {
    terminated(
        separated(1.., separated_pair(dec_uint, ',', dec_uint), multispace1),
        multispace0,
    )
    .parse_next(input)
}

fn process_input(input: &'static str) -> ProcessedInput {
    parse_input
        .parse(input)
        .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
}

fn bfs(bytes: &ProcessedInput, size: usize, byte_count: usize) -> Option<u64> {
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

fn part1_parameterized(bytes: &ProcessedInput, size: usize, byte_count: usize) -> u64 {
    bfs(bytes, size, byte_count).unwrap()
}

fn part1(bytes: &ProcessedInput) -> u64 {
    part1_parameterized(bytes, 71, 1024)
}

fn part2_parameterized(bytes: &ProcessedInput, size: usize) -> String {
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

fn part2(bytes: &ProcessedInput) -> String {
    part2_parameterized(bytes, 71)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> ProcessedInput {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: ProcessedInput) {
        assert_eq!(part1_parameterized(&data, 7, 12), 22);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2_parameterized(&data, 7), "6,1");
    }
}
