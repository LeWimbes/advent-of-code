use std::collections::VecDeque;

use aoc_utils::Solution;

struct Day20;

impl Solution for Day20 {
    type Input<'a> = Vec<(usize, usize)>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let map: Vec<Vec<_>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();

        let mut enhanced_map = vec![vec![false; map[0].len()]; map.len()];
        let mut start = (0, 0);
        let mut end = (0, 0);

        for (y, row) in map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    '.' => enhanced_map[y][x] = true,
                    'S' => {
                        enhanced_map[y][x] = true;
                        start = (x, y);
                    }
                    'E' => {
                        enhanced_map[y][x] = true;
                        end = (x, y);
                    }
                    _ => {}
                }
            }
        }

        bfs(&enhanced_map, &start, &end)
    }

    fn part1(path: &Self::Input<'_>) -> Self::Output1 {
        count_good_shortcuts(path, 2)
    }

    fn part2(path: &Self::Input<'_>) -> Self::Output2 {
        count_good_shortcuts(path, 20)
    }
}

aoc_utils::run!(2024, 20, Day20);

fn bfs<'a>(
    map: &[Vec<bool>],
    start: &(usize, usize),
    end: &(usize, usize),
) -> <Day20 as Solution>::Input<'a> {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut parents = vec![vec![None; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();

    queue.push_back(*start);
    visited[start.1][start.0] = true;

    while let Some(pos) = queue.pop_front() {
        if pos == *end {
            break;
        }
        let (x, y) = pos;

        let nexts = [(x, y - 1), (x + 1, y), (x, y + 1), (x - 1, y)];

        for next in nexts {
            if map[next.1][next.0] && !visited[next.1][next.0] {
                queue.push_back(next);
                visited[next.1][next.0] = true;
                parents[next.1][next.0] = Some(pos);
            }
        }
    }

    let mut path = vec![*end];
    while let Some(pos) = parents[path[path.len() - 1].1][path[path.len() - 1].0] {
        path.push(pos);
    }

    path.reverse();

    path
}

fn count_good_shortcuts(path: &[(usize, usize)], max_length: usize) -> usize {
    let mut shortcuts = Vec::new();

    for i in 0..(path.len() - max_length - 1) {
        let pos1 = path[i];
        for (j, pos2) in path.iter().enumerate().skip(i + max_length + 1) {
            let dist = pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1);
            if dist <= max_length {
                shortcuts.push(j - i - dist);
            }
        }
    }

    shortcuts.into_iter().filter(|dist| *dist >= 100).count()
}

#[cfg(all(test, feature = "test-answers"))]
mod tests {
    use super::*;

    aoc_utils::solution_tests!(2024, 20, Day20);
}
