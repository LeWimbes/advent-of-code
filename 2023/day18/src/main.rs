use aoc_utils::Solution;
use aoc_utils::grid::Direction;
use regex::Regex;

struct Day18;

impl Solution for Day18 {
    type Input<'a> = (Vec<(i64, i64)>, Vec<(i64, i64)>);
    type Output1 = i64;
    type Output2 = i64;

    fn process(input: &str) -> Self::Input<'_> {
        let re = Regex::new(
        r"(?<direction1>[UDLR]) (?<meters1>\d+) \(#(?<meters2>[0-9a-fA-F]{5})(?<direction2>\d)\)",
    )
    .unwrap();
        let instructions: Vec<Instruction> = re
            .captures_iter(input)
            .map(|caps| {
                let direction1 = parse_direction(&caps["direction1"]);
                let meters1 = caps["meters1"].parse().unwrap();
                let direction2 = parse_direction(&caps["direction2"]);
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
            current1 = dig(current1, instruction.direction1, instruction.meters1);
            coords1.push(current1);
            current2 = dig(current2, instruction.direction2, instruction.meters2);
            coords2.push(current2);
        }

        (coords1, coords2)
    }

    fn part1(coords: &Self::Input<'_>) -> Self::Output1 {
        calculate_area(&coords.0)
    }

    fn part2(coords: &Self::Input<'_>) -> Self::Output2 {
        calculate_area(&coords.1)
    }
}

aoc_utils::run!(2023, 18, Day18);

fn parse_direction(direction: &str) -> Direction {
    match direction {
        "U" | "3" => Direction::North,
        "D" | "1" => Direction::South,
        "L" | "2" => Direction::West,
        "R" | "0" => Direction::East,
        _ => panic!("Unknown direction: {direction}"),
    }
}

/// The position after digging `meters` from `(x, y)` in `direction`.
fn dig((x, y): (i64, i64), direction: Direction, meters: i64) -> (i64, i64) {
    let (dx, dy) = direction.offset();
    (x + dx as i64 * meters, y + dy as i64 * meters)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Instruction {
    direction1: Direction,
    meters1: i64,
    direction2: Direction,
    meters2: i64,
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 18, Day18, examples = [1]);

    #[rstest]
    fn part1_test(data1: <Day18 as Solution>::Input<'_>) {
        assert_eq!(62, Day18::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day18 as Solution>::Input<'_>) {
        assert_eq!(952_408_144_115, Day18::part2(&data1));
    }
}
