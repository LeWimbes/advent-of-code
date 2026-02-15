use aoc_utils::Solution;
use regex::Regex;

struct Day02;

impl Solution for Day02 {
    type Input<'a> = Vec<Vec<(u32, u32, u32)>>;
    type Output1 = u32;
    type Output2 = u32;

    fn process(input: &str) -> Self::Input<'_> {
        input
            .lines()
            .map(|line| {
                line.split(';')
                    .map(|set| {
                        let re = Regex::new(r"(?<count>\d+) (?<color>(red|green|blue))").unwrap();

                        let mut red: u32 = 0;
                        let mut green: u32 = 0;
                        let mut blue: u32 = 0;

                        for caps in re.captures_iter(set) {
                            match &caps["color"] {
                                "red" => red = caps["count"].parse().unwrap(),
                                "green" => green = caps["count"].parse().unwrap(),
                                "blue" => blue = caps["count"].parse().unwrap(),
                                _ => {}
                            }
                        }

                        (red, green, blue)
                    })
                    .collect()
            })
            .collect()
    }

    fn part1(games: &Self::Input<'_>) -> Self::Output1 {
        let max: (u32, u32, u32) = (12, 13, 14);
        games
            .iter()
            .enumerate()
            .filter_map(|(id, game)| {
                if game
                    .iter()
                    .all(|set| set.0 <= max.0 && set.1 <= max.1 && set.2 <= max.2)
                {
                    Some(id as u32 + 1)
                } else {
                    None
                }
            })
            .sum()
    }

    fn part2(games: &Self::Input<'_>) -> Self::Output2 {
        games
            .iter()
            .map(|game| {
                let minimums = game.iter().fold((0, 0, 0), |acc, set| {
                    (acc.0.max(set.0), acc.1.max(set.1), acc.2.max(set.2))
                });
                minimums.0 * minimums.1 * minimums.2
            })
            .sum()
    }
}

aoc_utils::run!(2023, 2, Day02);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 2, Day02);

    #[rstest]
    fn part1_test(data1: <Day02 as Solution>::Input<'_>) {
        assert_eq!(8, Day02::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day02 as Solution>::Input<'_>) {
        assert_eq!(2286, Day02::part2(&data1));
    }
}
