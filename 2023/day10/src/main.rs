use std::collections::{HashMap, HashSet, LinkedList};

use aoc_utils::Solution;
use aoc_utils::grid::{Direction, Grid, Pos};

use colored::{Color, Colorize};
use itertools::Itertools;

struct Day10;

impl Solution for Day10 {
    type Input<'a> = (Grid<TileType>, Pos, Vec<Pos>);
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let mut area = Grid::parse(input, |tile| match tile {
            '|' => TileType::TopBottom,
            '-' => TileType::LeftRight,
            'L' => TileType::TopRight,
            'J' => TileType::TopLeft,
            '7' => TileType::BottomLeft,
            'F' => TileType::BottomRight,
            // The examples mark tiles outside and inside the loop with O and I
            '.' | 'O' | 'I' => TileType::Ground,
            'S' => TileType::Start,
            unknown => panic!("Unknown tile: {unknown}"),
        });
        let start = area.find(&TileType::Start).unwrap();

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
            area[start] = *tile_type;

            find_cycle(&area, &start)
        });

        (area, start, cycle.unwrap())
    }

    fn part1((_area, _start, cycle): &Self::Input<'_>) -> Self::Output1 {
        cycle.len() / 2
    }

    fn part2((area, start, cycle): &Self::Input<'_>) -> Self::Output2 {
        // Only keep the tiles of the loop
        let mut clean_area = Grid::new(area.width(), area.height(), TileType::Ground);
        for &pos in cycle {
            clean_area[pos] = area[pos];
        }
        let cycle: HashSet<_> = cycle.iter().collect();

        let enclosed_tiles: HashSet<Pos> = clean_area
            .rows()
            .enumerate()
            .flat_map(|(y, row)| {
                let mut inside = false;
                let mut last_changer = &TileType::TopBottom;
                row.iter()
                    .enumerate()
                    .filter_map(|(x, tile)| {
                        if *tile != TileType::Ground {
                            match tile {
                                TileType::TopBottom
                                | TileType::TopRight
                                | TileType::BottomRight => {
                                    inside = !inside;
                                    last_changer = tile;
                                }
                                TileType::LeftRight => {}
                                TileType::TopLeft => {
                                    if last_changer != &TileType::BottomRight {
                                        inside = !inside;
                                    }
                                    last_changer = tile;
                                }
                                TileType::BottomLeft => {
                                    if last_changer != &TileType::TopRight {
                                        inside = !inside;
                                    }
                                    last_changer = tile;
                                }
                                TileType::Ground | TileType::Start => panic!("Unexpected type!"),
                            }
                            None
                        } else if inside {
                            Some(Pos::new(x, y))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let colors = vec![
            (Color::Red, HashSet::from([start])),
            (Color::Blue, cycle),
            (Color::Yellow, enclosed_tiles.iter().collect()),
        ];

        pretty_print_area(&clean_area, &colors);
        enclosed_tiles.len()
    }
}

aoc_utils::run!(2023, 10, Day10);

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

impl TileType {
    /// The directions in which this tile connects to its neighbours.
    fn connections(self) -> &'static [Direction] {
        match self {
            TileType::TopBottom => &[Direction::North, Direction::South],
            TileType::LeftRight => &[Direction::West, Direction::East],
            TileType::TopRight => &[Direction::North, Direction::East],
            TileType::TopLeft => &[Direction::North, Direction::West],
            TileType::BottomLeft => &[Direction::South, Direction::West],
            TileType::BottomRight => &[Direction::South, Direction::East],
            TileType::Ground => &[],
            TileType::Start => &Direction::ALL4,
        }
    }
}

/// The neighbours that `coord` connects to and that connect back.
fn neighbors(area: &Grid<TileType>, coord: Pos) -> Vec<Pos> {
    area[coord]
        .connections()
        .iter()
        .filter_map(|&dir| {
            let next = area.step(coord, dir)?;
            area[next]
                .connections()
                .contains(&dir.opposite())
                .then_some(next)
        })
        .collect()
}

fn pretty_print_area(area: &Grid<TileType>, colors: &Vec<(Color, HashSet<&Pos>)>) {
    let top_bottom = (0..area.width()).map(|_| '─').join("").bold().green();
    println!("{}{}{}", "┌".bold().green(), top_bottom, "┐".bold().green());
    area.rows().enumerate().for_each(|(y, row)| {
        let row_str = row
            .iter()
            .enumerate()
            .map(|(x, tile)| {
                let tile_str = match tile {
                    TileType::TopBottom => "│",
                    TileType::LeftRight => "─",
                    TileType::TopRight => "└",
                    TileType::TopLeft => "┘",
                    TileType::BottomLeft => "┐",
                    TileType::BottomRight => "┌",
                    TileType::Ground => " ",
                    TileType::Start => "S",
                };
                if let Some(color) = colors.iter().find_map(|(color, tiles)| {
                    if tiles.contains(&Pos::new(x, y)) {
                        Some(color)
                    } else {
                        None
                    }
                }) {
                    if tile_str.trim().is_empty() {
                        tile_str.on_color(*color)
                    } else {
                        tile_str.color(*color)
                    }
                } else {
                    tile_str.normal()
                }
            })
            .join("");

        println!("{}{}{}", "│".bold().green(), row_str, "│".bold().green());
    });
    println!("{}{}{}", "└".bold().green(), top_bottom, "┘".bold().green());
}

fn find_cycle(area: &Grid<TileType>, start: &Pos) -> Option<Vec<Pos>> {
    let mut parent_map: HashMap<Pos, Pos> = HashMap::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut stack: LinkedList<(Pos, Pos)> = LinkedList::new();

    stack.push_back((*start, Pos::new(usize::MAX, usize::MAX)));

    while let Some((current, parent)) = stack.pop_back() {
        if visited.contains(&current) {
            let mut last = parent_map[&current];
            let mut cycle: Vec<Pos> = vec![last];
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 10, Day10, examples = [2, 4, 10, 12, 13, 15]);

    #[rstest]
    fn part1_test1(data1: <Day10 as Solution>::Input<'_>) {
        assert_eq!(4, Day10::part1(&data1));
    }

    #[rstest]
    fn part1_test2(data2: <Day10 as Solution>::Input<'_>) {
        assert_eq!(8, Day10::part1(&data2));
    }

    #[rstest]
    fn part2_test1(data3: <Day10 as Solution>::Input<'_>) {
        assert_eq!(4, Day10::part2(&data3));
    }

    #[rstest]
    fn part2_test2(data4: <Day10 as Solution>::Input<'_>) {
        assert_eq!(4, Day10::part2(&data4));
    }

    #[rstest]
    fn part2_test3(data5: <Day10 as Solution>::Input<'_>) {
        assert_eq!(8, Day10::part2(&data5));
    }

    #[rstest]
    fn part2_test4(data6: <Day10 as Solution>::Input<'_>) {
        assert_eq!(10, Day10::part2(&data6));
    }
}
