use std::collections::{HashMap, HashSet};

use aoc_utils::Solution;

struct Day22;

impl Solution for Day22 {
    type Input<'a> = Vec<Vec<(i64, i64, i64)>>;
    type Output1 = i64;
    type Output2 = i64;

    fn process(input: &str) -> Self::Input<'_> {
        let secrets: Vec<_> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.parse().unwrap())
            .collect();

        secrets.into_iter().map(evolve_secret).collect()
    }

    fn part1(secrets: &Self::Input<'_>) -> Self::Output1 {
        secrets.iter().map(|secret| secret.last().unwrap().0).sum()
    }

    fn part2(secrets: &Self::Input<'_>) -> Self::Output2 {
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
}

aoc_utils::run!(2024, 22, Day22);

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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // Note: The automatic test input fetching retrieves incorrect data.
    aoc_utils::solution_tests!(2024, 22, Day22);

    #[rstest]
    fn part1_test(data1: <Day22 as Solution>::Input<'_>) {
        assert_eq!(37_327_623, Day22::part1(&data1));
    }

    #[rstest]
    fn part2_test(data2: <Day22 as Solution>::Input<'_>) {
        assert_eq!(23, Day22::part2(&data2));
    }
}
