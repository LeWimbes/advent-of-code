use std::ops::{BitAnd, BitOrAssign};

use itertools::repeat_n;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 16);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North = 0b0001,
    West = 0b0010,
    South = 0b0100,
    East = 0b1000,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Ray {
    coord: (usize, usize),
    direction: Direction,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TileType {
    None,
    Slash,
    Backslash,
    Vertical,
    Horizontal,
}

impl Direction {
    fn get_next_coord(
        self,
        coord: &(usize, usize),
        bounds: &(usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Direction::North => {
                if coord.1 != 0 {
                    return Some((coord.0, coord.1 - 1));
                }
            }
            Direction::West => {
                if coord.0 != 0 {
                    return Some((coord.0 - 1, coord.1));
                }
            }
            Direction::South => {
                if coord.1 < bounds.1 - 1 {
                    return Some((coord.0, coord.1 + 1));
                }
            }
            Direction::East => {
                if coord.0 < bounds.0 - 1 {
                    return Some((coord.0 + 1, coord.1));
                }
            }
        };
        None
    }

    fn get_next_ray(self, coord: &(usize, usize), bounds: &(usize, usize)) -> Option<Ray> {
        self.get_next_coord(coord, bounds)
            .map(|new_coord| Ray::from(new_coord, self))
    }

    fn get_next_rays(self, coord: &(usize, usize), bounds: &(usize, usize)) -> Vec<Ray> {
        self.get_next_ray(coord, bounds)
            .map(|next_ray| vec![next_ray])
            .unwrap_or_default()
    }
}

impl Ray {
    fn from(coord: (usize, usize), direction: Direction) -> Self {
        Ray { coord, direction }
    }

    fn get_next_ray(self, bounds: &(usize, usize)) -> Option<Ray> {
        self.direction
            .get_next_coord(&self.coord, bounds)
            .map(|new_coord| Ray::from(new_coord, self.direction))
    }

    fn get_next_rays(self, bounds: &(usize, usize)) -> Vec<Ray> {
        self.get_next_ray(bounds)
            .map(|new_ray| vec![new_ray])
            .unwrap_or_default()
    }
}

fn get_next_rays(
    directions: &[Direction],
    coord: &(usize, usize),
    bounds: &(usize, usize),
) -> Vec<Ray> {
    directions
        .iter()
        .filter_map(|direction| {
            direction
                .get_next_coord(coord, bounds)
                .map(|new_coord| Ray::from(new_coord, *direction))
        })
        .collect()
}

impl TileType {
    fn get_outgoing_rays(self, incoming: &Ray, bounds: &(usize, usize)) -> Vec<Ray> {
        match self {
            TileType::None => incoming.get_next_rays(bounds),

            TileType::Slash => match incoming.direction {
                Direction::North => Direction::East.get_next_rays(&incoming.coord, bounds),
                Direction::West => Direction::South.get_next_rays(&incoming.coord, bounds),
                Direction::South => Direction::West.get_next_rays(&incoming.coord, bounds),
                Direction::East => Direction::North.get_next_rays(&incoming.coord, bounds),
            },

            TileType::Backslash => match incoming.direction {
                Direction::North => Direction::West.get_next_rays(&incoming.coord, bounds),
                Direction::West => Direction::North.get_next_rays(&incoming.coord, bounds),
                Direction::South => Direction::East.get_next_rays(&incoming.coord, bounds),
                Direction::East => Direction::South.get_next_rays(&incoming.coord, bounds),
            },

            TileType::Vertical => match incoming.direction {
                Direction::North | Direction::South => incoming.get_next_rays(bounds),
                Direction::West | Direction::East => get_next_rays(
                    &[Direction::North, Direction::South],
                    &incoming.coord,
                    bounds,
                ),
            },

            TileType::Horizontal => match incoming.direction {
                Direction::North | Direction::South => {
                    get_next_rays(&[Direction::West, Direction::East], &incoming.coord, bounds)
                }
                Direction::West | Direction::East => incoming.get_next_rays(bounds),
            },
        }
    }
}

fn process_input(input: &'static str) -> Vec<Vec<TileType>> {
    input
        .lines()
        .map(|row| {
            row.chars()
                .map(|char| match char {
                    '/' => TileType::Slash,
                    '\\' => TileType::Backslash,
                    '|' => TileType::Vertical,
                    '-' => TileType::Horizontal,
                    _ => TileType::None,
                })
                .collect()
        })
        .collect()
}

fn get_energized_tiles(layout: &[Vec<TileType>], start: Ray) -> u64 {
    let bounds = (layout[0].len(), layout.len());

    let mut rays_on_tiles = vec![vec![0b0000isize; bounds.0]; bounds.1];

    let mut rays = vec![start];
    rays_on_tiles[start.coord.1][start.coord.0].bitor_assign(start.direction as isize);

    while !rays.is_empty() {
        for ray in &rays {
            rays_on_tiles[ray.coord.1][ray.coord.0].bitor_assign(ray.direction as isize);
        }

        rays = rays
            .iter()
            .flat_map(|ray| layout[ray.coord.1][ray.coord.0].get_outgoing_rays(ray, &bounds))
            .filter(|ray| {
                let direction_val = ray.direction as isize;
                if rays_on_tiles[ray.coord.1][ray.coord.0].bitand(direction_val) != 0 {
                    false
                } else {
                    rays_on_tiles[ray.coord.1][ray.coord.0].bitor_assign(direction_val);
                    true
                }
            })
            .collect();
    }

    rays_on_tiles
        .iter()
        .map(|row| row.iter().filter(|tile| **tile != 0).count() as u64)
        .sum()
}

fn part1(layout: &[Vec<TileType>]) -> u64 {
    get_energized_tiles(layout, Ray::from((0, 0), Direction::East))
}

fn part2(layout: &[Vec<TileType>]) -> u64 {
    // top
    (0..layout[0].len())
        .zip(repeat_n(0, layout.len()))
        .zip(repeat_n(Direction::South, layout[0].len()))
        // bottom
        .chain(
            (0..layout[0].len())
                .zip(repeat_n(0, layout.len()))
                .zip(repeat_n(Direction::North, layout[0].len())),
        )
        // left
        .chain(
            repeat_n(0, layout[0].len())
                .zip(0..layout.len())
                .zip(repeat_n(Direction::East, layout[0].len())),
        )
        // right
        .chain(
            repeat_n(0, layout[0].len())
                .zip(0..layout.len())
                .zip(repeat_n(Direction::West, layout[0].len())),
        )
        .map(|(coord, direction)| get_energized_tiles(layout, Ray::from(coord, direction)))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> Vec<Vec<TileType>> {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<Vec<TileType>>) {
        assert_eq!(part1(&data), 46);
    }

    #[rstest]
    fn part2_test(data: Vec<Vec<TileType>>) {
        assert_eq!(part2(&data), 51);
    }
}
