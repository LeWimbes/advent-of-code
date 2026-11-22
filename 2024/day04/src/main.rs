use aoc_utils::Solution;
use aoc_utils::grid::{Grid, OFFSETS8, Pos};

struct Day04;

impl Solution for Day04 {
    type Input<'a> = Grid<char>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        Grid::parse(input, |c| c)
    }

    fn part1(matrix: &Self::Input<'_>) -> Self::Output1 {
        matrix.positions().map(|pos| xmas_starts(matrix, pos)).sum()
    }

    fn part2(matrix: &Self::Input<'_>) -> Self::Output2 {
        matrix
            .positions()
            .filter(|&pos| is_x_mas_middle(matrix, pos))
            .count()
    }
}

aoc_utils::run!(2024, 4, Day04);

/// The number of directions in which XMAS starts at `pos`.
fn xmas_starts(matrix: &<Day04 as Solution>::Input<'_>, pos: Pos) -> usize {
    OFFSETS8
        .into_iter()
        .filter(|&(dx, dy)| {
            "XMAS".chars().zip(0..).all(|(c, k)| {
                pos.offset(dx * k, dy * k).and_then(|pos| matrix.get(pos)) == Some(&c)
            })
        })
        .count()
}

fn is_x_mas_middle(matrix: &<Day04 as Solution>::Input<'_>, pos: Pos) -> bool {
    let at = |dx: isize, dy: isize| pos.offset(dx, dy).and_then(|pos| matrix.get(pos));
    let is_mas = |a: Option<&char>, b: Option<&char>| {
        matches!((a, b), (Some('M'), Some('S')) | (Some('S'), Some('M')))
    };

    matrix[pos] == 'A' && is_mas(at(-1, -1), at(1, 1)) && is_mas(at(-1, 1), at(1, -1))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 4, Day04, examples = [2]);

    #[rstest]
    fn part1_test(data1: <Day04 as Solution>::Input<'_>) {
        assert_eq!(18, Day04::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day04 as Solution>::Input<'_>) {
        assert_eq!(9, Day04::part2(&data1));
    }
}
