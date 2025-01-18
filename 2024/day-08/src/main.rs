use itertools::Itertools;
use num::Integer;
use std::collections::HashMap;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 8);

type ParsedInput = Vec<Vec<char>>;
type ProcessedInput = ((i32, i32), Vec<Vec<(i32, i32)>>);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn process_input(input: &'static str) -> ProcessedInput {
    let map = parse_input(input);

    let dimensions = (map[0].len() as i32, map.len() as i32);

    let mut antennas_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] != '.' {
                antennas_map
                    .entry(map[y][x])
                    .or_default()
                    .push((x as i32, y as i32));
            }
        }
    }

    let antennas = antennas_map.into_values().collect();

    (dimensions, antennas)
}

fn part1((dimensions, antennas): &ProcessedInput) -> usize {
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

fn part2((dimensions, antennas): &ProcessedInput) -> usize {
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

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> ProcessedInput {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: ProcessedInput) {
        assert_eq!(part1(&data), 14);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2(&data), 34);
    }
}
