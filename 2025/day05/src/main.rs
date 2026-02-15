use aoc_utils::Solution;
use winnow::ascii::{dec_uint, multispace0, multispace1};
use winnow::combinator::{separated, separated_pair, terminated};
use winnow::{Parser, Result};

struct Day05;

impl Solution for Day05 {
    type Input<'a> = (Vec<FreshRange>, Vec<usize>);
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let (ranges, mut ids) = parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"));

        let mut fresh_ranges: Vec<FreshRange> = ranges
            .into_iter()
            .map(|(start, end)| FreshRange { start, end })
            .collect();

        fresh_ranges.sort();
        fresh_ranges = compile_fresh_ranges(fresh_ranges);
        ids.sort();

        (fresh_ranges, ids)
    }

    fn part1(database: &Self::Input<'_>) -> Self::Output1 {
        count_fresh(database)
    }

    fn part2((fresh_ranges, _ids): &Self::Input<'_>) -> Self::Output2 {
        fresh_ranges
            .iter()
            .map(|range| range.end - range.start + 1)
            .sum()
    }
}

aoc_utils::run!(2025, 5, Day05);

/// Represents a range of fresh record IDs.
///
/// `start` and `end` are inclusive.
///
/// Can be sorted by start value (small to big), then by end value (big to small).
struct FreshRange {
    start: usize,
    end: usize,
}

impl PartialEq for FreshRange {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl Eq for FreshRange {}

impl Ord for FreshRange {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start
            .cmp(&other.start)
            .then_with(|| other.end.cmp(&self.end))
    }
}

impl PartialOrd for FreshRange {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FreshRange {
    fn contains(&self, id: usize) -> bool {
        self.start <= id && id <= self.end
    }

    fn contains_range(&self, other: &FreshRange) -> bool {
        self.start <= other.start && other.end <= self.end
    }

    fn overlaps(&self, other: &FreshRange) -> bool {
        self.start <= other.end && other.start <= self.end
    }
}

type ParsedInput = (Vec<(usize, usize)>, Vec<usize>);
fn parse_input(input: &mut &str) -> Result<ParsedInput> {
    terminated(
        separated_pair(
            separated(0.., separated_pair(dec_uint, '-', dec_uint), multispace1),
            multispace1,
            separated(0.., dec_uint::<_, usize, _>, multispace1),
        ),
        multispace0,
    )
    .parse_next(input)
}

/// Compile overlapping or contained fresh ranges into minimal set of ranges.
///
/// Assumes input ranges are sorted.
fn compile_fresh_ranges(ranges: Vec<FreshRange>) -> Vec<FreshRange> {
    let mut iter = ranges.into_iter();
    let mut compiled: Vec<FreshRange> = vec![iter.next().unwrap()];

    for range in iter {
        let last = compiled.last_mut().unwrap();
        if last.contains_range(&range) {
            // Current range is fully contained in last range, skip it
            continue;
        } else if last.overlaps(&range) {
            // Ranges overlap, extend last range
            last.end = range.end;
        } else {
            // No overlap, add new range
            compiled.push(range);
        }
    }

    compiled
}

/// Count how many IDs are in fresh ranges.
///
/// Assumes both lists are sorted.
fn count_fresh<'a>(database: &<Day05 as Solution>::Input<'a>) -> usize {
    let (fresh_ranges, ids) = database;

    let mut fresh_range_index = 0;
    ids.iter()
        .filter(|id| {
            while fresh_range_index < fresh_ranges.len()
                && fresh_ranges[fresh_range_index].end < **id
            {
                fresh_range_index += 1;
            }

            if fresh_range_index == fresh_ranges.len() {
                return false;
            }

            fresh_ranges[fresh_range_index].contains(**id)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2025, 5, Day05);

    #[rstest]
    fn part1_test(data1: <Day05 as Solution>::Input<'_>) {
        assert_eq!(3, Day05::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day05 as Solution>::Input<'_>) {
        assert_eq!(14, Day05::part2(&data1));
    }
}
