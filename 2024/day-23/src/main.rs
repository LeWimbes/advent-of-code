use std::collections::{HashMap, HashSet};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 23);

type ParsedInput = Vec<(&'static str, &'static str)>;
type ProcessedInput = (Vec<Vec<bool>>, Vec<&'static str>);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.split_once('-').unwrap())
        .collect()
}

fn process_input(input: &'static str) -> ProcessedInput {
    let edges = parse_input(input);

    let mut next_id = 0;
    let mut vertices: HashMap<&str, usize> = HashMap::new();

    for (a, b) in &edges {
        if !vertices.contains_key(a) {
            vertices.insert(a, next_id);
            next_id += 1;
        }
        if !vertices.contains_key(b) {
            vertices.insert(b, next_id);
            next_id += 1;
        }
    }

    let mut adj: Vec<Vec<bool>> = vec![vec![false; vertices.len()]; vertices.len()];

    for (a, b) in edges {
        let a_idx = vertices[&a];
        let b_idx = vertices[&b];

        adj[a_idx][b_idx] = true;
        adj[b_idx][a_idx] = true;
    }

    let mut sorted_vertices: Vec<&str> = vec![""; vertices.len()];
    for (name, idx) in vertices {
        sorted_vertices[idx] = name;
    }

    (adj, sorted_vertices)
}

fn part1((adj, vertices): &ProcessedInput) -> usize {
    let mut historian_candidates: Vec<usize> = Vec::new();
    for (idx, name) in vertices.iter().enumerate() {
        if name.starts_with('t') {
            historian_candidates.push(idx);
        }
    }

    let mut cliques_with_historian = HashSet::new();

    for a in historian_candidates {
        for b in 0..adj.len() {
            for c in 0..adj.len() {
                if adj[a][b] && adj[b][c] && adj[c][a] {
                    let mut abc = [a, b, c];
                    abc.sort_unstable();
                    cliques_with_historian.insert((abc[0], abc[1], abc[2]));
                }
            }
        }
    }

    cliques_with_historian.len()
}

fn find_largest_clique(adj: &Vec<Vec<bool>>) -> Vec<usize> {
    let mut r: Vec<usize> = Vec::new();
    let mut p: Vec<usize> = (0..adj.len()).collect();
    let mut x: Vec<usize> = Vec::new();

    let mut max_clique: Vec<usize> = Vec::new();

    bron_kerbosch(&mut r, &mut p, &mut x, adj, &mut max_clique);

    max_clique
}

fn bron_kerbosch(
    r: &mut Vec<usize>,
    p: &mut Vec<usize>,
    x: &mut Vec<usize>,
    adjacency: &Vec<Vec<bool>>,
    max_clique: &mut Vec<usize>,
) {
    // found maximal clique
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            max_clique.clone_from(r);
        }
        return;
    }

    let p_snapshot = p.clone();
    for v in p_snapshot {
        // R ⋃ {v}
        r.push(v);

        // P ∩ N(v)
        let mut p_intersect_nv = Vec::new();
        for &u in p.iter() {
            if adjacency[v][u] {
                p_intersect_nv.push(u);
            }
        }

        // X ∩ N(v)
        let mut x_intersect_nv = Vec::new();
        for &u in x.iter() {
            if adjacency[v][u] {
                x_intersect_nv.push(u);
            }
        }

        bron_kerbosch(
            r,
            &mut p_intersect_nv,
            &mut x_intersect_nv,
            adjacency,
            max_clique,
        );

        // backtrack
        r.pop();

        // move v from P to X
        p.retain(|&u| u != v);
        x.push(v);
    }
}

fn part2((adj, vertices): &ProcessedInput) -> String {
    let largest_clique = find_largest_clique(adj);

    let mut clique_names: Vec<&str> = largest_clique
        .into_iter()
        .map(|idx| vertices[idx])
        .collect();

    clique_names.sort_unstable();
    clique_names.join(",")
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> ProcessedInput {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: ProcessedInput) {
        assert_eq!(part1(&data), 7);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2(&data), "co,de,ka,ta");
    }
}
