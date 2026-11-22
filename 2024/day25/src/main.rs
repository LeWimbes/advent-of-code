use aoc_utils::Solution;
use aoc_utils::grid::Grid;
use itertools::Itertools;

struct Day25;

impl Solution for Day25 {
    type Input<'a> = (Vec<Vec<usize>>, Vec<Vec<usize>>);
    type Output1 = usize;
    type Output2 = &'static str;

    fn process(input: &str) -> Self::Input<'_> {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        for schematic in input.split("\n\n").filter(|s| !s.is_empty()) {
            let schematic = Grid::parse(schematic, |c| c == '#');
            let is_lock = schematic.row(0).iter().all(|&filled| filled);

            // Locks are described by their empty tiles per column, keys by their filled ones
            let counts = schematic
                .columns()
                .map(|column| column.filter(|&&filled| filled != is_lock).count())
                .collect();

            if is_lock {
                locks.push(counts);
            } else {
                keys.push(counts);
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

    aoc_utils::solution_tests!(2024, 25, Day25, examples = [1]);

    #[rstest]
    fn part1_test(data1: <Day25 as Solution>::Input<'_>) {
        assert_eq!(3, Day25::part1(&data1));
    }
}
