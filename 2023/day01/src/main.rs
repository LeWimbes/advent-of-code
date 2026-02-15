use aoc_utils::Solution;

struct Day01;

impl Solution for Day01 {
    type Input<'a> = Vec<&'a str>;
    type Output1 = u32;
    type Output2 = u32;

    fn process(input: &str) -> Self::Input<'_> {
        input.lines().collect()
    }

    fn part1(lines: &Self::Input<'_>) -> Self::Output1 {
        lines
            .iter()
            .map(|line| {
                let mut it = line.chars().filter_map(|char| char.to_digit(10));

                let first = it.next().expect("First digit expected!");
                let last = it.next_back().unwrap_or(first);

                first * 10 + last
            })
            .sum()
    }

    fn part2(lines: &Self::Input<'_>) -> Self::Output2 {
        let mapping = [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ];

        lines
            .iter()
            .map(|line| {
                let mut it = (0..line.len()).filter_map(|start| {
                    let sub_line = &line[start..];
                    for (value_str, value) in mapping {
                        if sub_line.starts_with(value_str) {
                            return Some(value);
                        }
                    }
                    sub_line.chars().next().unwrap().to_digit(10)
                });

                let first = it.next().expect("First digit expected!");
                let last = it.next_back().unwrap_or(first);

                first * 10 + last
            })
            .sum()
    }
}

aoc_utils::run!(2023, 1, Day01);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 1, Day01);

    #[rstest]
    fn part1_test(data1: <Day01 as Solution>::Input<'_>) {
        assert_eq!(142, Day01::part1(&data1));
    }

    #[rstest]
    fn part2_test(data2: <Day01 as Solution>::Input<'_>) {
        assert_eq!(281, Day01::part2(&data2));
    }
}
