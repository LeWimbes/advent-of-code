use std::collections::VecDeque;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 20);

type ParsedInput = Vec<Vec<char>>;
type ProcessedInput = Vec<(usize, usize)>;

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn bfs(map: &[Vec<bool>], start: &(usize, usize), end: &(usize, usize)) -> Vec<(usize, usize)> {
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

fn process_input(input: &'static str) -> ProcessedInput {
    let map = parse_input(input);

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

fn count_good_shortcuts(path: &[(usize, usize)], max_length: usize) -> usize {
    let mut shortcuts = Vec::new();

    for i in 0..(path.len() - max_length - 1) {
        let pos1 = path[i];
        for j in (i + max_length + 1)..path.len() {
            let pos2 = path[j];

            let dist = pos1.0.abs_diff(pos2.0) + pos1.1.abs_diff(pos2.1);
            if dist <= max_length {
                shortcuts.push(j - i - dist);
            }
        }
    }

    shortcuts.into_iter().filter(|dist| *dist >= 100).count()
}

fn part1(path: &ProcessedInput) -> usize {
    count_good_shortcuts(path, 2)
}

fn part2(path: &ProcessedInput) -> usize {
    count_good_shortcuts(path, 20)
}

#[cfg(all(test, feature = "test-answers"))]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_macros::test_answers!(true);
}
