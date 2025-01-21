use std::collections::{HashMap, HashSet, LinkedList};

// use colored::Colorize;
// use itertools::Itertools;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 10);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TileType {
    TopBottom,
    LeftRight,
    TopRight,
    TopLeft,
    BottomLeft,
    BottomRight,
    Ground,
    Start,
}

fn push_top_if_valid(
    area: &[Vec<TileType>],
    coord: (usize, usize),
    neighbors: &mut Vec<(usize, usize)>,
) {
    let (x, y) = coord;
    if y > 0 {
        let top = area[y - 1][x];
        if top == TileType::Start
            || top == TileType::TopBottom
            || top == TileType::BottomLeft
            || top == TileType::BottomRight
        {
            neighbors.push((x, y - 1));
        }
    }
}

fn push_bottom_if_valid(
    area: &[Vec<TileType>],
    coord: (usize, usize),
    neighbors: &mut Vec<(usize, usize)>,
) {
    let (x, y) = coord;
    if y < area.len() - 1 {
        let bottom = area[y + 1][x];
        if bottom == TileType::Start
            || bottom == TileType::TopBottom
            || bottom == TileType::TopRight
            || bottom == TileType::TopLeft
        {
            neighbors.push((x, y + 1));
        }
    }
}

fn push_left_if_valid(
    area: &[Vec<TileType>],
    coord: (usize, usize),
    neighbors: &mut Vec<(usize, usize)>,
) {
    let (x, y) = coord;
    if x > 0 {
        let left = area[y][x - 1];
        if left == TileType::Start
            || left == TileType::LeftRight
            || left == TileType::TopRight
            || left == TileType::BottomRight
        {
            neighbors.push((x - 1, y));
        }
    }
}

fn push_right_if_valid(
    area: &[Vec<TileType>],
    coord: (usize, usize),
    neighbors: &mut Vec<(usize, usize)>,
) {
    let (x, y) = coord;
    if x < area[y].len() - 1 {
        let right = area[y][x + 1];
        if right == TileType::Start
            || right == TileType::LeftRight
            || right == TileType::TopLeft
            || right == TileType::BottomLeft
        {
            neighbors.push((x + 1, y));
        }
    }
}

fn neighbors(area: &[Vec<TileType>], coord: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbors = Vec::new();
    let (x, y) = coord;

    match area[y][x] {
        TileType::TopBottom => {
            push_top_if_valid(area, coord, &mut neighbors);
            push_bottom_if_valid(area, coord, &mut neighbors);
        }
        TileType::LeftRight => {
            push_left_if_valid(area, coord, &mut neighbors);
            push_right_if_valid(area, coord, &mut neighbors);
        }
        TileType::TopRight => {
            push_top_if_valid(area, coord, &mut neighbors);
            push_right_if_valid(area, coord, &mut neighbors);
        }
        TileType::TopLeft => {
            push_top_if_valid(area, coord, &mut neighbors);
            push_left_if_valid(area, coord, &mut neighbors);
        }
        TileType::BottomLeft => {
            push_bottom_if_valid(area, coord, &mut neighbors);
            push_left_if_valid(area, coord, &mut neighbors);
        }
        TileType::BottomRight => {
            push_bottom_if_valid(area, coord, &mut neighbors);
            push_right_if_valid(area, coord, &mut neighbors);
        }
        TileType::Ground => {}
        TileType::Start => {
            push_top_if_valid(area, coord, &mut neighbors);
            push_bottom_if_valid(area, coord, &mut neighbors);
            push_left_if_valid(area, coord, &mut neighbors);
            push_right_if_valid(area, coord, &mut neighbors);
        }
    }

    neighbors
}

// fn pretty_print_area(area: &[Vec<TileType>], colors: &Vec<(Color, HashSet<&(usize, usize)>)>) {
//     let top_bottom = (0..area[0].len()).map(|_| '─').join("").bold().green();
//     println!("{}{}{}", "┌".bold().green(), top_bottom, "┐".bold().green());
//     area.iter().enumerate().for_each(|(y, row)| {
//         let row_str = row
//             .iter()
//             .enumerate()
//             .map(|(x, tile)| {
//                 let coord = (x, y);
//                 let tile_str = match tile {
//                     TileType::TopBottom => "│",
//                     TileType::LeftRight => "─",
//                     TileType::TopRight => "└",
//                     TileType::TopLeft => "┘",
//                     TileType::BottomLeft => "┐",
//                     TileType::BottomRight => "┌",
//                     TileType::Ground => " ",
//                     TileType::Start => "S",
//                 };
//                 if let Some(color) = colors.iter().find_map(|(color, tiles)| {
//                     if tiles.contains(&coord) {
//                         Some(color)
//                     } else {
//                         None
//                     }
//                 }) {
//                     if tile_str.trim().is_empty() {
//                         tile_str.on_color(*color)
//                     } else {
//                         tile_str.color(*color)
//                     }
//                 } else {
//                     tile_str.normal()
//                 }
//             })
//             .join("");
//
//         println!("{}{}{}", "│".bold().green(), row_str, "│".bold().green());
//     });
//     println!("{}{}{}", "└".bold().green(), top_bottom, "┘".bold().green());
// }

fn find_cycle(area: &[Vec<TileType>], start: &(usize, usize)) -> Option<Vec<(usize, usize)>> {
    let mut parent_map: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut stack: LinkedList<((usize, usize), (usize, usize))> = LinkedList::new();

    stack.push_back((*start, (usize::MAX, usize::MAX)));

    while let Some((current, parent)) = stack.pop_back() {
        if visited.contains(&current) {
            let mut last = parent_map[&current];
            let mut cycle: Vec<(usize, usize)> = vec![last];
            while &last != start {
                last = parent_map[&last];
                cycle.push(last);
            }
            cycle.reverse();
            return Some(cycle);
        }

        visited.insert(*start);

        for child in &neighbors(area, current) {
            if child != &parent {
                parent_map.insert(*child, current);
                stack.push_back((*child, current));
            }
        }
    }

    None
}

fn process_input(input: &'static str) -> (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>) {
    let mut start = None;
    let mut area: Vec<Vec<TileType>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, tile)| match tile {
                    '|' => TileType::TopBottom,
                    '-' => TileType::LeftRight,
                    'L' => TileType::TopRight,
                    'J' => TileType::TopLeft,
                    '7' => TileType::BottomLeft,
                    'F' => TileType::BottomRight,
                    '.' => TileType::Ground,
                    'S' => {
                        start = Some((x, y));
                        TileType::Start
                    }
                    unknown => panic!("Unknown tile: {unknown}"),
                })
                .collect()
        })
        .collect();

    let start = start.unwrap();

    let cycle = [
        TileType::TopBottom,
        TileType::LeftRight,
        TileType::TopRight,
        TileType::TopLeft,
        TileType::BottomLeft,
        TileType::BottomRight,
    ]
    .iter()
    .find_map(|tile_type| {
        area[start.1][start.0] = *tile_type;

        find_cycle(&area, &start)
    });

    (area, start, cycle.unwrap())
}

fn part1(area_start_cycle: &(Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>)) -> u64 {
    let (_area, _start, cycle) = area_start_cycle;
    cycle.len() as u64 / 2
}

fn part2(area_start_cycle: &(Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>)) -> u64 {
    let (area, _start, cycle) = area_start_cycle;
    let cycle: HashSet<_> = cycle.iter().collect();
    let clean_area: Vec<Vec<TileType>> = area
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, tile)| {
                    if cycle.contains(&(x, y)) {
                        *tile
                    } else {
                        TileType::Ground
                    }
                })
                .collect()
        })
        .collect();

    let enclosed_tiles: HashSet<(usize, usize)> = clean_area
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let mut inside = false;
            let mut last_changer = TileType::TopBottom;
            row.iter()
                .enumerate()
                .filter_map(|(x, tile)| {
                    let coord = (x, y);
                    if cycle.contains(&coord) {
                        match tile {
                            TileType::TopBottom | TileType::TopRight | TileType::BottomRight => {
                                inside = !inside;
                                last_changer = *tile;
                            }
                            TileType::LeftRight => {}
                            TileType::TopLeft => {
                                if last_changer != TileType::BottomRight {
                                    inside = !inside;
                                }
                                last_changer = *tile;
                            }
                            TileType::BottomLeft => {
                                if last_changer != TileType::TopRight {
                                    inside = !inside;
                                }
                                last_changer = *tile;
                            }
                            TileType::Ground | TileType::Start => panic!("Unexpected type!"),
                        };
                        None
                    } else if inside {
                        Some(coord)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect();

    // let colors = vec![
    //     (Color::Red, HashSet::from([start])),
    //     (Color::Blue, cycle),
    //     (Color::Yellow, enclosed_tiles.iter().collect()),
    // ];

    // pretty_print_area(&clean_area, &colors);
    enclosed_tiles.len() as u64
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data11() -> (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>) {
        let input = include_str!("test_input1.txt");
        process_input(input)
    }

    #[fixture]
    fn data12() -> (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>) {
        let input = include_str!("test_input2.txt");
        process_input(input)
    }

    #[fixture]
    fn data21() -> (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>) {
        let input = include_str!("test_input3.txt");
        process_input(input)
    }

    #[fixture]
    fn data22() -> (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>) {
        let input = include_str!("test_input4.txt");
        process_input(input)
    }

    #[fixture]
    fn data23() -> (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>) {
        let input = include_str!("test_input5.txt");
        process_input(input)
    }

    #[fixture]
    fn data24() -> (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>) {
        let input = include_str!("test_input6.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test1(data11: (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>)) {
        assert_eq!(part1(&data11), 4);
    }

    #[rstest]
    fn part1_test2(data12: (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>)) {
        assert_eq!(part1(&data12), 8);
    }

    #[rstest]
    fn part2_test1(data21: (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>)) {
        assert_eq!(part2(&data21), 4);
    }

    #[rstest]
    fn part2_test2(data22: (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>)) {
        assert_eq!(part2(&data22), 4);
    }

    #[rstest]
    fn part2_test3(data23: (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>)) {
        assert_eq!(part2(&data23), 8);
    }

    #[rstest]
    fn part2_test4(data24: (Vec<Vec<TileType>>, (usize, usize), Vec<(usize, usize)>)) {
        assert_eq!(part2(&data24), 10);
    }
}
