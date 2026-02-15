use aoc_utils::Solution;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

struct Day12;

impl Solution for Day12 {
    type Input<'a> = Vec<(Vec<(usize, usize)>, Vec<((usize, usize), Direction)>)>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let map: Vec<Vec<_>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();

        let rows = map.len();
        let cols = if rows > 0 { map[0].len() } else { 0 };

        let new_rows = rows + 2;
        let new_cols = cols + 2;

        let mut enhanced_map = vec![vec!['.'; new_cols]; new_rows];

        for i in 0..rows {
            for j in 0..cols {
                enhanced_map[i + 1][j + 1] = map[i][j];
            }
        }

        get_areas_perimeters(&enhanced_map)
    }

    fn part1(areas_perimeters: &Self::Input<'_>) -> Self::Output1 {
        areas_perimeters
            .iter()
            .map(|(area, perimeter)| area.len() * perimeter.len())
            .sum()
    }

    fn part2(areas_perimeters: &Self::Input<'_>) -> Self::Output2 {
        areas_perimeters
            .iter()
            .map(|(area, perimeter)| {
                let mut sides: Vec<((&usize, &usize), &Direction)> = Vec::new();

                for ((x, y), dir) in perimeter {
                    let side = sides
                        .iter()
                        .find_position(|((s_x, s_y), s_dir)| {
                            dir == *s_dir
                                && ((y == *s_y && s_x.abs_diff(*x) == 1)
                                    || (x == *s_x && s_y.abs_diff(*y) == 1))
                        })
                        .map(|(i, _)| i);

                    if let Some(side) = side {
                        sides[side] = ((x, y), dir);
                    } else {
                        sides.push(((x, y), dir));
                    }
                }

                area.len() * sides.len()
            })
            .sum()
    }
}

aoc_utils::run!(2024, 12, Day12);

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn get_areas_perimeters(map: &[Vec<char>]) -> <Day12 as Solution>::Input<'_> {
    let mut unvisited = (1..map[0].len() - 1)
        .cartesian_product(1..map.len() - 1)
        .collect::<HashSet<_>>();

    let mut areas_perimeters = Vec::new();

    while let Some(&start) = unvisited.iter().next() {
        let mut area = HashSet::new();
        let mut perimeter = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back(start);
        area.insert(start);
        unvisited.remove(&start);

        while let Some(pos) = queue.pop_front() {
            let region = map[pos.1][pos.0];

            let next = vec![
                ((pos.0, pos.1 - 1), Direction::North),
                ((pos.0 + 1, pos.1), Direction::East),
                ((pos.0, pos.1 + 1), Direction::South),
                ((pos.0 - 1, pos.1), Direction::West),
            ];

            for (next_pos, direction) in next {
                let new_region = map[next_pos.1][next_pos.0];

                if new_region == region {
                    if unvisited.remove(&next_pos) {
                        queue.push_back(next_pos);
                        area.insert(next_pos);
                    }
                } else {
                    perimeter.insert((next_pos, direction));
                }
            }
        }

        let area = area
            .into_iter()
            .sorted_by(|(x1, y1), (x2, y2)| {
                let y_cmp = y1.cmp(y2);
                if y_cmp == Ordering::Equal {
                    x1.cmp(x2)
                } else {
                    y_cmp
                }
            })
            .collect();
        let perimeter = perimeter
            .into_iter()
            .sorted_by(|((x1, y1), dir1), ((x2, y2), dir2)| {
                let dir_cmp = dir1.cmp(dir2);
                if dir_cmp == Ordering::Equal {
                    let y_cmp = y1.cmp(y2);
                    if y_cmp == Ordering::Equal {
                        x1.cmp(x2)
                    } else {
                        y_cmp
                    }
                } else {
                    dir_cmp
                }
            })
            .collect();

        areas_perimeters.push((map[start.1][start.0], (area, perimeter)));
    }

    areas_perimeters
        .into_iter()
        .sorted_by_key(|&(region, _)| region)
        .map(|(_region, area_perimeter)| area_perimeter)
        .collect()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // Note: The automatic test input fetching retrieves incorrect data.
    aoc_utils::solution_tests!(2024, 12, Day12);

    #[rstest]
    fn part1_test1(data1: <Day12 as Solution>::Input<'_>) {
        assert_eq!(140, Day12::part1(&data1));
    }

    #[rstest]
    fn part1_test2(data2: <Day12 as Solution>::Input<'_>) {
        assert_eq!(772, Day12::part1(&data2));
    }

    #[rstest]
    fn part1_test3(data3: <Day12 as Solution>::Input<'_>) {
        assert_eq!(1930, Day12::part1(&data3));
    }

    #[rstest]
    fn part2_test1(data1: <Day12 as Solution>::Input<'_>) {
        assert_eq!(80, Day12::part2(&data1));
    }

    #[rstest]
    fn part2_test2(data2: <Day12 as Solution>::Input<'_>) {
        assert_eq!(436, Day12::part2(&data2));
    }

    #[rstest]
    fn part2_test3(data3: <Day12 as Solution>::Input<'_>) {
        assert_eq!(1206, Day12::part2(&data3));
    }

    #[rstest]
    fn part2_test4(data4: <Day12 as Solution>::Input<'_>) {
        assert_eq!(236, Day12::part2(&data4));
    }

    #[rstest]
    fn part2_test5(data5: <Day12 as Solution>::Input<'_>) {
        assert_eq!(368, Day12::part2(&data5));
    }
}
