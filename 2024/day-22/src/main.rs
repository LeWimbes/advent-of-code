use std::collections::{HashMap, HashSet};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 22);

type ParsedInput = Vec<i64>;
type ProcessedInput = Vec<Vec<(i64, i64, i64)>>;

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.parse().unwrap())
        .collect()
}

fn evolve_secret(secret: i64) -> Vec<(i64, i64, i64)> {
    let mut result = vec![(0, 0, 0); 2000];
    let mut new_secret = secret;
    let mut last_price = new_secret % 10;

    for res in &mut result {
        new_secret = ((new_secret * 64) ^ new_secret) % 16_777_216;
        new_secret = ((new_secret / 32) ^ new_secret) % 16_777_216;
        new_secret = ((new_secret * 2048) ^ new_secret) % 16_777_216;

        let new_price = new_secret % 10;
        let change = new_price - last_price;
        last_price = new_price;

        *res = (new_secret, new_price, change);
    }

    result
}

fn process_input(input: &'static str) -> ProcessedInput {
    let secrets = parse_input(input);

    secrets.into_iter().map(evolve_secret).collect()
}

fn part1(secrets: &ProcessedInput) -> i64 {
    secrets.iter().map(|secret| secret.last().unwrap().0).sum()
}

fn part2(secrets: &ProcessedInput) -> i64 {
    let mut bananas: HashMap<(i64, i64, i64, i64), i64> = HashMap::new();

    for secret in secrets {
        let mut seen: HashSet<(i64, i64, i64, i64)> = HashSet::new();

        for i in 3..secret.len() {
            let history = (
                secret[i - 3].2,
                secret[i - 2].2,
                secret[i - 1].2,
                secret[i].2,
            );

            if seen.contains(&history) {
                continue;
            }
            seen.insert(history);

            *bananas.entry(history).or_insert(0) += secret[i].1;
        }
    }

    *bananas.values().max().unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data1() -> ProcessedInput {
        let input = include_str!("test_input1.txt");
        process_input(input)
    }

    #[fixture]
    fn data2() -> ProcessedInput {
        let input = include_str!("test_input2.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data1: ProcessedInput) {
        assert_eq!(part1(&data1), 37_327_623);
    }

    #[rstest]
    fn part2_test(data2: ProcessedInput) {
        assert_eq!(part2(&data2), 23);
    }
}
