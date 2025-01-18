use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 12);

type ParsedInput = Vec<Vec<char>>;
type ProcessedInput = Vec<(Vec<(usize, usize)>, Vec<((usize, usize), Direction)>)>;

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn get_areas_perimeters(map: &[Vec<char>]) -> ProcessedInput {
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

fn process_input(input: &'static str) -> ProcessedInput {
    let map = parse_input(input);

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

fn part1(areas_perimeters: &ProcessedInput) -> usize {
    areas_perimeters
        .iter()
        .map(|(area, perimeter)| area.len() * perimeter.len())
        .sum()
}

fn part2(areas_perimeters: &ProcessedInput) -> usize {
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

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data1() -> ProcessedInput {
        let input = include_str!("test_input1.txt");
        process_input(input)
    }

    #[fixture]
    fn data2() -> ProcessedInput {
        let input = include_str!("test_input2.txt");
        process_input(input)
    }

    #[fixture]
    fn data3() -> ProcessedInput {
        let input = include_str!("test_input3.txt");
        process_input(input)
    }

    #[fixture]
    fn data4() -> ProcessedInput {
        let input = include_str!("test_input4.txt");
        process_input(input)
    }

    #[fixture]
    fn data5() -> ProcessedInput {
        let input = include_str!("test_input5.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test1(data1: ProcessedInput) {
        assert_eq!(part1(&data1), 140);
    }

    #[rstest]
    fn part1_test2(data2: ProcessedInput) {
        assert_eq!(part1(&data2), 772);
    }

    #[rstest]
    fn part1_test3(data3: ProcessedInput) {
        assert_eq!(part1(&data3), 1930);
    }

    #[rstest]
    fn part2_test1(data1: ProcessedInput) {
        assert_eq!(part2(&data1), 80);
    }

    #[rstest]
    fn part2_test2(data2: ProcessedInput) {
        assert_eq!(part2(&data2), 436);
    }

    #[rstest]
    fn part2_test3(data3: ProcessedInput) {
        assert_eq!(part2(&data3), 1206);
    }

    #[rstest]
    fn part2_test4(data4: ProcessedInput) {
        assert_eq!(part2(&data4), 236);
    }

    #[rstest]
    fn part2_test5(data5: ProcessedInput) {
        assert_eq!(part2(&data5), 368);
    }
}
