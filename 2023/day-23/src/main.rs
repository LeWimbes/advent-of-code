use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 23);

fn main() {
    let data1 = process_input(INPUT.2);
    println!("Part1: {}", part1(&data1));
    let data2 = process_input(INPUT.2);
    println!("Part2: {}", part2(&data2));
}

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

fn generate_graph(map: &[Vec<char>]) -> Graph {
    let bounds = (map[0].len(), map.len());

    // The iteration order ensures that the first node (0) is the start node and the last node is the end node (nodes.len() - 1).
    let nodes: Vec<(usize, usize, char)> = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, tile)| (*tile != '#').then_some((x, y, *tile)))
        })
        .collect();
    let indices: HashMap<(usize, usize), usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, (x, y, _tile))| ((*x, *y), i))
        .collect();
    let n = nodes.len();
    let mut graph = Graph::from(n);

    for (u, tile) in nodes.iter().enumerate() {
        let (x, y, tile) = *tile;

        if (tile == '.' || tile == '<') && x != 0 && map[y][x - 1] != '#' {
            graph.add_edge(u, indices[&(x - 1, y)]);
        }
        if (tile == '.' || tile == '^') && y != 0 && map[y - 1][x] != '#' {
            graph.add_edge(u, indices[&(x, y - 1)]);
        }
        if (tile == '.' || tile == '>') && x + 1 != bounds.0 && map[y][x + 1] != '#' {
            graph.add_edge(u, indices[&(x + 1, y)]);
        }
        if (tile == '.' || tile == 'v') && y + 1 != bounds.1 && map[y + 1][x] != '#' {
            graph.add_edge(u, indices[&(x, y + 1)]);
        }
    }

    graph.simplify(graph.n - 1);

    graph
}

fn process_input(input: &'static str) -> (Graph, Graph) {
    let map_with_slopes: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let simple_map: Vec<Vec<char>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|char| if char == '#' { '#' } else { '.' })
                .collect()
        })
        .collect();

    (
        generate_graph(&map_with_slopes),
        generate_graph(&simple_map),
    )
}

fn part1((graph, _): &(Graph, Graph)) -> usize {
    graph.longest_path(0, graph.n - 1).unwrap()
}

fn part2((_, graph): &(Graph, Graph)) -> usize {
    graph.exhaustive_dfs(0, graph.n - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data1() -> (Graph, Graph) {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[fixture]
    fn data2() -> (Graph, Graph) {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data1: (Graph, Graph)) {
        assert_eq!(part1(&data1), 94);
    }

    #[rstest]
    fn part2_test(data2: (Graph, Graph)) {
        assert_eq!(part2(&data2), 154);
    }
}
