use aoc_utils::Solution;
use aoc_utils::grid::Grid;

struct Day13;

impl Solution for Day13 {
    type Input<'a> = Vec<Grid<bool>>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        input
            .split("\n\n")
            .map(|pattern| {
                Grid::parse(pattern, |tile| match tile {
                    '.' => false,
                    '#' => true,
                    _ => panic!("Unexpected tile!"),
                })
            })
            .collect()
    }

    fn part1(patterns: &Self::Input<'_>) -> Self::Output1 {
        find_reflections(patterns, 0)
    }

    fn part2(patterns: &Self::Input<'_>) -> Self::Output2 {
        find_reflections(patterns, 1)
    }
}

aoc_utils::run!(2023, 13, Day13);

fn find_reflections(patterns: &<Day13 as Solution>::Input<'_>, smudges: usize) -> u64 {
    patterns
        .iter()
        .map(|pattern| {
            let row_differences = |above: usize, below: usize| {
                let (above, below) = (pattern.row(above), pattern.row(below));
                above.iter().zip(below).filter(|(a, b)| a != b).count()
            };
            let column_differences = |left: usize, right: usize| {
                let (left, right) = (pattern.column(left), pattern.column(right));
                left.zip(right).filter(|(l, r)| l != r).count()
            };

            find_reflection(pattern.height(), smudges, row_differences)
                .map(|rows_above| 100 * rows_above)
                .or_else(|| find_reflection(pattern.width(), smudges, column_differences))
                .unwrap_or_default()
        })
        .sum()
}

/// The number of lines before the reflection line,
/// if there is one with exactly `smudges` differing tiles.
/// `differences(a, b)` counts the differing tiles of lines `a` and `b`.
fn find_reflection(
    lines: usize,
    smudges: usize,
    differences: impl Fn(usize, usize) -> usize,
) -> Option<u64> {
    (1..lines)
        .find(|&before| {
            let to_compare = before.min(lines - before);
            let before_range = (before - to_compare)..before;
            let after_range = before..(before + to_compare);

            before_range
                .rev()
                .zip(after_range)
                .map(|(a, b)| differences(a, b))
                .sum::<usize>()
                == smudges
        })
        .map(|before| before as u64)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 13, Day13, examples = [1]);

    #[rstest]
    fn part1_test(data1: <Day13 as Solution>::Input<'_>) {
        assert_eq!(405, Day13::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day13 as Solution>::Input<'_>) {
        assert_eq!(400, Day13::part2(&data1));
    }
}
