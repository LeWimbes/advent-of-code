use aoc_utils::Solution;
use aoc_utils::grid::{Direction, Grid, Pos};

struct Day15;

impl Solution for Day15 {
    type Input<'a> = (Grid<Tile>, Vec<Direction>, Pos);
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let (map, movements) = input.split_once("\n\n").unwrap();

        let map = Grid::parse(map, |c| c);
        let robot = map.find(&'@').unwrap();
        let map = map.map(|&tile| match tile {
            '#' => Tile::Wall,
            '.' | '@' => Tile::Floor,
            'O' => Tile::Box,
            _ => panic!("Unknown tile: {tile}"),
        });

        let movements = movements
            .lines()
            .flat_map(str::chars)
            .map(|movement| match movement {
                '^' => Direction::North,
                '>' => Direction::East,
                'v' => Direction::South,
                '<' => Direction::West,
                _ => panic!("Unknown movement: {movement}"),
            })
            .collect();

        (map, movements, robot)
    }

    fn part1((map, movements, robot): &Self::Input<'_>) -> Self::Output1 {
        let mut map = map.clone();
        let mut robot = *robot;

        calculate_future_gps(&mut map, movements, &mut robot)
    }

    fn part2((map, movements, robot): &Self::Input<'_>) -> Self::Output2 {
        let mut map = expand_map(map);
        let mut robot = Pos::new(robot.x * 2, robot.y);

        calculate_future_gps(&mut map, movements, &mut robot)
    }
}

aoc_utils::run!(2024, 15, Day15);

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Tile {
    Wall,
    Floor,
    Box,
    LeftBox,
    RightBox,
}

fn push_boxes_horizontally(map: &mut Grid<Tile>, start: Pos, dir: Direction) -> bool {
    let mut f_pos = start;

    loop {
        let n_pos = f_pos.step(dir).unwrap();

        match map[n_pos] {
            Tile::Wall => return false,
            Tile::Floor => {
                let mut b_pos = n_pos;

                while b_pos != start {
                    let new_b_pos = b_pos.step(dir.opposite()).unwrap();
                    map[b_pos] = map[new_b_pos];
                    b_pos = new_b_pos;
                }
                map[start] = Tile::Floor;
                return true;
            }
            Tile::Box | Tile::LeftBox | Tile::RightBox => {
                f_pos = n_pos;
            }
        }
    }
}

/// The moves of the box at `pos`, or of both halves of a wide box, one step in `dir`.
fn box_moves(
    map: &Grid<Tile>,
    pos: Pos,
    dir: Direction,
) -> impl Iterator<Item = (Tile, Pos, Pos)> + '_ {
    let (first, second) = match map[pos] {
        Tile::Wall | Tile::Floor => panic!("Shouldn't encounter wall or floor here."),
        Tile::Box => (pos, None),
        Tile::LeftBox => (pos, pos.step(Direction::East)),
        Tile::RightBox => (pos.step(Direction::West).unwrap(), Some(pos)),
    };

    std::iter::once(first)
        .chain(second)
        .map(move |from| (map[from], from, from.step(dir).unwrap()))
}

fn push_boxes_vertically(map: &mut Grid<Tile>, start: Pos, dir: Direction) -> bool {
    let mut changes = vec![box_moves(map, start, dir).collect::<Vec<_>>()];

    loop {
        let mut new_changes = Vec::new();
        for &(_tile, _from, to) in &changes[changes.len() - 1] {
            match map[to] {
                Tile::Wall => return false,
                Tile::Floor => {}
                Tile::Box | Tile::LeftBox | Tile::RightBox => {
                    new_changes.extend(box_moves(map, to, dir));
                }
            }
        }

        if new_changes.is_empty() {
            for row_change in changes.iter().rev() {
                for &(tile, from, to) in row_change {
                    map[from] = Tile::Floor;
                    map[to] = tile;
                }
            }
            return true;
        }

        changes.push(new_changes);
    }
}

fn move_robot(map: &mut Grid<Tile>, robot: &mut Pos, dir: Direction) {
    let next_pos = robot.step(dir).unwrap();

    match map[next_pos] {
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

fn get_gps(map: &Grid<Tile>) -> usize {
    map.iter()
        .filter(|&(_pos, &tile)| matches!(tile, Tile::Box | Tile::LeftBox))
        .map(|(pos, _)| 100 * pos.y + pos.x)
        .sum()
}

fn calculate_future_gps(map: &mut Grid<Tile>, movements: &[Direction], robot: &mut Pos) -> usize {
    for &dir in movements {
        move_robot(map, robot, dir);
    }

    get_gps(map)
}

fn expand_map(map: &Grid<Tile>) -> Grid<Tile> {
    let mut wide_map = Grid::new(map.width() * 2, map.height(), Tile::Floor);

    for (pos, &tile) in map.iter() {
        let (left, right) = match tile {
            Tile::Wall => (Tile::Wall, Tile::Wall),
            Tile::Floor => (Tile::Floor, Tile::Floor),
            Tile::Box => (Tile::LeftBox, Tile::RightBox),
            Tile::LeftBox | Tile::RightBox => panic!("Wide boxes are not supported."),
        };
        wide_map[Pos::new(pos.x * 2, pos.y)] = left;
        wide_map[Pos::new(pos.x * 2 + 1, pos.y)] = right;
    }

    wide_map
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 15, Day15, examples = [1, 2]);

    #[rstest]
    fn part1_test1(data1: <Day15 as Solution>::Input<'_>) {
        assert_eq!(10_092, Day15::part1(&data1));
    }

    #[rstest]
    fn part1_test2(data2: <Day15 as Solution>::Input<'_>) {
        assert_eq!(2028, Day15::part1(&data2));
    }

    #[rstest]
    fn part2_test1(data1: <Day15 as Solution>::Input<'_>) {
        assert_eq!(9021, Day15::part2(&data1));
    }
}
