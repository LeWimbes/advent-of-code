use itertools::Itertools;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 25);

type ParsedInput = Vec<Vec<Vec<char>>>;
type ProcessedInput = (Vec<Vec<usize>>, Vec<Vec<usize>>);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    input
        .split("\n\n")
        .filter(|s| !s.is_empty())
        .map(|schematic| {
            schematic
                .lines()
                .map(|line| line.chars().collect())
                .collect()
        })
        .collect()
}

fn process_input(input: &'static str) -> ProcessedInput {
    let schematics = parse_input(input);

    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematic in schematics {
        if schematic[0].iter().all(|&c| c == '#') {
            let empty: Vec<usize> = (0..schematic[0].len())
                .map(|x| {
                    (0..schematic.len())
                        .filter(|&y| schematic[y][x] == '.')
                        .count()
                })
                .collect();
            locks.push(empty);
        } else {
            let full: Vec<usize> = (0..schematic[0].len())
                .map(|x| {
                    (0..schematic.len())
                        .filter(|&y| schematic[y][x] == '#')
                        .count()
                })
                .collect();
            keys.push(full);
        }
    }

    (locks, keys)
}

fn part1((locks, keys): &ProcessedInput) -> usize {
    locks
        .iter()
        .cartesian_product(keys)
        .filter(|(lock, key)| {
            lock.iter()
                .zip(key.iter())
                .all(|(lock_empty, key_full)| lock_empty >= key_full)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(false);

    #[fixture]
    fn data() -> ProcessedInput {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: ProcessedInput) {
        assert_eq!(part1(&data), 3);
    }
}
