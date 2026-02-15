use std::collections::HashMap;

use aoc_utils::Solution;

struct Day11;

impl Solution for Day11 {
    type Input<'a> = HashMap<u64, u64>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        let stones: Vec<_> = input
            .trim()
            .split(' ')
            .map(|number| number.parse::<u64>().unwrap())
            .collect();

        let mut stones_map = HashMap::new();

        for stone in stones {
            stones_map
                .entry(stone)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        stones_map
    }

    fn part1(stones: &Self::Input<'_>) -> Self::Output1 {
        stones_after_blinks(stones, 25)
    }

    fn part2(stones: &Self::Input<'_>) -> Self::Output2 {
        stones_after_blinks(stones, 75)
    }
}

aoc_utils::run!(2024, 11, Day11);

fn stones_after_blinks(stones: &<Day11 as Solution>::Input<'_>, blinks: usize) -> u64 {
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // Note: The automatic test input fetching retrieves incorrect data.
    aoc_utils::solution_tests!(2024, 11, Day11);

    #[rstest]
    fn part1_test(data1: <Day11 as Solution>::Input<'_>) {
        assert_eq!(55_312, Day11::part1(&data1));
    }
}
