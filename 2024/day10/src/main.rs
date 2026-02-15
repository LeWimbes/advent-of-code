use std::collections::HashSet;

use aoc_utils::Solution;

struct Day10;

impl Solution for Day10 {
    type Input<'a> = (usize, usize);
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let map: Vec<Vec<_>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
            .collect();

        let rows = map.len();
        let cols = if rows > 0 { map[0].len() } else { 0 };

        let new_rows = rows + 2;
        let new_cols = cols + 2;

        let mut enhanced_map = vec![vec![u32::MAX; new_cols]; new_rows];

        for i in 0..rows {
            for j in 0..cols {
                enhanced_map[i + 1][j + 1] = map[i][j];
            }
        }

        let trailheads = enhanced_map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, height)| if *height == 0 { Some((x, y)) } else { None })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        run_dfs(&enhanced_map, &trailheads)
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

fn run_dfs<'a>(map: &[Vec<u32>], trailheads: &[(usize, usize)]) -> <Day10 as Solution>::Input<'a> {
    trailheads
        .iter()
        .map(|trailhead| {
            let mut visited = vec![vec![false; map[0].len()]; map.len()];

            let mut trailtails = HashSet::new();

            let mut distinct_trails = 0;

            let mut stack = vec![(*trailhead, VisitState::Pre)];
            visited[trailhead.1][trailhead.0] = true;

            while let Some((pos, state)) = stack.pop() {
                match state {
                    VisitState::Pre => {
                        stack.push((pos, VisitState::Post));

                        let height = map[pos.1][pos.0];

                        let next = vec![
                            (pos.0, pos.1 - 1),
                            (pos.0 + 1, pos.1),
                            (pos.0, pos.1 + 1),
                            (pos.0 - 1, pos.1),
                        ];

                        for next_pos in next {
                            if !visited[next_pos.1][next_pos.0] {
                                let new_height = map[next_pos.1][next_pos.0];
                                if new_height == height + 1 {
                                    if new_height == 9 {
                                        trailtails.insert(next_pos);
                                        distinct_trails += 1;
                                    } else {
                                        stack.push((next_pos, VisitState::Pre));
                                        visited[next_pos.1][next_pos.0] = true;
                                    }
                                }
                            }
                        }
                    }
                    VisitState::Post => {
                        visited[pos.1][pos.0] = false;
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

    // Note: The automatic test input fetching retrieves incorrect data.
    aoc_utils::solution_tests!(2024, 10, Day10);

    #[rstest]
    fn part1_test(data1: <Day10 as Solution>::Input<'_>) {
        assert_eq!(36, Day10::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day10 as Solution>::Input<'_>) {
        assert_eq!(81, Day10::part2(&data1));
    }
}
