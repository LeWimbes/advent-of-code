use aoc_utils::Solution;

struct Day13;

impl Solution for Day13 {
    type Input<'a> = Vec<Vec<Vec<bool>>>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        input
            .split("\n\n")
            .map(|pattern| {
                pattern
                    .lines()
                    .map(|line| {
                        line.chars()
                            .map(|tile| match tile {
                                '.' => false,
                                '#' => true,
                                _ => panic!("Unexpected tile!"),
                            })
                            .collect()
                    })
                    .collect()
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
    let (columns, rows) = patterns.iter().fold((0, 0), |(columns, rows), pattern| {
        let rows_above = (1..pattern.len()).find(|rows_above| {
            let rows_above = *rows_above;
            let rows_to_compare = rows_above.min(pattern.len() - rows_above);
            let above_range = (rows_above - rows_to_compare)..rows_above;
            let below_range = rows_above..(rows_above + rows_to_compare);

            above_range
                .rev()
                .zip(below_range)
                .map(|(above, below)| {
                    pattern[above]
                        .iter()
                        .zip(pattern[below].iter())
                        .filter(|(above_tile, below_tile)| above_tile != below_tile)
                        .count()
                })
                .sum::<usize>()
                == smudges
        });

        if let Some(rows_above) = rows_above {
            return (columns, rows + rows_above as u64);
        }

        let cols_left = (1..pattern[0].len()).find(|cols_left| {
            let cols_left = *cols_left;
            let cols_to_compare = cols_left.min(pattern[0].len() - cols_left);
            let left_range = (cols_left - cols_to_compare)..cols_left;
            let right_range = cols_left..(cols_left + cols_to_compare);

            left_range
                .rev()
                .zip(right_range)
                .map(|(left, right)| pattern.iter().filter(|row| row[left] != row[right]).count())
                .sum::<usize>()
                == smudges
        });

        if let Some(cols_left) = cols_left {
            return (columns + cols_left as u64, rows);
        }

        (columns, rows)
    });

    columns + 100 * rows
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 13, Day13);

    #[rstest]
    fn part1_test(data1: <Day13 as Solution>::Input<'_>) {
        assert_eq!(405, Day13::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day13 as Solution>::Input<'_>) {
        assert_eq!(400, Day13::part2(&data1));
    }
}
