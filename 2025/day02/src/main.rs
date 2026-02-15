use aoc_utils::Solution;
use winnow::ascii::{dec_uint, multispace0};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{Parser, Result};

struct Day02;

impl Solution for Day02 {
    type Input<'a> = Vec<(u64, u64)>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
    }

    fn part1(ranges: &Self::Input<'_>) -> Self::Output1 {
        sum_invalid_ids_in_ranges(ranges, has_sequence_twice)
    }

    fn part2(ranges: &Self::Input<'_>) -> Self::Output2 {
        sum_invalid_ids_in_ranges(ranges, has_sequence_multiple_times)
    }
}

aoc_utils::run!(2025, 2, Day02);

fn parse_input<'a>(input: &mut &str) -> Result<<Day02 as Solution>::Input<'a>> {
    terminated(
        separated(
            0..,
            separated_pair(dec_uint, '-', dec_uint),
            (',', multispace0),
        ),
        multispace0,
    )
    .parse_next(input)
}

fn has_sequence_twice(id: u64) -> bool {
    let id_string = id.to_string();
    if !id_string.len().is_multiple_of(2) {
        return false;
    }
    if id_string[..id_string.len() / 2] != id_string[id_string.len() / 2..] {
        return false;
    }
    true
}

fn has_sequence_multiple_times(id: u64) -> bool {
    let id_string = id.to_string();
    for div in 2..=id_string.len() {
        if !id_string.len().is_multiple_of(div) {
            continue;
        }
        let segment_length = id_string.len() / div;
        let segment = &id_string[..segment_length];
        for i in 1..div {
            let start = i * segment_length;
            let end = start + segment_length;
            if &id_string[start..end] != segment {
                break;
            }
            if i == div - 1 {
                return true;
            }
        }
    }
    false
}

fn sum_invalid_ids_in_ranges(
    ranges: &<Day02 as Solution>::Input<'_>,
    is_invalid_id: fn(u64) -> bool,
) -> u64 {
    let mut total_sum: u64 = 0;
    for (start, end) in ranges {
        for id in *start..=*end {
            if is_invalid_id(id) {
                total_sum += id;
            }
        }
    }
    total_sum
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2025, 2, Day02);

    #[rstest]
    fn part1_test(data1: <Day02 as Solution>::Input<'_>) {
        assert_eq!(1_227_775_554, Day02::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day02 as Solution>::Input<'_>) {
        assert_eq!(4_174_379_265, Day02::part2(&data1));
    }
}
