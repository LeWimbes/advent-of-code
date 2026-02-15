use aoc_utils::Solution;

struct Day07;

impl Solution for Day07 {
    type Input<'a> = Vec<Vec<char>>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>()
    }

    fn part1(map: &Self::Input<'_>) -> Self::Output1 {
        let width = map[0].len();
        let mut beams = vec![false; width];
        beams[width / 2] = true;
        let mut splits = 0;

        for row in map.iter().skip(1) {
            let mut new_beams = vec![false; width];
            for (i, is_beam) in beams.iter().enumerate() {
                if !is_beam {
                    continue;
                }
                if row[i] == '^' {
                    splits += 1;
                    new_beams[i - 1] = true;
                    new_beams[i + 1] = true;
                } else {
                    new_beams[i] = true;
                }
            }
            beams = new_beams;
        }

        splits
    }

    fn part2(map: &Self::Input<'_>) -> Self::Output2 {
        let width = map[0].len();
        let mut beams = vec![0u64; width];
        beams[width / 2] = 1;

        for row in map.iter().skip(1) {
            let mut new_beams = vec![0; width];
            for (i, &worlds) in beams.iter().enumerate() {
                if worlds == 0 {
                    continue;
                }
                if row[i] == '^' {
                    new_beams[i - 1] += worlds;
                    new_beams[i + 1] += worlds;
                } else {
                    new_beams[i] += worlds;
                }
            }
            beams = new_beams;
        }

        beams.iter().sum()
    }
}

aoc_utils::run!(2025, 7, Day07);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2025, 7, Day07);

    #[rstest]
    fn part1_test(data1: <Day07 as Solution>::Input<'_>) {
        assert_eq!(21, Day07::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day07 as Solution>::Input<'_>) {
        assert_eq!(40, Day07::part2(&data1));
    }
}
