use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 16);

type ParsedInput = Vec<Vec<char>>;
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
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

fn position_to_id(x: usize, y: usize, dir: Direction, width: usize, dirs: usize) -> usize {
    y * width * dirs + x * dirs + dir as usize
}

fn id_to_coord(id: usize, width: usize, dirs: usize) -> (usize, usize) {
    let coords = id / dirs;
    let y = coords / width;
    let x = coords % width;
    (x, y)
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
struct State {
    dist: usize,
    id: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run_exhaustive_dijkstra(
    adj: &[Vec<(usize, usize)>],
    start: usize,
    ends: &[usize],
    width: usize,
    dirs: usize,
) -> (usize, usize) {
    let mut distance = vec![(usize::MAX, Vec::new()); adj.len()];
    let mut heap = BinaryHeap::new();

    distance[start].0 = 0;
    heap.push(State { dist: 0, id: start });

    while let Some(State { dist, id }) = heap.pop() {
        if dist > distance[id].0 {
            continue;
        }

        for (next, cost) in &adj[id] {
            let next = State {
                dist: dist + *cost,
                id: *next,
            };

            if next.dist == distance[next.id].0 {
                distance[next.id].1.push(id);
            } else if next.dist < distance[next.id].0 {
                heap.push(next);
                distance[next.id].0 = next.dist;
                distance[next.id].1.clear();
                distance[next.id].1.push(id);
            }
        }
    }

    let min = ends.iter().map(|end| distance[*end].0).min().unwrap();
    let mut tiles_to_explore = ends
        .iter()
        .filter(|end| distance[**end].0 == min)
        .collect::<Vec<_>>();

    let mut explored = HashSet::new();
    let mut tiles_on_shortest_paths = HashSet::new();

    while let Some(&tile) = tiles_to_explore.pop() {
        let coord = id_to_coord(tile, width, dirs);
        if !explored.contains(&tile) {
            explored.insert(tile);
            tiles_on_shortest_paths.insert(coord);
            tiles_to_explore.extend(&distance[tile].1);
        }
    }

    (min, tiles_on_shortest_paths.len())
}

fn process_input(input: &'static str) -> ProcessedInput {
    let map = parse_input(input);

    let height = map.len();
    let width = map[0].len();
    let dirs = 4;

    let mut adj_map = vec![Vec::new(); height * width * dirs];
    let mut start = 0;
    let mut ends = Vec::new();

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let tile = map[y][x];

            if tile == '#' {
                continue;
            }

            if tile == 'S' {
                start = position_to_id(x, y, Direction::East, width, dirs);
            } else if tile == 'E' {
                ends = vec![
                    position_to_id(x, y, Direction::North, width, dirs),
                    position_to_id(x, y, Direction::East, width, dirs),
                    position_to_id(x, y, Direction::South, width, dirs),
                    position_to_id(x, y, Direction::West, width, dirs),
                ];
            }

            for dir in [
                Direction::North,
                Direction::East,
                Direction::South,
                Direction::West,
            ] {
                let adj = &mut adj_map[position_to_id(x, y, dir, width, dirs)];
                match dir {
                    Direction::North => {
                        adj.push((position_to_id(x, y, Direction::East, width, dirs), 1000));
                        adj.push((position_to_id(x, y, Direction::West, width, dirs), 1000));
                        if map[y - 1][x] != '#' {
                            adj.push((position_to_id(x, y - 1, dir, width, dirs), 1));
                        }
                    }
                    Direction::East => {
                        adj.push((position_to_id(x, y, Direction::South, width, dirs), 1000));
                        adj.push((position_to_id(x, y, Direction::North, width, dirs), 1000));
                        if map[y][x + 1] != '#' {
                            adj.push((position_to_id(x + 1, y, dir, width, dirs), 1));
                        }
                    }
                    Direction::South => {
                        adj.push((position_to_id(x, y, Direction::West, width, dirs), 1000));
                        adj.push((position_to_id(x, y, Direction::East, width, dirs), 1000));
                        if map[y + 1][x] != '#' {
                            adj.push((position_to_id(x, y + 1, dir, width, dirs), 1));
                        }
                    }
                    Direction::West => {
                        adj.push((position_to_id(x, y, Direction::South, width, dirs), 1000));
                        adj.push((position_to_id(x, y, Direction::North, width, dirs), 1000));
                        if map[y][x - 1] != '#' {
                            adj.push((position_to_id(x - 1, y, dir, width, dirs), 1));
                        }
                    }
                }
            }
        }
    }

    run_exhaustive_dijkstra(&adj_map, start, &ends, width, dirs)
}

fn part1((min, _tiles_on_shortest_paths): &ProcessedInput) -> usize {
    *min
}

fn part2((_min, tiles_on_shortest_paths): &ProcessedInput) -> usize {
    *tiles_on_shortest_paths
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data1() -> ProcessedInput {
        let input = include_str!("test_input1.txt");
        process_input(input)
    }

    #[fixture]
    fn data2() -> ProcessedInput {
        let input = include_str!("test_input2.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test1(data1: ProcessedInput) {
        assert_eq!(part1(&data1), 7036);
    }

    #[rstest]
    fn part1_test2(data2: ProcessedInput) {
        assert_eq!(part1(&data2), 11048);
    }

    #[rstest]
    fn part2_test1(data1: ProcessedInput) {
        assert_eq!(part2(&data1), 45);
    }

    #[rstest]
    fn part2_test2(data2: ProcessedInput) {
        assert_eq!(part2(&data2), 64);
    }
}
