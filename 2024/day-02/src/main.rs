use itertools::Itertools;
use winnow::ascii::{dec_uint, multispace0, multispace1, space1};
use winnow::combinator::{separated, terminated};
use winnow::{PResult, Parser};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 2);

type ParsedInput = Vec<Vec<u32>>;
type ProcessedInput = Vec<Vec<u32>>;

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &mut &str) -> PResult<ParsedInput> {
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

fn process_input(input: &'static str) -> ProcessedInput {
    parse_input
        .parse(input)
        .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
}

fn is_valid_report(report: &[u32]) -> bool {
    report.iter().tuple_windows().all(|(a, &b)| {
        (1..=3).contains(&a.abs_diff(b))
            && (report.iter().is_sorted() || report.iter().rev().is_sorted())
    })
}

fn part1(reports: &ProcessedInput) -> usize {
    reports
        .iter()
        .filter(|&report| is_valid_report(report))
        .count()
}

fn part2(reports: &ProcessedInput) -> usize {
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
        assert_eq!(part1(&data), 2);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2(&data), 4);
    }
}
