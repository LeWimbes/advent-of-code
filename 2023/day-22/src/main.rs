use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use regex::Regex;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 22);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Brick {
    id: usize,
    min: (usize, usize, usize),
    max: (usize, usize, usize),
    bottom: Vec<(usize, usize, usize)>,
    top: Vec<(usize, usize, usize)>,
}

impl Brick {
    fn from(id: usize, x0: usize, y0: usize, z0: usize, x1: usize, y1: usize, z1: usize) -> Self {
        let bottom = (x0..=x1)
            .cartesian_product(y0..=y1)
            .map(|(x, y)| (x, y, z0))
            .collect();
        let top = (x0..=x1)
            .cartesian_product(y0..=y1)
            .map(|(x, y)| (x, y, z1))
            .collect();
        Brick {
            id,
            min: (x0, y0, z0),
            max: (x1, y1, z1),
            bottom,
            top,
        }
    }

    fn fall(&mut self, fall_dist: usize) {
        self.min.2 -= fall_dist;
        self.max.2 -= fall_dist;
        for cube in &mut self.bottom {
            cube.2 -= fall_dist;
        }
        for cube in &mut self.top {
            cube.2 -= fall_dist;
        }
    }

    // fn contains(&self, coord: &(usize, usize, usize)) -> bool {
    //     coord.0 >= self.min.0
    //         && coord.0 <= self.max.0
    //         && coord.1 >= self.min.1
    //         && coord.1 <= self.max.1
    //         && coord.2 >= self.min.2
    //         && coord.2 <= self.max.2
    // }
}

// fn print_stack_side(bricks_max: &(Vec<Brick>, (usize, usize, usize)), looking_in_y_dir: bool) {
//     let (bricks, max_xyz) = bricks_max;
//
//     let side: Vec<usize>;
//     let depth: Vec<usize>;
//     let to_coord: fn(usize, usize, usize) -> (usize, usize, usize);
//     if looking_in_y_dir {
//         side = (0..=max_xyz.0).collect();
//         depth = (0..=max_xyz.1).collect();
//         to_coord = |z: usize, side: usize, depth: usize| (side, depth, z);
//     } else {
//         side = (0..=max_xyz.1).collect();
//         depth = (0..=max_xyz.0).collect();
//         to_coord = |z: usize, side: usize, depth: usize| (depth, side, z);
//     }
//     let id_map = if bricks.len() <= 26 {
//         |id: usize| ((b'A' + id as u8) as char).to_string()
//     } else {
//         |id: usize| id.to_string()
//     };
//
//     let z_label_level = (max_xyz.2 + 1) / 2;
//     if looking_in_y_dir {
//         println!("{}x", " ".repeat(side.len() / 2));
//     } else {
//         println!("{}y", " ".repeat(side.len() / 2));
//     }
//     println!("{}", side.iter().join(""));
//     for z in (1..=max_xyz.2).rev() {
//         for s in &side {
//             let bricks_here: Vec<usize> = depth
//                 .iter()
//                 .filter_map(|d| {
//                     let coord = to_coord(z, *s, *d);
//                     bricks
//                         .iter()
//                         .find_map(|brick| brick.contains(&coord).then_some(brick.id))
//                 })
//                 .unique()
//                 .collect();
//             if bricks_here.is_empty() {
//                 print!(".");
//             } else if bricks_here.len() == 1 {
//                 print!("{}", id_map(bricks_here[0]));
//             } else {
//                 print!("?");
//             }
//         }
//         print!(" {z}");
//         if z == z_label_level {
//             print!(" z");
//         }
//         println!();
//     }
//     println!("{} 0", "-".repeat(side.len()));
//     println!();
// }

// fn print_stack(bricks_max: &(Vec<Brick>, (usize, usize, usize))) {
//     print_stack_side(bricks_max, true);
//     print_stack_side(bricks_max, false);
// }

type ProcessedData = (
    HashMap<usize, HashSet<usize>>,
    HashMap<usize, HashSet<usize>>,
);

fn process_input(input: &'static str) -> ProcessedData {
    let re =
        Regex::new(r"(?<x0>\d+),(?<y0>\d+),(?<z0>\d+)~(?<x1>\d+),(?<y1>\d+),(?<z1>\d+)").unwrap();
    let mut bricks: Vec<Brick> = re
        .captures_iter(input)
        .enumerate()
        .map(|(id, caps)| {
            let x0 = caps["x0"].parse().unwrap();
            let y0 = caps["y0"].parse().unwrap();
            let z0 = caps["z0"].parse().unwrap();
            let x1 = caps["x1"].parse().unwrap();
            let y1 = caps["y1"].parse().unwrap();
            let z1 = caps["z1"].parse().unwrap();
            Brick::from(id, x0, y0, z0, x1, y1, z1)
        })
        .collect();

    let mut max_xyz = bricks.iter().fold((0, 0, 0), |acc, brick| {
        (
            acc.0.max(brick.max.0),
            acc.1.max(brick.max.1),
            acc.2.max(brick.max.2),
        )
    });

    // sort by z
    bricks.sort_by(|a, b| a.min.2.cmp(&b.min.2).then(a.id.cmp(&b.id)));

    // let the bricks settle
    let mut next_valid = vec![vec![1usize; max_xyz.0 + 1]; max_xyz.1 + 1];
    // skip floor; let all others fall
    for brick in &mut bricks {
        let new_bottom_z = brick
            .bottom
            .iter()
            .map(|(x, y, _z)| next_valid[*y][*x])
            .max()
            .unwrap();
        let fall_dist = brick.bottom[0].2 - new_bottom_z;

        brick.fall(fall_dist);
        for (x, y, z) in &brick.top {
            next_valid[*y][*x] = z + 1;
        }
    }
    max_xyz.2 = bricks.iter().map(|brick| brick.max.2).max().unwrap();

    let supporting: HashMap<usize, HashSet<usize>> = bricks
        .par_iter()
        .map(|brick| {
            let z = brick.max.2 + 1;
            let supported = bricks
                .iter()
                .filter_map(|other| {
                    (brick.id != other.id
                        && other.min.2 == z
                        && brick.min.0 <= other.max.0
                        && other.min.0 <= brick.max.0
                        && brick.min.1 <= other.max.1
                        && other.min.1 <= brick.max.1)
                        .then_some(other.id)
                })
                .collect();
            (brick.id, supported)
        })
        .collect();

    let supported_by: HashMap<usize, HashSet<usize>> = bricks
        .par_iter()
        .map(|brick| {
            let z = brick.min.2 - 1;
            let supported = bricks
                .iter()
                .filter_map(|other| {
                    (brick.id != other.id
                        && other.max.2 == z
                        && brick.min.0 <= other.max.0
                        && other.min.0 <= brick.max.0
                        && brick.min.1 <= other.max.1
                        && other.min.1 <= brick.max.1)
                        .then_some(other.id)
                })
                .collect();
            (brick.id, supported)
        })
        .collect();

    (supporting, supported_by)
}

fn part1(supporting_supported_by: &ProcessedData) -> usize {
    let (supporting, supported_by) = supporting_supported_by;

    supporting
        .keys()
        .par_bridge()
        .filter(|id| {
            supporting[*id]
                .iter()
                .all(|supported| supported_by[supported].len() >= 2)
        })
        .count()
}

fn part2(supporting_supported_by: &ProcessedData) -> usize {
    let (supporting, supported_by) = supporting_supported_by;

    supporting
        .keys()
        .par_bridge()
        .map(|id| {
            let mut falling = HashSet::new();
            falling.insert(*id);
            let mut check_next = Vec::new();
            check_next.push(*id);

            while let Some(id) = check_next.pop() {
                let will_fall: Vec<usize> = supporting[&id]
                    .iter()
                    .filter(|id| supported_by[*id].is_subset(&falling))
                    .copied()
                    .collect();
                falling.extend(will_fall.iter());
                check_next.extend(will_fall);
            }

            falling.len() - 1
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> ProcessedData {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: ProcessedData) {
        assert_eq!(part1(&data), 5);
    }

    #[rstest]
    fn part2_test(data: ProcessedData) {
        assert_eq!(part2(&data), 7);
    }
}
