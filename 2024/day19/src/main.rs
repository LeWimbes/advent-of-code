use aoc_utils::Solution;

struct Day19;

impl Solution for Day19 {
    type Input<'a> = Vec<usize>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let parts = input.split_once("\n\n").unwrap();

        let patterns = parts.0.split(", ").collect::<Vec<_>>();
        let designs = parts
            .1
            .lines()
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();

        designs
            .iter()
            .map(|design| possible_arrangements(design, &patterns))
            .collect()
    }

    fn part1(arrangements: &Self::Input<'_>) -> Self::Output1 {
        arrangements.iter().filter(|&&a| a > 0).count()
    }

    fn part2(arrangements: &Self::Input<'_>) -> Self::Output2 {
        arrangements.iter().sum()
    }
}

aoc_utils::run!(2024, 19, Day19);

fn possible_arrangements(design: &str, patterns: &[&str]) -> usize {
    let mut arrangements = vec![0; design.len()];

    for index in (0..design.len()).rev() {
        let sub_design = &design[index..];

        for &pattern in patterns {
            if sub_design == pattern {
                arrangements[index] += 1;
            } else if sub_design.starts_with(pattern) {
                arrangements[index] += arrangements[index + pattern.len()];
            }
        }
    }

    arrangements[0]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 19, Day19);

    #[rstest]
    fn part1_test(data1: <Day19 as Solution>::Input<'_>) {
        assert_eq!(6, Day19::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day19 as Solution>::Input<'_>) {
        assert_eq!(16, Day19::part2(&data1));
    }
}
