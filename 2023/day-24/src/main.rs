use std::hash::Hash;
use std::time::Instant;

use itertools::Itertools;
use nalgebra::{Matrix3, Matrix6, Vector3, Vector6};
use regex::Regex;

// Part2 is based on https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kepu26z/

fn main() {
    let now = Instant::now();
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));

    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Hail {
    id: usize,
    p: Vector3<i64>,
    v: Vector3<i64>,
}

impl Hail {
    fn from(id: usize, x: i64, y: i64, z: i64, dx: i64, dy: i64, dz: i64) -> Self {
        let p = Vector3::new(x, y, z);
        let v = Vector3::new(dx, dy, dz);
        Hail { id, p, v }
    }
    /// Given two lines, the intersection is a point that both have in common, so where the following two equations hold:
    /// s.x + u * s.dx = o.x + v * o.dx
    /// s.y + u * s.dy = o.y + v * o.dy
    ///
    /// Solving the system of equations for u and v gives the formulas used below for u and v.
    fn intersect(&self, other: &Hail) -> Option<(f64, f64)> {
        let det = other.v.x * self.v.y - other.v.y * self.v.x;
        if det == 0 {
            // lines are parallel or coincident
            return None;
        }

        let det = det as f64;
        let dx = other.p.x - self.p.x;
        let dy = other.p.y - self.p.y;
        let u = (dy * other.v.x - dx * other.v.y) as f64 / det;
        let v = (dy * self.v.x - dx * self.v.y) as f64 / det;
        if u < 0.0 || v < 0.0 {
            // lines intersect in the past
            return None;
        }

        let x = self.p.x as f64 + u * self.v.x as f64;
        let y = self.p.y as f64 + u * self.v.y as f64;
        Some((x, y))
    }
}

fn process_input(input: &str) -> Vec<Hail> {
    let re = Regex::new(r"(?<x>-?\d+),\s*(?<y>-?\d+),\s*(?<z>-?\d+)\s*@\s*(?<dx>-?\d+),\s*(?<dy>-?\d+),\s*(?<dz>-?\d+)").unwrap();
    re.captures_iter(input)
        .enumerate()
        .map(|(id, caps)| {
            let x = caps["x"].parse().unwrap();
            let y = caps["y"].parse().unwrap();
            let z = caps["z"].parse().unwrap();
            let dx = caps["dx"].parse().unwrap();
            let dy = caps["dy"].parse().unwrap();
            let dz = caps["dz"].parse().unwrap();
            Hail::from(id, x, y, z, dx, dy, dz)
        })
        .collect()
}

fn count_intersections_in_region(hail: &[Hail], low: f64, high: f64) -> usize {
    hail.iter()
        .combinations(2)
        .filter(|hail_pair| {
            hail_pair[0]
                .intersect(hail_pair[1])
                .is_some_and(|(x, y)| (low..=high).contains(&x) && (low..=high).contains(&y))
        })
        .count()
}

fn part1(hail: &[Hail]) -> usize {
    count_intersections_in_region(hail, 200000000000000.0, 400000000000000.0)
}

fn cross_matrix(v: Vector3<f64>) -> Matrix3<f64> {
    Matrix3::new(0.0, -v[2], v[1], v[2], 0.0, -v[0], -v[1], v[0], 0.0)
}

// There are some rounding errors; add 2 to get the correct result... :/
fn part2(hail: &[Hail]) -> i64 {
    let p0 = hail[0].p.map(|x| x as f64);
    let v0 = hail[0].v.map(|x| x as f64);
    let p1 = hail[1].p.map(|x| x as f64);
    let v1 = hail[1].v.map(|x| x as f64);
    let p2 = hail[2].p.map(|x| x as f64);
    let v2 = hail[2].v.map(|x| x as f64);

    let mut m = Matrix6::<f64>::zeros();
    let mut rhs = Vector6::<f64>::zeros();

    rhs.fixed_rows_mut::<3>(0)
        .copy_from(&(-p0.cross(&v0) + p1.cross(&v1)));

    rhs.fixed_rows_mut::<3>(3)
        .copy_from(&(-p0.cross(&v0) + p2.cross(&v2)));

    m.fixed_view_mut::<3, 3>(0, 0)
        .copy_from(&(cross_matrix(v0) - cross_matrix(v1)));

    m.fixed_view_mut::<3, 3>(3, 0)
        .copy_from(&(cross_matrix(v0) - cross_matrix(v2)));

    m.fixed_view_mut::<3, 3>(0, 3)
        .copy_from(&(-cross_matrix(p0) + cross_matrix(p1)));

    m.fixed_view_mut::<3, 3>(3, 3)
        .copy_from(&(-cross_matrix(p0) + cross_matrix(p2)));

    let result = m.try_inverse().unwrap() * rhs;

    result.iter().take(3).map(|&x| x.round() as i64).sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn data() -> Vec<Hail> {
        let input = include_str!("input_test.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<Hail>) {
        assert_eq!(count_intersections_in_region(&data, 7.0, 27.0), 2);
    }

    #[rstest]
    fn part2_test(data: Vec<Hail>) {
        assert_eq!(part2(&data), 47);
    }
}
