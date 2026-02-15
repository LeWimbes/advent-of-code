use aoc_utils::Solution;
use itertools::Itertools;

struct Day25;

impl Solution for Day25 {
    type Input<'a> = (Vec<Vec<usize>>, Vec<Vec<usize>>);
    type Output1 = usize;
    type Output2 = &'static str;

    fn process(input: &str) -> Self::Input<'_> {
        let schematics: Vec<Vec<Vec<_>>> = input
            .split("\n\n")
            .filter(|s| !s.is_empty())
            .map(|schematic| {
                schematic
                    .lines()
                    .map(|line| line.chars().collect())
                    .collect()
            })
            .collect();

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

    fn part1((locks, keys): &Self::Input<'_>) -> Self::Output1 {
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

    fn part2(_input: &Self::Input<'_>) -> Self::Output2 {
        // Day 25 has no part 2
        "N/A"
    }
}

aoc_utils::run!(2024, 25, Day25);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // Note: The automatic test input fetching retrieves incorrect data.
    aoc_utils::solution_tests!(2024, 25, Day25);

    #[rstest]
    fn part1_test(data1: <Day25 as Solution>::Input<'_>) {
        assert_eq!(3, Day25::part1(&data1));
    }
}
