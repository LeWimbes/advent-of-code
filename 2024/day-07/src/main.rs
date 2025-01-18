use winnow::ascii::{dec_uint, multispace0, multispace1, space1};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{PResult, Parser};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 7);

type ParsedInput = Vec<(u64, Vec<u64>)>;
type ProcessedInput = Vec<(u64, Vec<u64>)>;

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &mut &str) -> PResult<ParsedInput> {
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

fn process_input(input: &'static str) -> ProcessedInput {
    parse_input
        .parse(input)
        .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
}

fn part1(equations: &ProcessedInput) -> u64 {
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

fn concat(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

fn part2(equations: &ProcessedInput) -> u64 {
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

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> ProcessedInput {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: ProcessedInput) {
        assert_eq!(part1(&data), 3749);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2(&data), 11387);
    }
}
