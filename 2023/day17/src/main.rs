use std::cmp::Reverse;
use std::collections::BinaryHeap;

use aoc_utils::Solution;
use aoc_utils::grid::{Direction, Grid, Pos};

struct Day17;

impl Solution for Day17 {
    type Input<'a> = Grid<u32>;
    type Output1 = u32;
    type Output2 = u32;

    fn process(input: &str) -> Self::Input<'_> {
        Grid::parse(input, |char| char.to_digit(10).unwrap())
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
struct State {
    pos: Pos,
    direction: Direction,
    moves_in_direction: usize,
}

fn shortest_path(
    layout: &<Day17 as Solution>::Input<'_>,
    min_moves: usize,
    max_moves: usize,
) -> u32 {
    let end = Pos::new(layout.width() - 1, layout.height() - 1);
    let directions = Direction::ALL4.len();

    let index = |state: State| {
        (layout.flat_index(state.pos) * directions + state.direction as usize) * max_moves
            + state.moves_in_direction
    };
    let state = |index: usize| {
        let cell = index / (directions * max_moves);
        State {
            pos: Pos::new(cell % layout.width(), cell / layout.width()),
            direction: Direction::ALL4[index / max_moves % directions],
            moves_in_direction: index % max_moves,
        }
    };
    let mut dists = vec![u32::MAX; layout.width() * layout.height() * directions * max_moves];

    let mut heap = BinaryHeap::new();
    for direction in Direction::ALL4 {
        let start = index(State {
            pos: Pos::new(0, 0),
            direction,
            moves_in_direction: 0,
        });
        dists[start] = 0;
        heap.push(Reverse((0, start)));
    }

    while let Some(Reverse((cost, current_index))) = heap.pop() {
        if cost > dists[current_index] {
            continue;
        }
        let current = state(current_index);

        for direction in Direction::ALL4 {
            let straight = direction == current.direction;
            if direction == current.direction.opposite()
                || (current.moves_in_direction < min_moves && !straight)
                || (current.moves_in_direction + 1 >= max_moves && straight)
            {
                continue;
            }

            let Some(pos) = layout.step(current.pos, direction) else {
                continue;
            };
            let next = index(State {
                pos,
                direction,
                moves_in_direction: if straight {
                    current.moves_in_direction + 1
                } else {
                    1
                },
            });
            let next_cost = cost + layout[pos];

            if next_cost < dists[next] {
                dists[next] = next_cost;
                heap.push(Reverse((next_cost, next)));
            }
        }
    }

    (min_moves..max_moves)
        .flat_map(|moves_in_direction| {
            Direction::ALL4.map(|direction| {
                dists[index(State {
                    pos: end,
                    direction,
                    moves_in_direction,
                })]
            })
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 17, Day17, examples = [1, 4]);

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
