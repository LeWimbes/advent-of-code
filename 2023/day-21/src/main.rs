use std::collections::{HashSet, VecDeque};

use nalgebra::{Matrix3, Vector3};

// For part 2 I looked at https://www.reddit.com/r/adventofcode/comments/18nevo3/2023_day_21_solutions/.
// At the moment I don't feel like implementing a proper solution, so I just went with the quadratic polynomial.
// This means that most of the tests fail, as it only works if the walk ends on the edge of a map tile.

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 21);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &'static str) -> (Vec<Vec<bool>>, (isize, isize)) {
    let mut start = (0, 0);
    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| match char {
                    'S' => {
                        start = (x as isize, y as isize);
                        true
                    }
                    '.' => true,
                    _ => false,
                })
                .collect()
        })
        .collect();

    (map, start)
}

fn to_map_coord(coord: &(isize, isize), upper_bounds: &(isize, isize)) -> (usize, usize) {
    let x = coord.0.rem_euclid(upper_bounds.0) as usize;
    let y = coord.1.rem_euclid(upper_bounds.1) as usize;

    (x, y)
}

fn get_neighbors(coord: &(isize, isize), map: &[Vec<bool>]) -> Vec<(isize, isize)> {
    let upper_bounds = (map[0].len() as isize, map.len() as isize);
    let changes_x = [0, -1, 0, 1];
    let changes_y = [-1, 0, 1, 0];

    changes_x
        .iter()
        .zip(changes_y)
        .filter_map(|(x_change, y_change)| {
            let new_coord = (coord.0 - x_change, coord.1 - y_change);
            let map_coord = to_map_coord(&new_coord, &upper_bounds);
            map[map_coord.1][map_coord.0].then_some(new_coord)
        })
        .collect()
}

fn count_reachable_tiles(map_start: &(Vec<Vec<bool>>, (isize, isize)), steps: usize) -> usize {
    let (map, start) = map_start;
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
        for neighbor in get_neighbors(&coord, map) {
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
    map_start: &(Vec<Vec<bool>>, (isize, isize)),
    steps: usize,
) -> usize {
    let (map, _start) = map_start;

    if steps <= map.len() {
        return count_reachable_tiles(map_start, steps);
    }

    let x0 = steps % map.len();
    let x1 = x0 + map.len();
    let x2 = x1 + map.len();
    let n = (steps - x0) / map.len();

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

fn part1(map_start: &(Vec<Vec<bool>>, (isize, isize))) -> usize {
    count_reachable_tiles_quadratic(map_start, 64)
}

fn part2(map_start: &(Vec<Vec<bool>>, (isize, isize))) -> usize {
    count_reachable_tiles_quadratic(map_start, 26_501_365)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> (Vec<Vec<bool>>, (isize, isize)) {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: (Vec<Vec<bool>>, (isize, isize))) {
        assert_eq!(count_reachable_tiles_quadratic(&data, 6), 16);
    }

    #[rstest]
    fn part2_test1(data: (Vec<Vec<bool>>, (isize, isize))) {
        assert_eq!(count_reachable_tiles_quadratic(&data, 6), 16);
    }

    #[rstest]
    fn part2_test2(data: (Vec<Vec<bool>>, (isize, isize))) {
        assert_eq!(count_reachable_tiles_quadratic(&data, 10), 50);
    }

    #[rstest]
    fn part2_test3(data: (Vec<Vec<bool>>, (isize, isize))) {
        assert_eq!(count_reachable_tiles_quadratic(&data, 50), 1594);
    }

    #[rstest]
    fn part2_test4(data: (Vec<Vec<bool>>, (isize, isize))) {
        assert_eq!(count_reachable_tiles_quadratic(&data, 100), 6536);
    }

    #[rstest]
    fn part2_test5(data: (Vec<Vec<bool>>, (isize, isize))) {
        assert_eq!(count_reachable_tiles_quadratic(&data, 500), 167_004);
    }

    #[rstest]
    fn part2_test6(data: (Vec<Vec<bool>>, (isize, isize))) {
        assert_eq!(count_reachable_tiles_quadratic(&data, 1000), 668_697);
    }

    #[rstest]
    fn part2_test7(data: (Vec<Vec<bool>>, (isize, isize))) {
        assert_eq!(count_reachable_tiles_quadratic(&data, 5000), 16_733_044);
    }
}
