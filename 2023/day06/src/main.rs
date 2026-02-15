use aoc_utils::Solution;

struct Day06;

impl Solution for Day06 {
    type Input<'a> = (Vec<u64>, Vec<u64>);
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let lines = input.split_once('\n').unwrap();

        (
            lines
                .0
                .split_whitespace()
                .skip(1)
                .map(|num| num.parse().unwrap())
                .collect(),
            lines
                .1
                .split_whitespace()
                .skip(1)
                .map(|num| num.parse().unwrap())
                .collect(),
        )
    }

    fn part1((time, distance): &Self::Input<'_>) -> Self::Output1 {
        time.iter()
            .enumerate()
            .map(|(i, time)| (*time, distance[i]))
            .map(|(time, distance)| {
                (0..=time)
                    .filter(|charge| (time - charge) * charge > distance)
                    .count()
            })
            .product()
    }

    fn part2((time, distance): &Self::Input<'_>) -> Self::Output2 {
        let time = time.iter().fold(0u64, |acc, part| {
            acc * (10u64.pow(part.checked_ilog10().unwrap_or(0) + 1)) + part
        });
        let distance = distance.iter().fold(0u64, |acc, part| {
            acc * (10u64.pow(part.checked_ilog10().unwrap_or(0) + 1)) + part
        });

        (0..=time)
            .filter(|charge| (time - charge) * charge > distance)
            .count()
    }
}

aoc_utils::run!(2023, 6, Day06);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 6, Day06);

    #[rstest]
    fn part1_test(data1: <Day06 as Solution>::Input<'_>) {
        assert_eq!(288, Day06::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day06 as Solution>::Input<'_>) {
        assert_eq!(71_503, Day06::part2(&data1));
    }
}
