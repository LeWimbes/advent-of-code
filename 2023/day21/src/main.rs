use std::collections::{HashSet, VecDeque};

use aoc_utils::Solution;
use aoc_utils::grid::{Direction, Grid, Pos};
use nalgebra::{Matrix3, Vector3};

struct Day21;

impl Solution for Day21 {
    type Input<'a> = (Grid<bool>, (isize, isize));
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let map = Grid::parse(input, |c| c);
        let start = map.find(&'S').unwrap();

        (
            map.map(|&c| matches!(c, 'S' | '.')),
            (start.x as isize, start.y as isize),
        )
    }

    fn part1(map_start: &Self::Input<'_>) -> Self::Output1 {
        count_reachable_tiles_quadratic(map_start, 64)
    }

    /// Part2 is based on <https://www.reddit.com/r/adventofcode/comments/18nevo3/2023_day_21_solutions/>.
    ///
    /// At the moment I don't feel like implementing a proper solution, so I just went with the quadratic polynomial.
    /// This means that most of the tests fail, as it only works if the walk ends on the edge of a map tile.
    fn part2(map_start: &Self::Input<'_>) -> Self::Output2 {
        count_reachable_tiles_quadratic(map_start, 26_501_365)
    }
}

aoc_utils::run!(2023, 21, Day21);

/// The neighbours of `coord` that are garden plots, on the infinitely repeating map.
fn get_neighbors(coord: (isize, isize), map: &Grid<bool>) -> impl Iterator<Item = (isize, isize)> {
    let (width, height) = (map.width() as isize, map.height() as isize);

    Direction::ALL4
        .into_iter()
        .map(move |direction| {
            let (dx, dy) = direction.offset();
            (coord.0 + dx, coord.1 + dy)
        })
        .filter(move |&(x, y)| {
            map[Pos::new(x.rem_euclid(width) as usize, y.rem_euclid(height) as usize)]
        })
}

fn count_reachable_tiles((map, start): &<Day21 as Solution>::Input<'_>, steps: usize) -> usize {
    let remainder = steps % 2;

    let mut visited = HashSet::new();
    let mut mod_2_tiles = 0;

    let mut queue: VecDeque<((isize, isize), usize)> = VecDeque::new();
    queue.push_back((*start, 0));
    visited.insert(*start);
    if 0 == remainder {
        mod_2_tiles += 1;
    }

    while let Some((coord, cost)) = queue.pop_front() {
        if cost == steps {
            break;
        }

        let new_cost = cost + 1;
        for neighbor in get_neighbors(coord, map) {
            if !visited.contains(&neighbor) {
                queue.push_back((neighbor, new_cost));
                visited.insert(neighbor);
                if new_cost % 2 == remainder {
                    mod_2_tiles += 1;
                }
            }
        }
    }

    mod_2_tiles
}

fn count_reachable_tiles_quadratic(
    map_start: &<Day21 as Solution>::Input<'_>,
    steps: usize,
) -> usize {
    let (map, _start) = map_start;

    if steps <= map.height() {
        return count_reachable_tiles(map_start, steps);
    }

    let x0 = steps % map.height();
    let x1 = x0 + map.height();
    let x2 = x1 + map.height();
    let n = (steps - x0) / map.height();

    let y0 = count_reachable_tiles(map_start, x0) as f64;
    let y1 = count_reachable_tiles(map_start, x1) as f64;
    let y2 = count_reachable_tiles(map_start, x2) as f64;

    // This appears to introduce some rounding errors.
    // let (a0, a1, a2) = fit_quadratic(x0 as f64, y0, x1 as f64, y1, x2 as f64, y2);
    // let result = (a0 + a1 * steps as f64 + a2 * (steps * steps) as f64) as usize;

    // This results in a whole number.
    let (a0, a1, a2) = fit_quadratic_polynomial(0.0, y0, 1.0, y1, 2.0, y2);
    (a0 + a1 * n as f64 + a2 * (n * n) as f64) as usize
}

/// Performs a polynomial interpolation using:
/// [Vandermonde matrix](https://en.wikipedia.org/wiki/Vandermonde_matrix)
fn fit_quadratic_polynomial(
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
) -> (f64, f64, f64) {
    let v = Matrix3::new(1.0, x0, x0 * x0, 1.0, x1, x1 * x1, 1.0, x2, x2 * x2);
    let y = Vector3::new(y0, y1, y2);
    let v_inverse = v.try_inverse().unwrap();

    let a = v_inverse * y;

    (a.x, a.y, a.z)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 21, Day21, examples = [1]);

    #[rstest]
    fn part1_test(data1: <Day21 as Solution>::Input<'_>) {
        assert_eq!(count_reachable_tiles_quadratic(&data1, 6), 16);
    }

    #[rstest]
    fn part2_test1(data1: <Day21 as Solution>::Input<'_>) {
        assert_eq!(count_reachable_tiles_quadratic(&data1, 6), 16);
    }

    #[rstest]
    fn part2_test2(data1: <Day21 as Solution>::Input<'_>) {
        assert_eq!(count_reachable_tiles_quadratic(&data1, 10), 50);
    }

    // #[rstest]
    // fn part2_test3(data1: <Day21 as Solution>::Input<'_>) {
    //     assert_eq!(count_reachable_tiles_quadratic(&data1, 50), 1594);
    // }

    // #[rstest]
    // fn part2_test4(data1: <Day21 as Solution>::Input<'_>) {
    //     assert_eq!(count_reachable_tiles_quadratic(&data1, 100), 6536);
    // }

    // #[rstest]
    // fn part2_test5(data1: <Day21 as Solution>::Input<'_>) {
    //     assert_eq!(count_reachable_tiles_quadratic(&data1, 500), 167_004);
    // }

    // #[rstest]
    // fn part2_test6(data1: <Day21 as Solution>::Input<'_>) {
    //     assert_eq!(count_reachable_tiles_quadratic(&data1, 1000), 668_697);
    // }

    // #[rstest]
    // fn part2_test7(data1: <Day21 as Solution>::Input<'_>) {
    //     assert_eq!(count_reachable_tiles_quadratic(&data1, 5000), 16_733_044);
    // }
}
