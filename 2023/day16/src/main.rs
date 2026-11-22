use aoc_utils::Solution;
use aoc_utils::grid::{Direction, Grid, Pos};

struct Day16;

impl Solution for Day16 {
    type Input<'a> = Grid<TileType>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        Grid::parse(input, |char| match char {
            '/' => TileType::Slash,
            '\\' => TileType::Backslash,
            '|' => TileType::Vertical,
            '-' => TileType::Horizontal,
            _ => TileType::None,
        })
    }

    fn part1(layout: &Self::Input<'_>) -> Self::Output1 {
        get_energized_tiles(layout, Pos::new(0, 0), Direction::East)
    }

    fn part2(layout: &Self::Input<'_>) -> Self::Output2 {
        let top_bottom = (0..layout.width()).flat_map(|x| {
            [
                (Pos::new(x, 0), Direction::South),
                (Pos::new(x, layout.height() - 1), Direction::North),
            ]
        });
        let left_right = (0..layout.height()).flat_map(|y| {
            [
                (Pos::new(0, y), Direction::East),
                (Pos::new(layout.width() - 1, y), Direction::West),
            ]
        });

        top_bottom
            .chain(left_right)
            .map(|(pos, direction)| get_energized_tiles(layout, pos, direction))
            .max()
            .unwrap()
    }
}

aoc_utils::run!(2023, 16, Day16);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum TileType {
    None,
    Slash,
    Backslash,
    Vertical,
    Horizontal,
}

impl TileType {
    /// The directions in which a beam leaves this tile when it enters moving in `direction`.
    fn outgoing(self, direction: Direction) -> [Option<Direction>; 2] {
        match (self, direction) {
            (TileType::Slash, Direction::North | Direction::South)
            | (TileType::Backslash, Direction::East | Direction::West) => {
                [Some(direction.turn_right()), None]
            }
            (TileType::Slash, Direction::East | Direction::West)
            | (TileType::Backslash, Direction::North | Direction::South) => {
                [Some(direction.turn_left()), None]
            }
            (TileType::Vertical, Direction::East | Direction::West) => {
                [Some(Direction::North), Some(Direction::South)]
            }
            (TileType::Horizontal, Direction::North | Direction::South) => {
                [Some(Direction::West), Some(Direction::East)]
            }
            _ => [Some(direction), None],
        }
    }
}

fn get_energized_tiles(
    layout: &<Day16 as Solution>::Input<'_>,
    start: Pos,
    direction: Direction,
) -> usize {
    // The directions of the beams on each tile as bit flags
    let mut beams_on_tiles = Grid::new(layout.width(), layout.height(), 0u8);
    let flag = |direction: Direction| 1 << direction as u8;

    let mut beams = vec![(start, direction)];
    beams_on_tiles[start] |= flag(direction);

    while let Some((pos, direction)) = beams.pop() {
        for next_direction in layout[pos].outgoing(direction).into_iter().flatten() {
            if let Some(next_pos) = layout.step(pos, next_direction)
                && beams_on_tiles[next_pos] & flag(next_direction) == 0
            {
                beams_on_tiles[next_pos] |= flag(next_direction);
                beams.push((next_pos, next_direction));
            }
        }
    }

    beams_on_tiles
        .iter()
        .filter(|&(_pos, &beams)| beams != 0)
        .count()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 16, Day16, examples = [1]);

    #[rstest]
    fn part1_test(data1: <Day16 as Solution>::Input<'_>) {
        assert_eq!(46, Day16::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day16 as Solution>::Input<'_>) {
        assert_eq!(51, Day16::part2(&data1));
    }
}
