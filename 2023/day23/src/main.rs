use std::collections::{HashSet, VecDeque};

use aoc_utils::Solution;
use aoc_utils::grid::{Direction, Grid, Pos};
use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

struct Day23;

impl Solution for Day23 {
    type Input<'a> = (Graph, Graph);
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let map_with_slopes = Grid::parse(input, |c| c);
        let simple_map = map_with_slopes.map(|&c| if c == '#' { '#' } else { '.' });

        (
            generate_graph(&map_with_slopes),
            generate_graph(&simple_map),
        )
    }

    fn part1((graph, _): &Self::Input<'_>) -> Self::Output1 {
        graph.longest_path(0, graph.n - 1).unwrap()
    }

    fn part2((_, graph): &Self::Input<'_>) -> Self::Output2 {
        graph.exhaustive_dfs(0, graph.n - 1).unwrap()
    }
}

aoc_utils::run!(2023, 23, Day23);

#[derive(Debug, Clone, Eq, PartialEq)]
struct Graph {
    n: usize,
    m: usize,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn from(n: usize) -> Graph {
        Graph {
            n,
            m: 0,
            adj: vec![Vec::new(); n],
        }
    }

    fn add_edge(&mut self, u: usize, v: usize) {
        self.adj[u].push(v);
        self.m += 1;
    }

    fn remove_edge(&mut self, u: usize, v: usize) {
        let index = self.adj[u].iter().find_position(|neighbor| **neighbor == v);
        if let Some((index, _)) = index {
            self.adj[u].swap_remove(index);
            self.m -= 1;
        }
    }

    fn longest_path(&self, start: usize, end: usize) -> Option<usize> {
        if let Some(sorting) = self.get_topological_sorting() {
            let mut distances: Vec<isize> = vec![isize::MIN; self.n];
            distances[start] = 0;

            for node in sorting {
                if distances[node] != isize::MIN {
                    let new_dist = distances[node] + 1;
                    for neighbor in &self.adj[node] {
                        distances[*neighbor] = distances[*neighbor].max(new_dist);
                    }
                }
            }

            (distances[end] != isize::MIN).then_some(distances[end] as usize)
        } else {
            self.exhaustive_dfs(start, end)
        }
    }

    fn exhaustive_dfs(&self, start: usize, end: usize) -> Option<usize> {
        if start == end {
            return Some(0);
        }

        let mut dist = isize::MIN;

        let mut visited = vec![false; self.n];
        let mut stack: Vec<(isize, isize)> = Vec::new();

        stack.push((start as isize, 0));

        while let Some((s, cost)) = stack.pop() {
            let s_index = s as usize;

            if s < 0 {
                // finished vertex
                let vertex = (-s - 1) as usize; // invert back
                visited[vertex] = !visited[vertex];
            } else if !visited[s_index] {
                // new vertex
                visited[s_index] = true;
                stack.push((-s - 1, 0)); // 0 == -0, therefore - 1
                let new_cost = cost + 1;
                for neighbor in &self.adj[s_index] {
                    let neighbor = *neighbor;
                    if !visited[neighbor] {
                        if neighbor == end {
                            dist = dist.max(new_cost);
                            continue;
                        }
                        stack.push((neighbor as isize, new_cost));
                    }
                }
            }
        }

        (dist != isize::MIN).then_some(dist as usize)
    }

    /// Removes neighbors that do not allow reaching the end
    fn simplify(&mut self, end: usize) {
        self.adj[end].clear();

        let to_remove: Vec<(usize, Vec<usize>)> = (0..self.n)
            .par_bridge()
            .map(|node| {
                let to_remove: Vec<usize> = self.adj[node]
                    .iter()
                    .filter(|neighbor| !self.can_reach_end(node, **neighbor, end))
                    .copied()
                    .collect();
                (node, to_remove)
            })
            .collect();

        for (node, to_remove) in to_remove {
            for neighbor in to_remove {
                self.remove_edge(node, neighbor);
            }
        }
    }

    fn can_reach_end(&self, start: usize, neighbor: usize, end: usize) -> bool {
        if start == end || neighbor == end {
            return true;
        }

        let mut queue: VecDeque<usize> = VecDeque::new();
        let mut visited = vec![false; self.n];

        visited[start] = true;
        visited[neighbor] = true;
        queue.push_back(neighbor);
        while let Some(cur) = queue.pop_front() {
            for neighbor in &self.adj[cur] {
                if !visited[*neighbor] {
                    if *neighbor == end {
                        return true;
                    }
                    visited[*neighbor] = true;
                    queue.push_back(*neighbor);
                }
            }
        }

        false
    }

    fn get_topological_sorting(&self) -> Option<VecDeque<usize>> {
        let mut state: Vec<u8> = vec![0; self.n];

        // required in case there are multiple components
        let mut unvisited: HashSet<usize> = (0..self.n).collect();

        let mut ordering: VecDeque<usize> = VecDeque::with_capacity(self.n);

        let mut stack: Vec<isize> = Vec::new();

        while let Some(start) = unvisited.iter().next().copied() {
            stack.push(start as isize);

            while let Some(s) = stack.pop() {
                let s_index = s as usize;

                if s < 0 {
                    // finished vertex
                    let vertex = (-s - 1) as usize; // invert back
                    state[vertex] = 2;
                    ordering.push_front(vertex);
                } else if state[s_index] == 0 {
                    // new vertex
                    state[s_index] = 1;
                    unvisited.remove(&s_index);

                    stack.push(-s - 1); // 0 == -0, therefore - 1
                    for neighbor in &self.adj[s_index] {
                        let neighbor = *neighbor;
                        if state[neighbor] != 2 {
                            stack.push(neighbor as isize);
                        }
                    }
                } else {
                    // found cycle
                    return None;
                }
            }
        }

        Some(ordering)
    }
}

fn generate_graph(map: &Grid<char>) -> Graph {
    // The iteration order ensures that the first node (0) is the start node and the last node is the end node (nodes.len() - 1).
    let nodes: Vec<Pos> = map
        .iter()
        .filter(|&(_pos, &tile)| tile != '#')
        .map(|(pos, _)| pos)
        .collect();
    let mut indices = map.map(|_| None);
    for (i, &pos) in nodes.iter().enumerate() {
        indices[pos] = Some(i);
    }

    let mut graph = Graph::from(nodes.len());

    for (u, &pos) in nodes.iter().enumerate() {
        let directions: &[Direction] = match map[pos] {
            '^' => &[Direction::North],
            '>' => &[Direction::East],
            'v' => &[Direction::South],
            '<' => &[Direction::West],
            _ => &Direction::ALL4,
        };

        for &dir in directions {
            if let Some(v) = map.step(pos, dir).and_then(|next| indices[next]) {
                graph.add_edge(u, v);
            }
        }
    }

    graph.simplify(graph.n - 1);

    graph
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 23, Day23, examples = [1]);

    #[rstest]
    fn part1_test(data1: <Day23 as Solution>::Input<'_>) {
        assert_eq!(94, Day23::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day23 as Solution>::Input<'_>) {
        assert_eq!(154, Day23::part2(&data1));
    }
}
