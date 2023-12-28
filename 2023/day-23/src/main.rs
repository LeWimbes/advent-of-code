use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Instant;

use itertools::Itertools;
use rayon::iter::{ParallelBridge, ParallelIterator};

fn main() {
    let now = Instant::now();
    let input = include_str!("input.txt");

    let data1 = process_input(input, false);
    println!("Part1: {}", part1(&data1));
    let data2 = process_input(input, true);
    println!("Part2: {}", part2(&data2));

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Graph {
    n: usize,
    m: usize,
    adj: Vec<Vec<usize>>,
}

impl Graph {
    fn from(n: usize) -> Graph {
        Graph { n, m: 0, adj: vec![vec![]; n] }
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
        let mut stack: Vec<(isize, isize)> = vec![];

        stack.push((start as isize, 0));

        while let Some((s, cost)) = stack.pop() {
            let s_index = s as usize;

            if s < 0 { // finished vertex
                let vertex = (-s - 1) as usize; // invert back
                visited[vertex] = !visited[vertex];
            } else if !visited[s_index] { // new vertex
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

        let to_remove: Vec<(usize, Vec<usize>)> = (0..self.n).par_bridge().map(|node| {
            let to_remove: Vec<usize> = self.adj[node].iter()
                .filter(|neighbor| !self.can_reach_end(node, **neighbor, end))
                .copied()
                .collect();
            (node, to_remove)
        }).collect();

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

        let mut stack: Vec<isize> = vec![];

        while let Some(start) = unvisited.iter().next().copied() {
            stack.push(start as isize);

            while let Some(s) = stack.pop() {
                let s_index = s as usize;

                if s < 0 { // finished vertex
                    let vertex = (-s - 1) as usize; // invert back
                    state[vertex] = 2;
                    ordering.push_front(vertex);
                } else if state[s_index] == 0 { // new vertex
                    state[s_index] = 1;
                    unvisited.remove(&s_index);

                    stack.push(-s - 1); // 0 == -0, therefore - 1
                    for neighbor in &self.adj[s_index] {
                        let neighbor = *neighbor;
                        if state[neighbor] != 2 {
                            stack.push(neighbor as isize);
                        }
                    }
                } else { // found cycle
                    return None;
                }
            }
        }

        Some(ordering)
    }
}

fn process_input(input: &str, ignore_slopes: bool) -> Graph {
    let map: Vec<Vec<char>> = input.lines()
        .map(|line| {
            if ignore_slopes {
                line.chars().map(|char| if char == '#' { '#' } else { '.' }).collect()
            } else {
                line.chars().collect()
            }
        }).collect();
    let bounds = (map[0].len(), map.len());

    // The iteration order ensures that the first node (0) is the start node and the last node is the end node (nodes.len() - 1).
    let nodes: Vec<(usize, usize, char)> = map.iter().enumerate()
        .flat_map(|(y, row)|
            row.iter().enumerate()
                .filter_map(move |(x, tile)| {
                    (*tile != '#').then_some((x, y, *tile))
                }))
        .collect();
    let indices: HashMap<(usize, usize), usize> = nodes.iter().enumerate()
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

fn part1(graph: &Graph) -> usize {
    graph.longest_path(0, graph.n - 1).unwrap()
}

fn part2(graph: &Graph) -> usize {
    graph.exhaustive_dfs(0, graph.n - 1).unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn data1() -> Graph {
        let input = include_str!("input_test.txt");
        process_input(input, false)
    }

    #[fixture]
    fn data2() -> Graph {
        let input = include_str!("input_test.txt");
        process_input(input, true)
    }

    #[rstest]
    fn part1_test(data1: Graph) {
        assert_eq!(part1(&data1), 94);
    }

    #[rstest]
    fn part2_test(data2: Graph) {
        assert_eq!(part2(&data2), 154);
    }
}
