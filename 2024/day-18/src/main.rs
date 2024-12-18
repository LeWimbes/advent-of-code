use std::cmp::Ordering;
use std::collections::VecDeque;
use winnow::ascii::{dec_uint, multispace0, multispace1};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{PResult, Parser};

type ParsedInput = Vec<(usize, usize)>;
type ProcessedInput = Vec<(usize, usize)>;

fn main() {
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data, 71, 1024));
    println!("Part2: {}", part2(&data, 71));
}

fn parse_input(input: &mut &str) -> PResult<ParsedInput> {
    terminated(
        separated(1.., separated_pair(dec_uint, ',', dec_uint), multispace1),
        multispace0,
    )
    .parse_next(input)
}

fn process_input(input: &str) -> ProcessedInput {
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

fn part1(bytes: &ProcessedInput, size: usize, byte_count: usize) -> u64 {
    bfs(bytes, size, byte_count).unwrap()
}

fn part2(bytes: &ProcessedInput, size: usize) -> String {
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
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn data() -> ProcessedInput {
        let input = include_str!("input_test.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: ProcessedInput) {
        assert_eq!(part1(&data, 7, 12), 22);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2(&data, 7), "6,1");
    }
}
