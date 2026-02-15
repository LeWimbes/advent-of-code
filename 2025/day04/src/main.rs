use std::iter::once;

use aoc_utils::Solution;

struct Day04;

impl Solution for Day04 {
    type Input<'a> = Vec<Vec<u8>>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let diagram: Vec<Vec<_>> = input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|tile| (tile == '@') as u8)
                    .collect::<Vec<_>>()
            })
            .filter(|row| !row.is_empty())
            .collect();

        // Add a border of '.' tiles around the diagram
        let cols = diagram[0].len();

        once(vec![0; cols + 2])
            .chain(diagram.into_iter().map(|mut row| {
                let mut new_row = Vec::with_capacity(cols + 2);
                new_row.push(0);
                new_row.append(&mut row);
                new_row.push(0);
                new_row
            }))
            .chain(once(vec![0; cols + 2]))
            .collect()
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
            remove_rolls(&mut diagram, &positions);
            removed += positions.len();
        }

        removed
    }
}

aoc_utils::run!(2025, 4, Day04);

fn accessible_rolls(diagram: &<Day04 as Solution>::Input<'_>) -> Vec<(usize, usize)> {
    let rows = diagram.len();
    let cols = diagram[0].len();
    let mut positions = Vec::new();

    for r in 1..rows - 1 {
        for c in 1..cols - 1 {
            if diagram[r][c] == 1 {
                // Count surrounding rolls
                let surrounding = diagram[r - 1][c - 1..=c + 1]
                    .iter()
                    .chain(diagram[r][c - 1..=c + 1].iter())
                    .chain(diagram[r + 1][c - 1..=c + 1].iter())
                    .sum::<u8>();

                // We also count the current roll, so less than 5 instead of 4
                if surrounding < 5 {
                    positions.push((r, c));
                }
            }
        }
    }

    positions
}

fn remove_rolls(diagram: &mut <Day04 as Solution>::Input<'_>, positions: &[(usize, usize)]) {
    for &(r, c) in positions {
        diagram[r][c] = 0;
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2025, 4, Day04);

    #[rstest]
    fn part1_test(data1: <Day04 as Solution>::Input<'_>) {
        assert_eq!(13, Day04::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day04 as Solution>::Input<'_>) {
        assert_eq!(43, Day04::part2(&data1));
    }
}
