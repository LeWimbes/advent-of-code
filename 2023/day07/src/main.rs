use aoc_utils::Solution;
use itertools::Itertools;

struct Day07;

impl Solution for Day07 {
    type Input<'a> = Vec<(Vec<u32>, u64)>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        input
            .lines()
            .map(|line| {
                let parts = line.split_once(' ').unwrap();
                (
                    parts
                        .0
                        .chars()
                        .map(|char| match char {
                            'A' => 14,
                            'K' => 13,
                            'Q' => 12,
                            'J' => 11,
                            'T' => 10,
                            _ => char.to_digit(10).unwrap(),
                        })
                        .collect(),
                    parts.1.parse().unwrap(),
                )
            })
            .collect()
    }

    fn part1(hands: &Self::Input<'_>) -> Self::Output1 {
        hands
            .iter()
            .map(|(hand, bid)| (hand, bid, get_hand_type(hand, false)))
            .sorted_by_key(|hand| (hand.2, hand.0))
            .enumerate()
            .map(|(i, (_hand, bid, _hand_type))| (i as u64 + 1) * bid)
            .sum()
    }

    fn part2(hands: &Self::Input<'_>) -> Self::Output2 {
        hands
            .iter()
            .map(|(hand, bid)| (hand, bid, get_hand_type(hand, true)))
            .sorted_by_key(|hand| {
                (
                    hand.2,
                    hand.0
                        .iter()
                        .map(|card| match card {
                            11 => 1,
                            _ => *card,
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .enumerate()
            .map(|(i, (_hand, bid, _hand_type))| (i as u64 + 1) * bid)
            .sum()
    }
}

aoc_utils::run!(2023, 7, Day07);

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

fn get_hand_type(hand: &[u32], with_joker: bool) -> HandType {
    let mut counts = hand.iter().counts();
    let jokers = if with_joker {
        counts.remove(&11).unwrap_or(0)
    } else {
        0
    };

    let mut sorted_counts: Vec<_> = counts
        .values()
        .sorted_by_key(|&&count| std::cmp::Reverse(count))
        .copied()
        .collect();
    if jokers == 5 {
        sorted_counts.push(jokers);
    } else {
        sorted_counts[0] += jokers;
    }

    let counts_string = sorted_counts.iter().join("");
    match &*counts_string {
        "5" => HandType::FiveOfAKind,
        "41" => HandType::FourOfAKind,
        "32" => HandType::FullHouse,
        "311" => HandType::ThreeOfAKind,
        "221" => HandType::TwoPair,
        "2111" => HandType::OnePair,
        _ => HandType::HighCard,
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 7, Day07);

    #[rstest]
    fn part1_test(data1: <Day07 as Solution>::Input<'_>) {
        assert_eq!(6440, Day07::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day07 as Solution>::Input<'_>) {
        assert_eq!(5905, Day07::part2(&data1));
    }
}
