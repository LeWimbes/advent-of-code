use std::collections::HashSet;

use regex::Regex;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 4);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &'static str) -> Vec<(Vec<u32>, Vec<u32>)> {
    let re = Regex::new(r"Card\s+\d+:\s+(?<winning>[\d\s]+)\s+\|\s+(?<found>[\d\s]+)\s*").unwrap();
    re.captures_iter(input)
        .map(|caps| {
            let winning = caps["winning"]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            let found = caps["found"]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();

            (winning, found)
        })
        .collect()
}

fn part1(cards: &[(Vec<u32>, Vec<u32>)]) -> u32 {
    cards
        .iter()
        .map(|card| {
            let wins = card
                .0
                .iter()
                .collect::<HashSet<_>>()
                .intersection(&card.1.iter().collect::<HashSet<_>>())
                .count();
            if wins == 0 {
                0
            } else {
                2u32.pow(wins as u32 - 1)
            }
        })
        .sum()
}

fn part2(cards: &[(Vec<u32>, Vec<u32>)]) -> u32 {
    let card_wins: Vec<usize> = cards
        .iter()
        .map(|card| {
            card.0
                .iter()
                .collect::<HashSet<_>>()
                .intersection(&card.1.iter().collect::<HashSet<_>>())
                .count()
        })
        .collect();
    let mut card_counts = vec![1u32; cards.len()];

    for i in 0..cards.len() {
        for j in (i + 1)..(i + 1 + card_wins[i]).min(cards.len()) {
            card_counts[j] += card_counts[i];
        }
    }

    card_counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> Vec<(Vec<u32>, Vec<u32>)> {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<(Vec<u32>, Vec<u32>)>) {
        assert_eq!(part1(&data), 13);
    }

    #[rstest]
    fn part2_test(data: Vec<(Vec<u32>, Vec<u32>)>) {
        assert_eq!(part2(&data), 30);
    }
}
