use std::collections::VecDeque;

use aoc_utils::Solution;
use aoc_utils::grid::{Grid, Pos};

struct Day20;

impl Solution for Day20 {
    type Input<'a> = Vec<Pos>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let map = Grid::parse(input, |c| c);
        let start = map.find(&'S').expect("No start position found");
        let end = map.find(&'E').expect("No end position found");
        let track = map.map(|&c| matches!(c, '.' | 'S' | 'E'));

        bfs(&track, start, end)
    }

    fn part1(path: &Self::Input<'_>) -> Self::Output1 {
        count_good_shortcuts(path, 2)
    }

    fn part2(path: &Self::Input<'_>) -> Self::Output2 {
        count_good_shortcuts(path, 20)
    }
}

aoc_utils::run!(2024, 20, Day20);

fn bfs<'a>(map: &Grid<bool>, start: Pos, end: Pos) -> <Day20 as Solution>::Input<'a> {
    let mut visited = Grid::new(map.width(), map.height(), false);
    let mut parents = Grid::new(map.width(), map.height(), None);
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited[start] = true;

    while let Some(pos) = queue.pop_front() {
        if pos == end {
            break;
        }

        for next in map.neighbors4(pos) {
            if map[next] && !visited[next] {
                queue.push_back(next);
                visited[next] = true;
                parents[next] = Some(pos);
            }
        }
    }

    let mut path = vec![end];
    while let Some(pos) = parents[path[path.len() - 1]] {
        path.push(pos);
    }

    path.reverse();

    path
}

fn count_good_shortcuts(path: &[Pos], max_length: usize) -> usize {
    let mut shortcuts = Vec::new();

    for i in 0..(path.len() - max_length - 1) {
        let pos1 = path[i];
        for (j, pos2) in path.iter().enumerate().skip(i + max_length + 1) {
            let dist = pos1.manhattan(*pos2);
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
