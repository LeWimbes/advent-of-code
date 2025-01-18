use winnow::ascii::{dec_int, multispace0, multispace1};
use winnow::combinator::{delimited, repeat, separated_pair, terminated};
use winnow::{PResult, Parser};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 13);

type ParsedInput = Vec<((i64, i64), (i64, i64), (i64, i64))>;
type ProcessedInput = Vec<((i64, i64), (i64, i64), (i64, i64))>;

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &mut &str) -> PResult<ParsedInput> {
    terminated(
        repeat(
            0..,
            (
                delimited(
                    "Button A: X+",
                    separated_pair(dec_int, ", Y+", dec_int),
                    multispace1,
                ),
                delimited(
                    "Button B: X+",
                    separated_pair(dec_int, ", Y+", dec_int),
                    multispace1,
                ),
                delimited(
                    "Prize: X=",
                    separated_pair(dec_int, ", Y=", dec_int),
                    multispace1,
                ),
            ),
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

fn part1(machines: &ProcessedInput) -> i64 {
    machines
        .iter()
        .map(|(a, b, prize)| {
            let mut cheapest = i64::MAX;

            for a_presses in 0..=100 {
                for b_presses in 0..=100 {
                    if a_presses * a.0 + b_presses * b.0 == prize.0
                        && a_presses * a.1 + b_presses * b.1 == prize.1
                    {
                        let cost = a_presses * 3 + b_presses;
                        if cost < cheapest {
                            cheapest = cost;
                        }
                    }
                }
            }

            if cheapest == i64::MAX {
                0
            } else {
                cheapest
            }
        })
        .sum()
}

fn is_int_div(numerator: i64, denominator: i64) -> bool {
    denominator != 0 && numerator % denominator == 0
}

fn part2(machines: &ProcessedInput) -> i64 {
    let machines: ProcessedInput = machines
        .iter()
        .map(|(a, b, prize)| {
            (
                *a,
                *b,
                (10_000_000_000_000 + prize.0, 10_000_000_000_000 + prize.1),
            )
        })
        .collect();

    machines
        .iter()
        .map(|(a, b, prize)| {
            let determinant = a.0 * b.1 - b.0 * a.1;

            if determinant == 0 {
                // apparently the input was constructed so that if there is a solution, it's unique
                0
            } else {
                let a_dividend = prize.0 * b.1 - prize.1 * b.0;
                let b_dividend = a.0 * prize.1 - a.1 * prize.0;

                if is_int_div(a_dividend, determinant) && is_int_div(b_dividend, determinant) {
                    let a = a_dividend / determinant;
                    let b = b_dividend / determinant;
                    if a > 0 && b > 0 {
                        a * 3 + b
                    } else {
                        0
                    }
                } else {
                    0
                }
            }
        })
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
        assert_eq!(part1(&data), 480);
    }
}
