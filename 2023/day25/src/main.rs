use std::collections::HashMap;

use aoc_utils::Solution;

struct Day25;

impl Solution for Day25 {
    type Input<'a> = Graph;
    type Output1 = usize;
    type Output2 = &'static str;

    fn process(input: &str) -> Self::Input<'_> {
        let mut next_id = 0;
        let mut ids: HashMap<&str, usize> = HashMap::new();
        let mut adj: Vec<(&str, Vec<&str>)> = Vec::new();

        for line in input.lines() {
            let (source, children) = line.split_once(": ").unwrap();
            let children: Vec<&str> = children.split(' ').collect();
            if !ids.contains_key(source) {
                ids.insert(source, next_id);
                next_id += 1;
            }
            for child in &children {
                if !ids.contains_key(child) {
                    ids.insert(child, next_id);
                    next_id += 1;
                }
            }
            adj.push((source, children));
        }

        let mut graph = Graph::from(next_id);

        for (u, children) in adj {
            for v in children {
                graph.add_edge(ids[u], ids[v]);
            }
        }

        graph
    }

    fn part1(graph: &Self::Input<'_>) -> Self::Output1 {
        let (_cut_size, partition) = graph.min_cut();
        partition.len() * (graph.n - partition.len())
    }

    fn part2(_input: &Self::Input<'_>) -> Self::Output2 {
        // Day 25 has no part 2
        "N/A"
    }
}

aoc_utils::run!(2023, 25, Day25);

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
        self.adj[v].push(u);
        self.m += 2;
    }

    fn get_adjacency_matrix(&self) -> Vec<Vec<i32>> {
        let mut mat = vec![vec![0; self.n]; self.n];
        for (u, neighbors) in self.adj.iter().enumerate() {
            for v in neighbors {
                mat[u][*v] = 1;
            }
        }

        mat
    }

    /// Min-Cut calculation using:
    /// [Stoer–Wagner algorithm](https://en.wikipedia.org/wiki/Stoer–Wagner_algorithm)
    fn min_cut(&self) -> (i32, Vec<usize>) {
        let mut mat = self.get_adjacency_matrix();

        let mut best: (i32, Vec<usize>) = (i32::MAX, Vec::new());
        let mut co: Vec<Vec<usize>> = (0..self.n).map(|i| vec![i]).collect();

        for ph in 1..self.n {
            let mut w = mat[0].clone();
            let mut s = 0;
            let mut t = 0;

            for _ in 0..self.n - ph {
                w[t] = i32::MIN;
                s = t;
                t = w
                    .iter()
                    .enumerate()
                    .max_by_key(|&(_, &x)| x)
                    .map(|(i, _)| i)
                    .unwrap();

                for (i, wi) in w.iter_mut().enumerate().take(self.n) {
                    *wi += mat[t][i];
                }
            }

            let cot = co[t].clone();
            co[s].extend(cot.iter());
            best = best.min((w[t] - mat[t][t], cot));

            for i in 0..self.n {
                mat[s][i] += mat[t][i];
                mat[i][s] = mat[s][i];
            }

            mat[0][t] = i32::MIN;
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 25, Day25);

    #[rstest]
    fn part1_test(data1: <Day25 as Solution>::Input<'_>) {
        assert_eq!(54, Day25::part1(&data1));
    }
}
