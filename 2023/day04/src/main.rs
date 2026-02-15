use std::collections::HashSet;

use aoc_utils::Solution;
use regex::Regex;

struct Day04;

impl Solution for Day04 {
    type Input<'a> = Vec<(Vec<u32>, Vec<u32>)>;
    type Output1 = u32;
    type Output2 = u32;

    fn process(input: &str) -> Self::Input<'_> {
        let re =
            Regex::new(r"Card\s+\d+:\s+(?<winning>[\d\s]+)\s+\|\s+(?<found>[\d\s]+)\s*").unwrap();
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

    fn part1(cards: &Self::Input<'_>) -> Self::Output1 {
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

    fn part2(cards: &Self::Input<'_>) -> Self::Output2 {
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
}

aoc_utils::run!(2023, 4, Day04);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 4, Day04);

    #[rstest]
    fn part1_test(data1: <Day04 as Solution>::Input<'_>) {
        assert_eq!(13, Day04::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day04 as Solution>::Input<'_>) {
        assert_eq!(30, Day04::part2(&data1));
    }
}
