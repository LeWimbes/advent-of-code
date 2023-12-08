use std::ops::Deref;

use itertools::Itertools;

use crate::HandType::*;

fn main() {
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &str) -> Vec<(Vec<u32>, u64)> {
    input.lines()
        .map(|line| {
            let parts = line.split_once(" ").unwrap();
            (parts.0.chars().map(|char| {
                match char {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'J' => 11,
                    'T' => 10,
                    _ => char.to_digit(10).unwrap()
                }
            }).collect(), parts.1.parse().unwrap())
        }).collect()
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

fn get_hand_type(hand: &Vec<u32>, with_joker: bool) -> HandType {
    let mut counts = hand.iter().counts();
    let jokers = if with_joker {
        counts.remove(&11).unwrap_or(0)
    } else {
        0
    };

    let mut sorted_counts: Vec<_> = counts.values().sorted_by_key(|&&count| std::cmp::Reverse(count)).cloned().collect();
    if jokers == 5 {
        sorted_counts.push(jokers);
    } else {
        sorted_counts[0] = sorted_counts[0] + jokers;
    }

    let counts_string = sorted_counts.iter().join("");
    match counts_string.deref() {
        "5" => FiveOfAKind,
        "41" => FourOfAKind,
        "32" => FullHouse,
        "311" => ThreeOfAKind,
        "221" => TwoPair,
        "2111" => OnePair,
        _ => HighCard
    }
}

fn part1(hands: &Vec<(Vec<u32>, u64)>) -> u64 {
    hands.iter().map(|(hand, bid)| {
        (hand, bid, get_hand_type(hand, false))
    }).sorted_by_key(|hand| (hand.2, hand.0))
        .enumerate()
        .map(|(i, (_hand, bid, _hand_type))| {
            (i as u64 + 1) * bid
        }).sum()
}

fn part2(hands: &Vec<(Vec<u32>, u64)>) -> u64 {
    hands.iter().map(|(hand, bid)| {
        (hand, bid, get_hand_type(hand, true))
    }).sorted_by_key(|hand| (hand.2, hand.0.iter().map(|card| {
        match card {
            11 => 1,
            _ => *card
        }
    }).collect::<Vec<_>>()))
        .enumerate()
        .map(|(i, (_hand, bid, _hand_type))| {
            (i as u64 + 1) * bid
        }).sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn data() -> Vec<(Vec<u32>, u64)> {
        let input = include_str!("input_test.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<(Vec<u32>, u64)>) {
        assert_eq!(part1(&data), 6440);
    }

    #[rstest]
    fn part2_test(data: Vec<(Vec<u32>, u64)>) {
        assert_eq!(part2(&data), 5905);
    }
}
