use aoc_utils::Solution;
use aoc_utils::grid::{Grid, Pos};
use itertools::Itertools;

struct Day11;

impl Solution for Day11 {
    type Input<'a> = Vec<(u64, u64)>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        let image = Grid::parse(input, |c| c);

        let empty_rows: Vec<usize> = (0..image.height())
            .filter(|&y| image.row(y).iter().all(|&c| c == '.'))
            .collect();
        let empty_cols: Vec<usize> = (0..image.width())
            .filter(|&x| image.column(x).all(|&c| c == '.'))
            .collect();

        let galaxies: Vec<Pos> = image
            .iter()
            .filter(|&(_pos, &c)| c == '#')
            .map(|(pos, _c)| pos)
            .collect();

        galaxies
            .iter()
            .tuple_combinations()
            .map(|(&a, &b)| {
                let crossings = empty_rows
                    .iter()
                    .filter(|&&y| a.y.min(b.y) < y && y < a.y.max(b.y))
                    .count()
                    + empty_cols
                        .iter()
                        .filter(|&&x| a.x.min(b.x) < x && x < a.x.max(b.x))
                        .count();

                (a.manhattan(b) as u64, crossings as u64)
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

    aoc_utils::solution_tests!(2023, 11, Day11, examples = [1]);

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
