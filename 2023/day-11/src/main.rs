use itertools::Itertools;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 11);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &'static str) -> Vec<(u64, u64)> {
    let image: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let empty_rows: Vec<usize> = (0..image.len())
        .rev()
        .filter(|y| image[*y].iter().all(|char| char == &'.'))
        .collect();
    let empty_cols: Vec<usize> = (0..image[0].len())
        .rev()
        .filter(|x| image.iter().map(|row| row[*x]).all(|char| char == '.'))
        .collect();

    let galaxies: Vec<_> = image
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(
                move |(x, char)| {
                    if char == &'#' {
                        Some((x, y))
                    } else {
                        None
                    }
                },
            )
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

fn expand(expansion_factor: u64, distances: &[(u64, u64)]) -> Vec<u64> {
    distances
        .iter()
        .map(|(base_distance, crossings)| base_distance + crossings * expansion_factor - crossings)
        .collect()
}

fn part1(image_galaxies_distances: &[(u64, u64)]) -> u64 {
    expand(2, image_galaxies_distances).iter().sum()
}

fn part2(image_galaxies_distances: &[(u64, u64)]) -> u64 {
    expand(1_000_000, image_galaxies_distances).iter().sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> Vec<(u64, u64)> {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<(u64, u64)>) {
        assert_eq!(part1(&data), 374);
    }

    #[rstest]
    fn part2_test1(data: Vec<(u64, u64)>) {
        assert_eq!(expand(10, &data).iter().sum::<u64>(), 1030);
    }

    #[rstest]
    fn part2_test2(data: Vec<(u64, u64)>) {
        assert_eq!(expand(100, &data).iter().sum::<u64>(), 8410);
    }
}
