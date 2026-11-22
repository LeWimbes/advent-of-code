use std::collections::HashSet;

use aoc_utils::Solution;
use aoc_utils::grid::{Grid, Pos};

struct Day10;

impl Solution for Day10 {
    type Input<'a> = (usize, usize);
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let map = Grid::parse(input, |ch| ch.to_digit(10).unwrap());

        let trailheads = map
            .iter()
            .filter(|&(_, &height)| height == 0)
            .map(|(pos, _)| pos)
            .collect::<Vec<_>>();

        run_dfs(&map, &trailheads)
    }

    fn part1((trailtails, _distinct_trails): &Self::Input<'_>) -> Self::Output1 {
        *trailtails
    }

    fn part2((_trailtails, distinct_trails): &Self::Input<'_>) -> Self::Output2 {
        *distinct_trails
    }
}

aoc_utils::run!(2024, 10, Day10);

enum VisitState {
    Pre,
    Post,
}

fn run_dfs<'a>(map: &Grid<u32>, trailheads: &[Pos]) -> <Day10 as Solution>::Input<'a> {
    trailheads
        .iter()
        .map(|&trailhead| {
            let mut visited = map.map(|_| false);

            let mut trailtails = HashSet::new();

            let mut distinct_trails = 0;

            let mut stack = vec![(trailhead, VisitState::Pre)];
            visited[trailhead] = true;

            while let Some((pos, state)) = stack.pop() {
                match state {
                    VisitState::Pre => {
                        stack.push((pos, VisitState::Post));

                        let height = map[pos];

                        for next_pos in map.neighbors4(pos) {
                            if !visited[next_pos] && map[next_pos] == height + 1 {
                                if map[next_pos] == 9 {
                                    trailtails.insert(next_pos);
                                    distinct_trails += 1;
                                } else {
                                    stack.push((next_pos, VisitState::Pre));
                                    visited[next_pos] = true;
                                }
                            }
                        }
                    }
                    VisitState::Post => {
                        visited[pos] = false;
                    }
                }
            }

            (trailtails.len(), distinct_trails)
        })
        .fold((0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 10, Day10, examples = [5]);

    #[rstest]
    fn part1_test(data1: <Day10 as Solution>::Input<'_>) {
        assert_eq!(36, Day10::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day10 as Solution>::Input<'_>) {
        assert_eq!(81, Day10::part2(&data1));
    }
}
