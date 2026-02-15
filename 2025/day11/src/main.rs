use std::collections::{HashMap, VecDeque};

use aoc_utils::Solution;

struct Day11;

impl Solution for Day11 {
    type Input<'a> = Graph;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let parsed_input = input
            .lines()
            .map(|line| {
                let parts = line.split_once(": ").unwrap();
                let node = parts.0;
                let neighbors = parts.1.split(' ').collect::<Vec<_>>();
                (node, neighbors)
            })
            .collect::<Vec<_>>();

        let mut graph = Graph::new(parsed_input.len() + 4); // +4 for out, you/svr, dac, fft

        let mut node_indices = HashMap::with_capacity(parsed_input.len());
        node_indices.insert("you", YOU_NODE_ID);
        node_indices.insert("out", OUT_NODE_ID);
        node_indices.insert("svr", SVR_NODE_ID);
        node_indices.insert("dac", DAC_NODE_ID);
        node_indices.insert("fft", FFT_NODE_ID);
        let mut index = NEXT_NODE_ID;

        for (node, _) in &parsed_input {
            if !node_indices.contains_key(node) {
                node_indices.insert(node, index);
                index += 1;
            }
        }

        for (node, neighbors) in &parsed_input {
            let from_index = *node_indices.get(node).unwrap();
            for neighbor in neighbors {
                let to_index = *node_indices.get(neighbor).unwrap();
                graph.add_edge(from_index, to_index);
            }
        }

        graph
    }

    fn part1(graph: &Self::Input<'_>) -> Self::Output1 {
        graph.distinct_paths(YOU_NODE_ID, OUT_NODE_ID)
    }

    fn part2(graph: &Self::Input<'_>) -> Self::Output2 {
        graph.distinct_paths_through_waypoints(
            SVR_NODE_ID,
            OUT_NODE_ID,
            &[DAC_NODE_ID, FFT_NODE_ID],
        )
    }
}

aoc_utils::run!(2025, 11, Day11);

const YOU_NODE_ID: usize = 0;
const OUT_NODE_ID: usize = 1;
const SVR_NODE_ID: usize = 2;
const DAC_NODE_ID: usize = 3;
const FFT_NODE_ID: usize = 4;
const NEXT_NODE_ID: usize = 5;

struct Graph {
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    fn new(size: usize) -> Self {
        Self {
            adjacency_list: vec![Vec::new(); size],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        self.adjacency_list[from].push(to);
    }

    /// Compute topological order using Kahn's algorithm
    ///
    /// https://en.wikipedia.org/wiki/Topological_sorting#Kahn's_algorithm
    fn topological_order(&self) -> Vec<usize> {
        let n = self.adjacency_list.len();
        let mut in_degree = vec![0usize; n];

        for edges in &self.adjacency_list {
            for &to in edges {
                in_degree[to] += 1;
            }
        }

        // Initialize queue with nodes of in-degree 0
        let mut queue: VecDeque<usize> = in_degree
            .iter()
            .enumerate()
            .filter(|(_, d)| **d == 0)
            .map(|(i, _)| i)
            .collect();

        let mut order = Vec::with_capacity(n);
        while let Some(node) = queue.pop_front() {
            order.push(node);
            for &neighbor in &self.adjacency_list[node] {
                in_degree[neighbor] -= 1;
                if in_degree[neighbor] == 0 {
                    queue.push_back(neighbor);
                }
            }
        }

        order
    }

    /// Count distinct paths from start to end in a DAG using DP
    fn distinct_paths(&self, start: usize, end: usize) -> usize {
        let topo_order = self.topological_order();
        let n = self.adjacency_list.len();

        // paths[v] = number of paths from v to end
        let mut paths = vec![0usize; n];
        paths[end] = 1;

        // Process in reverse topological order
        for &node in topo_order.iter().rev() {
            for &neighbor in &self.adjacency_list[node] {
                paths[node] += paths[neighbor];
            }
        }

        paths[start]
    }

    /// Count distinct paths from start to end that pass through all waypoints (in any order)
    fn distinct_paths_through_waypoints(
        &self,
        start: usize,
        end: usize,
        waypoints: &[usize],
    ) -> usize {
        let topo_order = self.topological_order();
        let n = self.adjacency_list.len();
        let n_waypoints = waypoints.len();
        let n_states = 1 << n_waypoints;

        // Create a mapping from node to waypoint bit
        // Works with at most 64 waypoints
        let mut waypoint_bit = vec![0usize; n];
        for (i, &wp) in waypoints.iter().enumerate() {
            waypoint_bit[wp] = 1 << i;
        }

        // paths[node][mask] = number of paths from node to end, visited waypoints in mask
        let mut paths = vec![vec![0usize; n_states]; n];
        let end_mask = waypoint_bit[end];
        paths[end][end_mask] = 1;

        // Process in reverse topological order
        for &node in topo_order.iter().rev() {
            for &neighbor in &self.adjacency_list[node] {
                for mask in 0..n_states {
                    // Combine masks from neighbor + this node's bit
                    paths[node][mask | waypoint_bit[node]] += paths[neighbor][mask];
                }
            }
        }

        paths[start][(1 << n_waypoints) - 1]
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2025, 11, Day11);

    #[rstest]
    fn part1_test(data1: <Day11 as Solution>::Input<'_>) {
        assert_eq!(5, Day11::part1(&data1));
    }

    #[rstest]
    fn part2_test(data2: <Day11 as Solution>::Input<'_>) {
        assert_eq!(2, Day11::part2(&data2));
    }
}
