use std::collections::HashSet;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 10);

type ParsedInput = Vec<Vec<u32>>;
type ProcessedInput = (usize, usize);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|ch| ch.to_digit(10).unwrap()).collect())
        .collect()
}

enum VisitState {
    Pre,
    Post,
}

fn run_dfs(map: &[Vec<u32>], trailheads: &[(usize, usize)]) -> ProcessedInput {
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

fn process_input(input: &'static str) -> ProcessedInput {
    let map = parse_input(input);

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

fn part1((trailtails, _distinct_trails): &ProcessedInput) -> usize {
    *trailtails
}

fn part2((_trailtails, distinct_trails): &ProcessedInput) -> usize {
    *distinct_trails
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
        assert_eq!(part1(&data), 36);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2(&data), 81);
    }
}
