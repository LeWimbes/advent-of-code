use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use aoc_utils::Solution;

struct Day08;

impl Solution for Day08 {
    type Input<'a> = (usize, usize);
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let coords: Vec<_> = input
            .lines()
            .map(|line| {
                let mut iter = line.split(',').map(|num| num.parse::<usize>().unwrap());
                let (x, y, z) = (
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                );
                Coordinate { x, y, z }
            })
            .collect::<Vec<_>>();

        // Determine connections based on number of coordinates
        // The test data has 20 coordinates
        let connections = if coords.len() == 20 { 10 } else { 1000 };

        let mut distances = BinaryHeap::with_capacity(coords.len() * (coords.len() - 1) / 2);
        for i in 0..coords.len() {
            for j in (i + 1)..coords.len() {
                let dist = coords[i].euclidean_distance_squared(&coords[j]);
                distances.push((Reverse(dist), i, j));
            }
        }

        let mut circuits: Vec<Vec<usize>> = Vec::new();
        let mut coord_circuit_map = HashMap::new();
        let mut connections_made = 0;
        let mut three_largest_circuits_product = 1;
        let mut last_i = 0;
        let mut last_j = 0;

        while let Some((_, i, j)) = distances.pop()
            && (circuits.len() != 1 || circuits[0].len() != coords.len())
        {
            connections_made += 1;
            last_i = i;
            last_j = j;

            match (coord_circuit_map.get(&i), coord_circuit_map.get(&j)) {
                (None, None) => {
                    let new_circuit_index = circuits.len();
                    circuits.push(vec![i, j]);
                    coord_circuit_map.insert(i, new_circuit_index);
                    coord_circuit_map.insert(j, new_circuit_index);
                }
                (None, Some(&cj)) => {
                    circuits[cj].push(i);
                    coord_circuit_map.insert(i, cj);
                }
                (Some(&ci), None) => {
                    circuits[ci].push(j);
                    coord_circuit_map.insert(j, ci);
                }
                (Some(&ci), Some(&cj)) if ci != cj => {
                    let (keep, remove) = if ci < cj { (ci, cj) } else { (cj, ci) };
                    let circuit_removed = circuits.remove(remove);
                    circuits[keep].extend(circuit_removed);

                    // Update mappings for coordinates in the merged circuit
                    for &coord in &circuits[keep] {
                        coord_circuit_map.insert(coord, keep);
                    }

                    // Update mappings for circuits that shifted down
                    for (_, idx) in coord_circuit_map.iter_mut() {
                        if *idx > remove {
                            *idx -= 1;
                        }
                    }
                }
                _ => {} // Already in the same circuit, skip
            }

            if connections_made == connections {
                let mut circuit_sizes: Vec<usize> =
                    circuits.iter().map(|circuit| circuit.len()).collect();
                circuit_sizes.sort_unstable_by(|a, b| b.cmp(a));
                three_largest_circuits_product = circuit_sizes.iter().take(3).product();
            }
        }

        (
            three_largest_circuits_product,
            coords[last_i].x * coords[last_j].x,
        )
    }

    fn part1((three_largest_circuits_product, _): &Self::Input<'_>) -> Self::Output1 {
        *three_largest_circuits_product
    }

    fn part2((_, last_connection_x_product): &Self::Input<'_>) -> Self::Output2 {
        *last_connection_x_product
    }
}

aoc_utils::run!(2025, 8, Day08);

struct Coordinate {
    x: usize,
    y: usize,
    z: usize,
}

impl Coordinate {
    fn euclidean_distance_squared(&self, other: &Coordinate) -> usize {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        dx * dx + dy * dy + dz * dz
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2025, 8, Day08);

    #[rstest]
    fn part1_test(data1: <Day08 as Solution>::Input<'_>) {
        assert_eq!(40, Day08::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day08 as Solution>::Input<'_>) {
        assert_eq!(25_272, Day08::part2(&data1));
    }
}
