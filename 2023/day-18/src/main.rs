use regex::Regex;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 18);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North = 3,
    West = 2,
    South = 1,
    East = 0,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'U' | '3' => Ok(Direction::North),
            'D' | '1' => Ok(Direction::South),
            'L' | '2' => Ok(Direction::West),
            'R' | '0' => Ok(Direction::East),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Instruction {
    direction1: Direction,
    meters1: i64,
    direction2: Direction,
    meters2: i64,
}

fn process_input(input: &'static str) -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
    let re = Regex::new(
        r"(?<direction1>[UDLR]) (?<meters1>\d+) \(#(?<meters2>[0-9a-fA-F]{5})(?<direction2>\d)\)",
    )
    .unwrap();
    let instructions: Vec<Instruction> = re
        .captures_iter(input)
        .map(|caps| {
            let direction1 = caps["direction1"]
                .chars()
                .next()
                .unwrap()
                .try_into()
                .unwrap();
            let meters1 = caps["meters1"].parse().unwrap();
            let direction2 = caps["direction2"]
                .chars()
                .next()
                .unwrap()
                .try_into()
                .unwrap();
            let meters2 = i64::from_str_radix(&caps["meters2"], 16).unwrap();
            Instruction {
                direction1,
                meters1,
                direction2,
                meters2,
            }
        })
        .collect();

    let mut coords1: Vec<(i64, i64)> = Vec::with_capacity(instructions.len());
    let mut coords2: Vec<(i64, i64)> = Vec::with_capacity(instructions.len());
    let mut current1 = (0, 0);
    let mut current2 = (0, 0);
    coords1.push(current1);
    coords2.push(current2);

    for instruction in instructions.iter().take(instructions.len() - 1) {
        let meters = instruction.meters1;
        match instruction.direction1 {
            Direction::North => current1.1 -= meters,
            Direction::West => current1.0 -= meters,
            Direction::South => current1.1 += meters,
            Direction::East => current1.0 += meters,
        };
        coords1.push(current1);
        let meters = instruction.meters2;
        match instruction.direction2 {
            Direction::North => current2.1 -= meters,
            Direction::West => current2.0 -= meters,
            Direction::South => current2.1 += meters,
            Direction::East => current2.0 += meters,
        };
        coords2.push(current2);
    }

    (coords1, coords2)
}

/// Looking for i (inner area) + b (border)
/// Using:
/// [Pick's theorem](https://en.wikipedia.org/wiki/Pick%27s_theorem)
/// [Shoelace formula](https://en.wikipedia.org/wiki/Shoelace_formula)
///
///     A = i + b/2 - 1
/// <=> i = A - b/2 + 1
/// Adding b:
///  => i + b = A + b/2 + 1
fn calculate_area(coords: &[(i64, i64)]) -> i64 {
    let coord_pairs: Vec<_> = coords.iter().zip(coords.iter().cycle().skip(1)).collect();

    let b = coord_pairs
        .iter()
        .map(|((x1, y1), (x2, y2))| (x1 - x2).abs() + (y1 - y2).abs())
        .sum::<i64>();

    let i = coord_pairs
        .iter()
        .map(|((x1, y1), (x2, y2))| x1 * y2 - x2 * y1)
        .sum::<i64>()
        .abs()
        / 2;

    i + b / 2 + 1
}

fn part1(coords: &(Vec<(i64, i64)>, Vec<(i64, i64)>)) -> i64 {
    calculate_area(&coords.0)
}

fn part2(coords: &(Vec<(i64, i64)>, Vec<(i64, i64)>)) -> i64 {
    calculate_area(&coords.1)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> (Vec<(i64, i64)>, Vec<(i64, i64)>) {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: (Vec<(i64, i64)>, Vec<(i64, i64)>)) {
        assert_eq!(part1(&data), 62);
    }

    #[rstest]
    fn part2_test(data: (Vec<(i64, i64)>, Vec<(i64, i64)>)) {
        assert_eq!(part2(&data), 952_408_144_115);
    }
}
