use aoc_utils::Solution;
use itertools::Itertools;
use num::Integer;
use std::collections::HashMap;

struct Day08;

impl Solution for Day08 {
    type Input<'a> = ((i32, i32), Vec<Vec<(i32, i32)>>);
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let map: Vec<Vec<_>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();

        let dimensions = (map[0].len() as i32, map.len() as i32);

        let mut antennas_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

        for (y, row) in map.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell != '.' {
                    antennas_map
                        .entry(cell)
                        .or_default()
                        .push((x as i32, y as i32));
                }
            }
        }

        let antennas = antennas_map.into_values().collect();

        (dimensions, antennas)
    }

    fn part1((dimensions, antennas): &Self::Input<'_>) -> Self::Output1 {
        let (len_x, len_y) = dimensions;

        antennas
            .iter()
            .flat_map(|antenna_type| {
                antenna_type.iter().tuple_combinations().flat_map(|(a, b)| {
                    let (x1, y1) = a;
                    let (x2, y2) = b;

                    let xd = x2 - x1;
                    let yd = y2 - y1;

                    vec![(x1 - xd, y1 - yd), (x2 + xd, y2 + yd)]
                })
            })
            .unique()
            .filter(|(x, y)| x >= &0 && y >= &0 && x < len_x && y < len_y)
            .count()
    }

    fn part2((dimensions, antennas): &Self::Input<'_>) -> Self::Output2 {
        let (len_x, len_y) = dimensions;

        antennas
            .iter()
            .flat_map(|antenna_type| {
                antenna_type.iter().tuple_combinations().flat_map(|(a, b)| {
                    let (x1, y1) = a;
                    let (x2, y2) = b;

                    let xd = x2 - x1;
                    let yd = y2 - y1;

                    let gcd = xd.gcd(&yd);
                    let xd = xd / gcd;
                    let yd = yd / gcd;

                    let mut antinodes = Vec::new();

                    let mut x = *x1;
                    let mut y = *y1;
                    while x >= 0 && y >= 0 && x < *len_x && y < *len_y {
                        antinodes.push((x, y));
                        x -= xd;
                        y -= yd;
                    }

                    let mut x = *x1;
                    let mut y = *y1;
                    while x >= 0 && y >= 0 && x < *len_x && y < *len_y {
                        antinodes.push((x, y));
                        x += xd;
                        y += yd;
                    }

                    antinodes
                })
            })
            .unique()
            .count()
    }
}

aoc_utils::run!(2024, 8, Day08);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 8, Day08);

    #[rstest]
    fn part1_test(data1: <Day08 as Solution>::Input<'_>) {
        assert_eq!(14, Day08::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day08 as Solution>::Input<'_>) {
        assert_eq!(34, Day08::part2(&data1));
    }
}
