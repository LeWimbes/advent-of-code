use aoc_utils::Solution;
use winnow::ascii::{dec_uint, multispace0, multispace1, space1};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{Parser, Result};

struct Day07;

impl Solution for Day07 {
    type Input<'a> = Vec<(u64, Vec<u64>)>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
    }

    fn part1(equations: &Self::Input<'_>) -> Self::Output1 {
        equations
            .iter()
            .filter(|(result, components)| {
                let mut possible_values = vec![components[0]];

                for component in &components[1..] {
                    let mut next_possible_values = Vec::new();

                    for value in possible_values {
                        next_possible_values.push(value + *component);
                        next_possible_values.push(value * *component);
                    }

                    possible_values = next_possible_values;
                }

                possible_values.iter().any(|value| value == result)
            })
            .map(|equation| equation.0)
            .sum()
    }

    fn part2(equations: &Self::Input<'_>) -> Self::Output2 {
        equations
            .iter()
            .filter(|(result, components)| {
                let mut possible_values = vec![components[0]];

                for component in &components[1..] {
                    let mut next_possible_values = Vec::new();

                    for value in possible_values {
                        next_possible_values.push(value + *component);
                        next_possible_values.push(value * *component);
                        next_possible_values.push(concat(value, *component));
                    }

                    possible_values = next_possible_values;
                }

                possible_values.iter().any(|value| value == result)
            })
            .map(|equation| equation.0)
            .sum()
    }
}

aoc_utils::run!(2024, 7, Day07);

fn parse_input<'a>(input: &mut &str) -> Result<<Day07 as Solution>::Input<'a>> {
    terminated(
        separated(
            0..,
            separated_pair(
                dec_uint,
                (':', space1),
                separated(1.., dec_uint::<_, u64, _>, space1),
            ),
            multispace1,
        ),
        multispace0,
    )
    .parse_next(input)
}

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 7, Day07);

    #[rstest]
    fn part1_test(data1: <Day07 as Solution>::Input<'_>) {
        assert_eq!(3749, Day07::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day07 as Solution>::Input<'_>) {
        assert_eq!(11387, Day07::part2(&data1));
    }
}
