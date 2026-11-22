use aoc_utils::Solution;
use aoc_utils::grid::{Grid, Pos};

struct Day04;

impl Solution for Day04 {
    type Input<'a> = Grid<bool>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        Grid::parse(input, |tile| tile == '@')
    }

    fn part1(diagram: &Self::Input<'_>) -> Self::Output1 {
        accessible_rolls(diagram).len()
    }

    fn part2(diagram: &Self::Input<'_>) -> Self::Output2 {
        let mut diagram = diagram.clone();
        let mut removed = 0;

        loop {
            let positions = accessible_rolls(&diagram);
            if positions.is_empty() {
                break;
            }
            for &pos in &positions {
                diagram[pos] = false;
            }
            removed += positions.len();
        }

        removed
    }
}

aoc_utils::run!(2025, 4, Day04);

fn accessible_rolls(diagram: &<Day04 as Solution>::Input<'_>) -> Vec<Pos> {
    diagram
        .iter()
        .filter(|&(pos, &roll)| {
            roll && diagram
                .neighbors8(pos)
                .filter(|&next| diagram[next])
                .count()
                < 4
        })
        .map(|(pos, _)| pos)
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2025, 4, Day04, examples = [1]);

    #[rstest]
    fn part1_test(data1: <Day04 as Solution>::Input<'_>) {
        assert_eq!(13, Day04::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day04 as Solution>::Input<'_>) {
        assert_eq!(43, Day04::part2(&data1));
    }
}
