use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

use aoc_utils::Solution;
use aoc_utils::grid::{Direction, Grid, Pos};

struct Day16;

impl Solution for Day16 {
    type Input<'a> = (usize, usize);
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let map = Grid::parse(input, |c| c);

        // Each combination of position and direction is a node in the graph
        let id =
            |pos: Pos, dir: Direction| map.flat_index(pos) * Direction::ALL4.len() + dir as usize;

        let mut adj_map = vec![Vec::new(); map.width() * map.height() * Direction::ALL4.len()];

        for (pos, _) in map.iter().filter(|&(_pos, &tile)| tile != '#') {
            for dir in Direction::ALL4 {
                let adj = &mut adj_map[id(pos, dir)];
                adj.push((id(pos, dir.turn_right()), 1000));
                adj.push((id(pos, dir.turn_left()), 1000));
                if let Some(next) = map.step(pos, dir)
                    && map[next] != '#'
                {
                    adj.push((id(next, dir), 1));
                }
            }
        }

        let start = id(map.find(&'S').unwrap(), Direction::East);
        let end = map.find(&'E').unwrap();
        let ends = Direction::ALL4.map(|dir| id(end, dir));

        run_exhaustive_dijkstra(&adj_map, start, &ends)
    }

    fn part1((min, _tiles_on_shortest_paths): &Self::Input<'_>) -> Self::Output1 {
        *min
    }

    fn part2((_min, tiles_on_shortest_paths): &Self::Input<'_>) -> Self::Output2 {
        *tiles_on_shortest_paths
    }
}

aoc_utils::run!(2024, 16, Day16);

fn run_exhaustive_dijkstra(
    adj: &[Vec<(usize, usize)>],
    start: usize,
    ends: &[usize],
) -> (usize, usize) {
    let mut distance = vec![(usize::MAX, Vec::new()); adj.len()];
    let mut heap = BinaryHeap::new();

    distance[start].0 = 0;
    heap.push(Reverse((0, start)));

    while let Some(Reverse((dist, id))) = heap.pop() {
        if dist > distance[id].0 {
            continue;
        }

        for &(next, cost) in &adj[id] {
            let next_dist = dist + cost;

            if next_dist == distance[next].0 {
                distance[next].1.push(id);
            } else if next_dist < distance[next].0 {
                heap.push(Reverse((next_dist, next)));
                distance[next].0 = next_dist;
                distance[next].1.clear();
                distance[next].1.push(id);
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
        if explored.insert(tile) {
            // Nodes of the same position only differ in direction
            tiles_on_shortest_paths.insert(tile / Direction::ALL4.len());
            tiles_to_explore.extend(&distance[tile].1);
        }
    }

    (min, tiles_on_shortest_paths.len())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 16, Day16, examples = [1, 3]);

    #[rstest]
    fn part1_test1(data1: <Day16 as Solution>::Input<'_>) {
        assert_eq!(7036, Day16::part1(&data1));
    }

    #[rstest]
    fn part1_test2(data2: <Day16 as Solution>::Input<'_>) {
        assert_eq!(11_048, Day16::part1(&data2));
    }

    #[rstest]
    fn part2_test1(data1: <Day16 as Solution>::Input<'_>) {
        assert_eq!(45, Day16::part2(&data1));
    }

    #[rstest]
    fn part2_test2(data2: <Day16 as Solution>::Input<'_>) {
        assert_eq!(64, Day16::part2(&data2));
    }
}
