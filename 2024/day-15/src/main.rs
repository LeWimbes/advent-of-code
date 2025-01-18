const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 15);

type ParsedInput = (Vec<Vec<char>>, Vec<char>);
type ProcessedInput = (Vec<Vec<Tile>>, Vec<Direction>, (usize, usize));

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    let parts = input.split_once("\n\n").unwrap();
    (
        parts
            .0
            .lines()
            .filter(|s| !s.is_empty())
            .map(|line| line.chars().collect())
            .collect(),
        parts
            .1
            .lines()
            .filter(|s| !s.is_empty())
            .flat_map(|line| line.chars().collect::<Vec<_>>())
            .collect(),
    )
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Wall,
    Floor,
    Box,
    LeftBox,
    RightBox,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn offset(self) -> (isize, isize) {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
}

fn process_input(input: &'static str) -> ProcessedInput {
    let (map, movements) = parse_input(input);

    let mut robot = (0, 0);

    (
        map.into_iter()
            .enumerate()
            .map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(x, tile)| match tile {
                        '#' => Tile::Wall,
                        '.' => Tile::Floor,
                        'O' => Tile::Box,
                        '@' => {
                            robot = (x, y);
                            Tile::Floor
                        }
                        _ => panic!("Unknown tile: {tile}"),
                    })
                    .collect()
            })
            .collect(),
        movements
            .into_iter()
            .map(|movement| match movement {
                '^' => Direction::North,
                '>' => Direction::East,
                'v' => Direction::South,
                '<' => Direction::West,
                _ => panic!("Unknown movement: {movement}"),
            })
            .collect(),
        robot,
    )
}

fn push_boxes_horizontally(map: &mut [Vec<Tile>], start: (usize, usize), dir: Direction) -> bool {
    let (dx, dy) = dir.offset();

    let sx = start.0 as isize;
    let sy = start.1 as isize;
    let mut far_x = sx;
    let mut far_y = sy;

    loop {
        let nx = far_x + dx;
        let ny = far_y + dy;

        match map[ny as usize][nx as usize] {
            Tile::Wall => return false,
            Tile::Floor => {
                let mut bx = nx;
                let mut by = ny;

                while bx != sx || by != sy {
                    map[by as usize][bx as usize] = map[(by - dy) as usize][(bx - dx) as usize];
                    bx -= dx;
                    by -= dy;
                }
                map[sy as usize][sx as usize] = Tile::Floor;
                return true;
            }
            Tile::Box | Tile::LeftBox | Tile::RightBox => {
                far_x = nx;
                far_y = ny;
            }
        }
    }
}

fn push_boxes_vertically(map: &mut [Vec<Tile>], start: (usize, usize), dir: Direction) -> bool {
    let (dx, dy) = dir.offset();

    let sx = start.0 as isize;
    let sy = start.1 as isize;

    let mut changes = Vec::new();
    match map[start.1][start.0] {
        Tile::Wall | Tile::Floor => panic!("Shouldn't encounter wall or floor here."),
        Tile::Box => changes.push(vec![(Tile::Box, (sx, sy), (sx + dx, sy + dy))]),
        Tile::LeftBox => changes.push(vec![
            (Tile::LeftBox, (sx, sy), (sx + dx, sy + dy)),
            (Tile::RightBox, (sx + 1, sy), (sx + 1 + dx, sy + dy)),
        ]),
        Tile::RightBox => changes.push(vec![
            (Tile::LeftBox, (sx - 1, sy), (sx - 1 + dx, sy + dy)),
            (Tile::RightBox, (sx, sy), (sx + dx, sy + dy)),
        ]),
    }

    loop {
        let last_changes = &changes[changes.len() - 1];
        let mut new_changes = Vec::new();
        for &(_t, _from, (nx, ny)) in last_changes {
            match map[ny as usize][nx as usize] {
                Tile::Wall => return false,
                Tile::Floor => {}
                Tile::Box => {
                    new_changes.extend_from_slice(&[(Tile::Box, (nx, ny), (nx + dx, ny + dy))])
                }
                Tile::LeftBox => new_changes.extend_from_slice(&[
                    (Tile::LeftBox, (nx, ny), (nx + dx, ny + dy)),
                    (Tile::RightBox, (nx + 1, ny), (nx + 1 + dx, ny + dy)),
                ]),
                Tile::RightBox => new_changes.extend_from_slice(&[
                    (Tile::LeftBox, (nx - 1, ny), (nx - 1 + dx, ny + dy)),
                    (Tile::RightBox, (nx, ny), (nx + dx, ny + dy)),
                ]),
            }
        }

        if new_changes.is_empty() {
            for row_change in changes.iter().rev() {
                for &(t, (x, y), (nx, ny)) in row_change {
                    map[y as usize][x as usize] = Tile::Floor;
                    map[ny as usize][nx as usize] = t;
                }
            }
            return true;
        }

        changes.push(new_changes);
    }
}

fn move_robot(map: &mut [Vec<Tile>], robot: &mut (usize, usize), dir: Direction) {
    let (dx, dy) = dir.offset();
    let next_pos = (
        (robot.0 as isize + dx) as usize,
        (robot.1 as isize + dy) as usize,
    );
    match map[next_pos.1][next_pos.0] {
        Tile::Wall => {}
        Tile::Floor => *robot = next_pos,
        Tile::Box | Tile::LeftBox | Tile::RightBox => match dir {
            Direction::East | Direction::West => {
                if push_boxes_horizontally(map, next_pos, dir) {
                    *robot = next_pos;
                }
            }
            Direction::North | Direction::South => {
                if push_boxes_vertically(map, next_pos, dir) {
                    *robot = next_pos;
                }
            }
        },
    }
}

fn get_gps(map: &[Vec<Tile>]) -> usize {
    map.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, tile)| match tile {
                    Tile::Wall | Tile::Floor | Tile::RightBox => None,
                    Tile::Box | Tile::LeftBox => Some(100 * y + x),
                })
        })
        .sum()
}

fn calculate_future_gps(
    map: &mut [Vec<Tile>],
    movements: &Vec<Direction>,
    robot: &mut (usize, usize),
) -> usize {
    for &dir in movements {
        move_robot(map, robot, dir);
    }

    get_gps(map)
}

fn part1((map, movements, robot): &ProcessedInput) -> usize {
    let mut map = map.clone();
    let mut robot = *robot;

    calculate_future_gps(&mut map, movements, &mut robot)
}

fn expand_map(map: &mut [Vec<Tile>]) {
    let old_width = map[0].len();
    let new_width = old_width * 2;

    for row in map.iter_mut() {
        row.resize(new_width, Tile::Wall);

        for x in (0..old_width).rev() {
            let new_x = x * 2;
            row[new_x] = row[x];
            match row[x] {
                Tile::Wall => row[new_x + 1] = Tile::Wall,
                Tile::Floor => row[new_x + 1] = Tile::Floor,
                Tile::Box => {
                    row[new_x] = Tile::LeftBox;
                    row[new_x + 1] = Tile::RightBox;
                }
                Tile::LeftBox | Tile::RightBox => panic!("Wide boxes are not supported."),
            }
        }
    }
}

fn part2((map, movements, robot): &ProcessedInput) -> usize {
    let mut map = map.clone();
    expand_map(&mut map);
    let mut robot = (robot.0 * 2, robot.1);

    calculate_future_gps(&mut map, movements, &mut robot)
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

    #[rstest]
    fn part1_test1(data1: ProcessedInput) {
        assert_eq!(part1(&data1), 10092);
    }

    #[rstest]
    fn part1_test2(data2: ProcessedInput) {
        assert_eq!(part1(&data2), 2028);
    }

    #[rstest]
    fn part2_test1(data1: ProcessedInput) {
        assert_eq!(part2(&data1), 9021);
    }
}
