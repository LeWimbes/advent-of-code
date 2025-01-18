use std::collections::HashMap;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 11);

type ParsedInput = Vec<u64>;
type ProcessedInput = HashMap<u64, u64>;

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    input
        .trim()
        .split(' ')
        .map(|number| number.parse::<u64>().unwrap())
        .collect()
}

fn process_input(input: &'static str) -> ProcessedInput {
    let stones = parse_input(input);
    let mut stones_map = HashMap::new();

    for stone in stones {
        stones_map
            .entry(stone)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    stones_map
}

fn stones_after_blinks(stones: &ProcessedInput, blinks: usize) -> u64 {
    let mut stones = stones.clone();

    for _ in 0..blinks {
        let mut new_stones = HashMap::new();

        for (stone, count) in stones {
            if stone == 0 {
                new_stones
                    .entry(1)
                    .and_modify(|new_count| *new_count += count)
                    .or_insert(count);
            } else {
                let digits = stone.to_string();
                if digits.len() % 2 == 0 {
                    let mid = digits.len() / 2;
                    new_stones
                        .entry(digits[..mid].parse().unwrap())
                        .and_modify(|new_count| *new_count += count)
                        .or_insert(count);
                    new_stones
                        .entry(digits[mid..].parse().unwrap())
                        .and_modify(|new_count| *new_count += count)
                        .or_insert(count);
                } else {
                    new_stones
                        .entry(stone * 2024)
                        .and_modify(|new_count| *new_count += count)
                        .or_insert(count);
                }
            }
        }

        stones = new_stones;
    }

    stones.values().sum()
}

fn part1(stones: &ProcessedInput) -> u64 {
    stones_after_blinks(stones, 25)
}

fn part2(stones: &ProcessedInput) -> u64 {
    stones_after_blinks(stones, 75)
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
        assert_eq!(part1(&data), 55312);
    }
}
