use aoc_utils::Solution;
use itertools::Itertools;

struct Day11;

impl Solution for Day11 {
    type Input<'a> = Vec<(u64, u64)>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        let image: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let empty_rows: Vec<usize> = (0..image.len())
            .rev()
            .filter(|y| image[*y].iter().all(|char| char == &'.'))
            .collect();
        let empty_cols: Vec<usize> = (0..image[0].len())
            .rev()
            .filter(|x| image.iter().map(|row| row[*x]).all(|char| char == '.'))
            .collect();

        let galaxies: Vec<_> =
            image
                .iter()
                .enumerate()
                .flat_map(|(y, row)| {
                    row.iter().enumerate().filter_map(move |(x, char)| {
                        if char == &'#' { Some((x, y)) } else { None }
                    })
                })
                .collect();

        galaxies
            .iter()
            .combinations(2)
            .map(|galaxies| {
                let (x1, y1) = *galaxies[0];
                let (x2, y2) = *galaxies[1];

                let base_distance =
                    (((x2 as i64) - (x1 as i64)).abs() + ((y2 as i64) - (y1 as i64)).abs()) as u64;
                let crossings = (empty_rows
                    .iter()
                    .filter(|row| (y1.min(y2) + 1..y1.max(y2)).contains(row))
                    .count()
                    + empty_cols
                        .iter()
                        .filter(|row| (x1.min(x2) + 1..x1.max(x2)).contains(row))
                        .count()) as u64;

                (base_distance, crossings)
            })
            .collect()
    }

    fn part1(image_galaxies_distances: &Self::Input<'_>) -> Self::Output1 {
        expand(2, image_galaxies_distances).iter().sum()
    }

    fn part2(image_galaxies_distances: &Self::Input<'_>) -> Self::Output2 {
        expand(1_000_000, image_galaxies_distances).iter().sum()
    }
}

aoc_utils::run!(2023, 11, Day11);

fn expand(expansion_factor: u64, distances: &<Day11 as Solution>::Input<'_>) -> Vec<u64> {
    distances
        .iter()
        .map(|(base_distance, crossings)| base_distance + crossings * expansion_factor - crossings)
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // Note: The automatic test input fetching retrieves incorrect data.
    aoc_utils::solution_tests!(2023, 11, Day11);

    #[rstest]
    fn part1_test(data1: <Day11 as Solution>::Input<'_>) {
        assert_eq!(374, Day11::part1(&data1));
    }

    #[rstest]
    fn part2_test1(data1: <Day11 as Solution>::Input<'_>) {
        assert_eq!(1030, expand(10, &data1).iter().sum::<u64>());
    }

    #[rstest]
    fn part2_test2(data1: <Day11 as Solution>::Input<'_>) {
        assert_eq!(8410, expand(100, &data1).iter().sum::<u64>());
    }
}
