use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::sync::LazyLock;
use std::vec;

use aoc_utils::Solution;

struct Day17;

impl Solution for Day17 {
    type Input<'a> = Vec<Vec<u32>>;
    type Output1 = u32;
    type Output2 = u32;

    fn process(input: &str) -> Self::Input<'_> {
        input
            .lines()
            .map(|row| row.chars().map(|char| char.to_digit(10).unwrap()).collect())
            .collect()
    }

    fn part1(layout: &Self::Input<'_>) -> Self::Output1 {
        shortest_path(layout, 0, 4)
    }

    fn part2(layout: &Self::Input<'_>) -> Self::Output2 {
        shortest_path(layout, 4, 11)
    }
}

aoc_utils::run!(2023, 17, Day17);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Direction {
    North = 0,
    West = 1,
    South = 2,
    East = 3,
}

static DIRECTIONS: LazyLock<Vec<Direction>> = LazyLock::new(|| {
    vec![
        Direction::North,
        Direction::West,
        Direction::South,
        Direction::East,
    ]
});

impl Direction {
    fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::West => Direction::East,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
        }
    }
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
                if coord.1 + 1 < bounds.1 {
                    return Some((coord.0, coord.1 + 1));
                }
            }
            Direction::East => {
                if coord.0 + 1 < bounds.0 {
                    return Some((coord.0 + 1, coord.1));
                }
            }
        };

        None
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    cost: u32,
    coord: (usize, usize),
    direction: Direction,
    moves_in_direction: usize,
}

impl State {
    fn from(
        cost: u32,
        coord: (usize, usize),
        direction: Direction,
        moves_in_direction: usize,
    ) -> Self {
        State {
            cost,
            coord,
            direction,
            moves_in_direction,
        }
    }

    fn get_index(self, bounds: &(usize, usize), max_moves: usize) -> usize {
        self.moves_in_direction
            + self.direction as usize * max_moves
            + self.coord.0 * max_moves * DIRECTIONS.len()
            + self.coord.1 * max_moves * DIRECTIONS.len() * bounds.0
    }

    fn next_states(
        self,
        layout: &<Day17 as Solution>::Input<'_>,
        min_moves: usize,
        max_moves: usize,
    ) -> Vec<State> {
        let bounds = (layout[0].len(), layout.len());
        let opposite = self.direction.opposite();

        DIRECTIONS
            .iter()
            .filter(|dir| {
                **dir != opposite
                    && (self.moves_in_direction >= min_moves || **dir == self.direction)
                    && (self.moves_in_direction + 1 < max_moves || **dir != self.direction)
            })
            .filter_map(|dir| {
                dir.get_next_coord(&self.coord, &bounds).map(|coord| {
                    let cost = layout[coord.1][coord.0];
                    let dir = *dir;

                    if self.direction == dir {
                        State::from(self.cost + cost, coord, dir, self.moves_in_direction + 1)
                    } else {
                        State::from(self.cost + cost, coord, dir, 1)
                    }
                })
            })
            .collect()
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(
    layout: &<Day17 as Solution>::Input<'_>,
    min_moves: usize,
    max_moves: usize,
) -> u32 {
    let bounds = (layout[0].len(), layout.len());

    let end = (bounds.0 - 1, bounds.1 - 1);

    let mut dists = vec![u32::MAX; max_moves * DIRECTIONS.len() * bounds.0 * bounds.1];

    let mut heap = BinaryHeap::new();
    DIRECTIONS
        .iter()
        .map(|dir| State::from(0, (0, 0), *dir, 0))
        .for_each(|start| {
            dists[start.get_index(&bounds, max_moves)] = 0;
            heap.push(start);
        });

    while let Some(current) = heap.pop() {
        if current.cost > dists[current.get_index(&bounds, max_moves)] {
            continue;
        }

        for next in current.next_states(layout, min_moves, max_moves) {
            if next.cost < dists[next.get_index(&bounds, max_moves)] {
                dists[next.get_index(&bounds, max_moves)] = next.cost;
                heap.push(next);
            }
        }
    }

    (min_moves..max_moves)
        .map(|moves| {
            DIRECTIONS
                .iter()
                .map(|dir| {
                    let state = State::from(0, end, *dir, moves);
                    dists[state.get_index(&bounds, max_moves)]
                })
                .min()
                .unwrap()
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 17, Day17);

    #[rstest]
    fn part1_test(data1: <Day17 as Solution>::Input<'_>) {
        assert_eq!(102, Day17::part1(&data1));
    }

    #[rstest]
    fn part2_test1(data1: <Day17 as Solution>::Input<'_>) {
        assert_eq!(94, Day17::part2(&data1));
    }

    #[rstest]
    fn part2_test2(data2: <Day17 as Solution>::Input<'_>) {
        assert_eq!(71, Day17::part2(&data2));
    }
}
