use itertools::Itertools;
use nalgebra::{Matrix3, Matrix6, SMatrix, Vector3, Vector6};
use num::rational::Ratio;
use num::{BigInt, BigRational, Signed, Zero};
use regex::Regex;
use std::hash::Hash;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 24);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

// Floats cause rounding errors and i128 is too small, so we use BigInt instead.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Hail {
    id: usize,
    p: Vector3<BigRational>,
    v: Vector3<BigRational>,
}

impl Hail {
    fn from(
        id: usize,
        x: BigInt,
        y: BigInt,
        z: BigInt,
        dx: BigInt,
        dy: BigInt,
        dz: BigInt,
    ) -> Self {
        let p = Vector3::new(x.into(), y.into(), z.into());
        let v = Vector3::new(dx.into(), dy.into(), dz.into());
        Hail { id, p, v }
    }

    /// Given two lines, the intersection is a point that both have in common, so where the following two equations hold:
    /// s.x + u * s.dx = o.x + v * o.dx
    /// s.y + u * s.dy = o.y + v * o.dy
    ///
    /// Solving the system of equations for u and v gives the formulas used below for u and v.
    fn intersect(&self, other: &Hail) -> Option<(BigRational, BigRational)> {
        let det = other.v.x.clone() * self.v.y.clone() - other.v.y.clone() * self.v.x.clone();
        if det.is_zero() {
            // lines are parallel or coincident
            return None;
        }

        let dx = other.p.x.clone() - self.p.x.clone();
        let dy = other.p.y.clone() - self.p.y.clone();
        let u = (dy.clone() * other.v.x.clone() - dx.clone() * other.v.y.clone()) / det.clone();
        let v = (dy.clone() * self.v.x.clone() - dx.clone() * self.v.y.clone()) / det.clone();
        if u.is_negative() || v.is_negative() {
            // lines intersect in the past
            return None;
        }

        let x = self.p.x.clone() + u.clone() * self.v.x.clone();
        let y = self.p.y.clone() + u.clone() * self.v.y.clone();
        Some((x, y))
    }
}

fn process_input(input: &'static str) -> Vec<Hail> {
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

fn count_intersections_in_region(hail: &[Hail], low: &BigRational, high: &BigRational) -> usize {
    hail.iter()
        .combinations(2)
        .filter(|hail_pair| {
            hail_pair[0]
                .intersect(hail_pair[1])
                .is_some_and(|(x, y)| (low..=high).contains(&&x) && (low..=high).contains(&&y))
        })
        .count()
}

fn part1(hail: &[Hail]) -> usize {
    count_intersections_in_region(
        hail,
        &BigInt::from(200_000_000_000_000i64).into(),
        &BigInt::from(400_000_000_000_000i64).into(),
    )
}

/// Inverts a 6×6 matrix of `BigRational` using Gauss-Jordan elimination with partial pivoting using:
/// [Gauss–Jordan Elimination](https://en.wikipedia.org/wiki/Gaussian_elimination#Finding_the_inverse_of_a_matrix)
///
/// We can't use the `nalgebra` crate because it doesn't support `BigRational`.
///
/// Returns `None` if the matrix is singular, otherwise returns the inverse.
pub fn invert_matrix_6(m: &Matrix6<BigRational>) -> Option<Matrix6<BigRational>> {
    // We will build an augmented matrix of size 6 x 12: [m | I]
    // By applying row operations to reduce it to [I | m^-1]

    // Create the augmented matrix.
    let mut aug = SMatrix::<BigRational, 6, 12>::zeros();
    for r in 0..6 {
        for c in 0..6 {
            aug[(r, c)] = m[(r, c)].clone();
        }
        aug[(r, 6 + r)] = BigInt::from(1).into();
    }

    // Perform the Gauss-Jordan elimination.
    for i in 0..6 {
        // Find pivot = row with the largest absolute value in column i.
        let mut pivot_row = i;
        let mut pivot_val = aug[(i, i)].abs();
        for r in (i + 1)..6 {
            let candidate = aug[(r, i)].abs();
            if candidate > pivot_val {
                pivot_val = candidate;
                pivot_row = r;
            }
        }

        // If we found a zero pivot, the matrix is singular (not invertible).
        if aug[(pivot_row, i)].is_zero() {
            return None;
        }

        // Swap the pivot row into position i, if needed.
        if pivot_row != i {
            for c in 0..12 {
                aug.swap((i, c), (pivot_row, c));
            }
        }

        // Scale the pivot row so that the pivot element becomes 1.
        let pivot_element = aug[(i, i)].clone();
        for c in i..12 {
            aug[(i, c)] = aug[(i, c)].clone() / pivot_element.clone();
        }

        // Eliminate the pivot column in all other rows.
        for r in 0..6 {
            if r != i {
                let factor = aug[(r, i)].clone();
                // Subtract factor * pivot-row from row r.
                for c in i..12 {
                    aug[(r, c)] = aug[(r, c)].clone() - factor.clone() * aug[(i, c)].clone();
                }
            }
        }
    }

    // Extract the inverse from the right half of aug.
    let mut inv = Matrix6::<BigRational>::zeros();
    for r in 0..6 {
        for c in 0..6 {
            inv[(r, c)] = aug[(r, 6 + c)].clone();
        }
    }

    Some(inv)
}

fn cross_matrix(v: &Vector3<Ratio<BigInt>>) -> Matrix3<BigRational> {
    Matrix3::new(
        BigInt::from(0).into(),
        -v[2].clone(),
        v[1].clone(),
        v[2].clone(),
        BigInt::from(0).into(),
        -v[0].clone(),
        -v[1].clone(),
        v[0].clone(),
        BigInt::from(0).into(),
    )
}

/// Part2 is based on <https://www.reddit.com/r/adventofcode/comments/18pnycy/comment/kepu26z/>.
///
/// Since the stone must intersect all hailstones, and we are working in 3D, we can use the first three hailstones to solve the problem.
/// The following equations must hold:
/// p + t * v = p_i + t_i * v_i
/// p - p_i = t_i * v_i - t * v
/// (p - p_i) x (v - v_i) = 0
fn part2(hail: &[Hail]) -> BigInt {
    let mut m = Matrix6::<BigRational>::zeros();
    let mut rhs = Vector6::<BigRational>::zeros();

    rhs.fixed_rows_mut::<3>(0)
        .copy_from(&(-hail[0].p.cross(&hail[0].v) + hail[1].p.cross(&hail[1].v)));

    rhs.fixed_rows_mut::<3>(3)
        .copy_from(&(-hail[0].p.cross(&hail[0].v) + hail[2].p.cross(&hail[2].v)));

    m.fixed_view_mut::<3, 3>(0, 0)
        .copy_from(&(cross_matrix(&hail[0].v) - cross_matrix(&hail[1].v)));

    m.fixed_view_mut::<3, 3>(3, 0)
        .copy_from(&(cross_matrix(&hail[0].v) - cross_matrix(&hail[2].v)));

    m.fixed_view_mut::<3, 3>(0, 3)
        .copy_from(&(-cross_matrix(&hail[0].p) + cross_matrix(&hail[1].p)));

    m.fixed_view_mut::<3, 3>(3, 3)
        .copy_from(&(-cross_matrix(&hail[0].p) + cross_matrix(&hail[2].p)));

    let result = invert_matrix_6(&m).unwrap() * rhs;

    // Sum up the initial coordinates of the stone (not the velocity).
    result.iter().take(3).map(Ratio::to_integer).sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> Vec<Hail> {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<Hail>) {
        assert_eq!(
            count_intersections_in_region(&data, &BigInt::from(7).into(), &BigInt::from(27).into()),
            2
        );
    }

    #[rstest]
    fn part2_test(data: Vec<Hail>) {
        assert_eq!(part2(&data), BigInt::from(47));
    }
}
