use aoc_utils::Solution;
use winnow::ascii::{dec_int, multispace0, multispace1};
use winnow::combinator::{delimited, repeat, separated_pair, terminated};
use winnow::{Parser, Result};

struct Day13;

impl Solution for Day13 {
    type Input<'a> = Vec<((i64, i64), (i64, i64), (i64, i64))>;
    type Output1 = i64;
    type Output2 = i64;

    fn process(input: &str) -> Self::Input<'_> {
        parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
    }

    fn part1(machines: &Self::Input<'_>) -> Self::Output1 {
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

                if cheapest == i64::MAX { 0 } else { cheapest }
            })
            .sum()
    }

    fn part2(machines: &Self::Input<'_>) -> Self::Output2 {
        let machines: Self::Input<'_> = machines
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
                        if a > 0 && b > 0 { a * 3 + b } else { 0 }
                    } else {
                        0
                    }
                }
            })
            .sum()
    }
}

aoc_utils::run!(2024, 13, Day13);

fn parse_input<'a>(input: &mut &str) -> Result<<Day13 as Solution>::Input<'a>> {
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

fn is_int_div(numerator: i64, denominator: i64) -> bool {
    denominator != 0 && numerator % denominator == 0
}
#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 13, Day13);

    #[rstest]
    fn part1_test(data1: <Day13 as Solution>::Input<'_>) {
        assert_eq!(480, Day13::part1(&data1));
    }
}
