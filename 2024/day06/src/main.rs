use aoc_utils::Solution;
use aoc_utils::grid::{Direction, Grid, Pos};

struct Day06;

impl Solution for Day06 {
    type Input<'a> = (Grid<bool>, Agent);
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let map = Grid::parse(input, |c| c);

        let agent = map
            .iter()
            .find_map(|(pos, &c)| {
                let direction = match c {
                    '^' => Direction::North,
                    '>' => Direction::East,
                    'v' => Direction::South,
                    '<' => Direction::West,
                    _ => return None,
                };
                Some(Agent { pos, direction })
            })
            .expect("No guard found");

        (map.map(|&c| c == '#'), agent)
    }

    fn part1((obstructions, agent): &Self::Input<'_>) -> Self::Output1 {
        walk(obstructions, *agent)
            .expect("Shouldn't encounter loop")
            .iter()
            .filter(|&(_pos, &directions)| directions != 0)
            .count()
    }

    fn part2((obstructions, agent): &Self::Input<'_>) -> Self::Output2 {
        let mut with_new = obstructions.clone();

        obstructions
            .positions()
            .filter(|&pos| pos != agent.pos && !obstructions[pos])
            .filter(|&pos| {
                with_new[pos] = true;
                let is_loop = walk(&with_new, *agent).is_none();
                with_new[pos] = false;
                is_loop
            })
            .count()
    }
}

aoc_utils::run!(2024, 6, Day06);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Agent {
    pos: Pos,
    direction: Direction,
}

/// Walk until the guard leaves the map and
/// return the directions it faced on each position as bit flags,
/// or `None` if it ends up in a loop.
fn walk(obstructions: &Grid<bool>, mut agent: Agent) -> Option<Grid<u8>> {
    let mut visited = Grid::new(obstructions.width(), obstructions.height(), 0u8);

    loop {
        let flag = 1 << agent.direction as u8;
        if visited[agent.pos] & flag != 0 {
            return None;
        }
        visited[agent.pos] |= flag;

        match obstructions.step(agent.pos, agent.direction) {
            None => return Some(visited),
            Some(next) if obstructions[next] => agent.direction = agent.direction.turn_right(),
            Some(next) => agent.pos = next,
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 6, Day06, examples = [1]);

    #[rstest]
    fn part1_test(data1: <Day06 as Solution>::Input<'_>) {
        assert_eq!(41, Day06::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day06 as Solution>::Input<'_>) {
        assert_eq!(6, Day06::part2(&data1));
    }
}
