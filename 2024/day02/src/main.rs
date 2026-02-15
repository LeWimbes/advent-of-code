use aoc_utils::Solution;
use itertools::Itertools;
use winnow::ascii::{dec_uint, multispace0, multispace1, space1};
use winnow::combinator::{separated, terminated};
use winnow::{Parser, Result};

struct Day02;

impl Solution for Day02 {
    type Input<'a> = Vec<Vec<u32>>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
    }

    fn part1(reports: &Self::Input<'_>) -> Self::Output1 {
        reports
            .iter()
            .filter(|&report| is_valid_report(report))
            .count()
    }

    fn part2(reports: &Self::Input<'_>) -> Self::Output2 {
        reports
            .iter()
            .filter(|&report| {
                (0..report.len()).any(|skip| {
                    is_valid_report(
                        &report
                            .iter()
                            .enumerate()
                            .filter(|(i, _)| *i != skip)
                            .map(|(_, &v)| v)
                            .collect::<Vec<_>>(),
                    )
                })
            })
            .count()
    }
}

aoc_utils::run!(2024, 2, Day02);

fn parse_input<'a>(input: &mut &str) -> Result<<Day02 as Solution>::Input<'a>> {
    terminated(
        separated::<_, Vec<u32>, _, _, _, _, _>(
            0..,
            separated(1.., dec_uint::<_, u32, _>, space1),
            multispace1,
        ),
        multispace0,
    )
    .parse_next(input)
}

fn is_valid_report(report: &[u32]) -> bool {
    report.iter().tuple_windows().all(|(a, &b)| {
        (1..=3).contains(&a.abs_diff(b))
            && (report.iter().is_sorted() || report.iter().rev().is_sorted())
    })
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 2, Day02);

    #[rstest]
    fn part1_test(data1: <Day02 as Solution>::Input<'_>) {
        assert_eq!(2, Day02::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day02 as Solution>::Input<'_>) {
        assert_eq!(4, Day02::part2(&data1));
    }
}
