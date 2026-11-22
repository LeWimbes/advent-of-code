use aoc_utils::Solution;
use aoc_utils::grid::{Direction, Grid, Pos};
use itertools::Itertools;
use std::collections::{HashSet, VecDeque};

struct Day12;

impl Solution for Day12 {
    /// Each region's cells, and its perimeter as the cells on the edge with the direction they face
    type Input<'a> = Vec<(Vec<Pos>, Vec<(Pos, Direction)>)>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        get_areas_perimeters(&Grid::parse(input, |c| c))
    }

    fn part1(areas_perimeters: &Self::Input<'_>) -> Self::Output1 {
        areas_perimeters
            .iter()
            .map(|(area, perimeter)| area.len() * perimeter.len())
            .sum()
    }

    fn part2(areas_perimeters: &Self::Input<'_>) -> Self::Output2 {
        areas_perimeters
            .iter()
            .map(|(area, perimeter)| {
                let mut sides: Vec<(Pos, Direction)> = Vec::new();

                for &(pos, dir) in perimeter {
                    let side = sides
                        .iter()
                        .position(|&(s_pos, s_dir)| dir == s_dir && pos.manhattan(s_pos) == 1);

                    if let Some(side) = side {
                        sides[side] = (pos, dir);
                    } else {
                        sides.push((pos, dir));
                    }
                }

                area.len() * sides.len()
            })
            .sum()
    }
}

aoc_utils::run!(2024, 12, Day12);

fn get_areas_perimeters(map: &Grid<char>) -> <Day12 as Solution>::Input<'_> {
    let mut unvisited = map.positions().collect::<HashSet<_>>();

    let mut areas_perimeters = Vec::new();

    while let Some(&start) = unvisited.iter().next() {
        let region = map[start];
        let mut area = HashSet::new();
        let mut perimeter = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back(start);
        area.insert(start);
        unvisited.remove(&start);

        while let Some(pos) = queue.pop_front() {
            for direction in Direction::ALL4 {
                match map.step(pos, direction) {
                    Some(next_pos) if map[next_pos] == region => {
                        if unvisited.remove(&next_pos) {
                            queue.push_back(next_pos);
                            area.insert(next_pos);
                        }
                    }
                    _ => {
                        perimeter.insert((pos, direction));
                    }
                }
            }
        }

        areas_perimeters.push((
            area.into_iter().collect(),
            perimeter.into_iter().sorted().collect(),
        ));
    }

    areas_perimeters
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 12, Day12, examples = [1, 3, 4, 6, 7]);

    #[rstest]
    fn part1_test1(data1: <Day12 as Solution>::Input<'_>) {
        assert_eq!(140, Day12::part1(&data1));
    }

    #[rstest]
    fn part1_test2(data2: <Day12 as Solution>::Input<'_>) {
        assert_eq!(772, Day12::part1(&data2));
    }

    #[rstest]
    fn part1_test3(data3: <Day12 as Solution>::Input<'_>) {
        assert_eq!(1930, Day12::part1(&data3));
    }

    #[rstest]
    fn part2_test1(data1: <Day12 as Solution>::Input<'_>) {
        assert_eq!(80, Day12::part2(&data1));
    }

    #[rstest]
    fn part2_test2(data2: <Day12 as Solution>::Input<'_>) {
        assert_eq!(436, Day12::part2(&data2));
    }

    #[rstest]
    fn part2_test3(data3: <Day12 as Solution>::Input<'_>) {
        assert_eq!(1206, Day12::part2(&data3));
    }

    #[rstest]
    fn part2_test4(data4: <Day12 as Solution>::Input<'_>) {
        assert_eq!(236, Day12::part2(&data4));
    }

    #[rstest]
    fn part2_test5(data5: <Day12 as Solution>::Input<'_>) {
        assert_eq!(368, Day12::part2(&data5));
    }
}
