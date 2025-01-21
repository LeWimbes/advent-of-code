const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 13);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &'static str) -> Vec<Vec<Vec<bool>>> {
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

fn find_reflections(patterns: &[Vec<Vec<bool>>], smudges: usize) -> u64 {
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

fn part1(patterns: &[Vec<Vec<bool>>]) -> u64 {
    find_reflections(patterns, 0)
}

fn part2(patterns: &[Vec<Vec<bool>>]) -> u64 {
    find_reflections(patterns, 1)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> Vec<Vec<Vec<bool>>> {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<Vec<Vec<bool>>>) {
        assert_eq!(part1(&data), 405);
    }

    #[rstest]
    fn part2_test(data: Vec<Vec<Vec<bool>>>) {
        assert_eq!(part2(&data), 400);
    }
}
