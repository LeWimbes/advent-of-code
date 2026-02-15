use std::collections::HashMap;

use aoc_utils::Solution;
use num::Integer;
use regex::Regex;

struct Day08;

impl Solution for Day08 {
    type Input<'a> = (Vec<usize>, HashMap<&'a str, (&'a str, &'a str)>);
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        let blocks = input.split_once("\n\n").unwrap();

        let instructions: Vec<usize> = blocks
            .0
            .chars()
            .filter_map(|char| match char {
                'L' => Some(0),
                'R' => Some(1),
                _ => None,
            })
            .collect();

        let re = Regex::new(r"(?<node>\w\w\w) = \((?<left>\w\w\w), (?<right>\w\w\w)\)").unwrap();
        let map: HashMap<_, _> = re
            .captures_iter(blocks.1)
            .map(|caps| {
                (
                    caps.name("node").unwrap().as_str(),
                    (
                        caps.name("left").unwrap().as_str(),
                        caps.name("right").unwrap().as_str(),
                    ),
                )
            })
            .collect();

        (instructions, map)
    }

    fn part1((instructions, map): &Self::Input<'_>) -> Self::Output1 {
        let mut current_node = "AAA";
        instructions
            .iter()
            .cycle()
            .enumerate()
            .find_map(|(i, instruction)| {
                current_node = match instruction {
                    0 => map[current_node].0,
                    _ => map[current_node].1,
                };
                if current_node == "ZZZ" {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn part2((instructions, map): &Self::Input<'_>) -> Self::Output2 {
        let starting_nodes: Vec<&str> = map
            .keys()
            .filter(|key| key.ends_with('A'))
            .copied()
            .collect();

        // It seems that the input is constructed in such a way that by finding the first node ending with 'Z', we have found a cycle.
        starting_nodes
            .iter()
            .map(|starting_node| {
                let mut current_node = *starting_node;
                instructions
                    .iter()
                    .cycle()
                    .enumerate()
                    .find_map(|(i, instruction)| {
                        current_node = match instruction {
                            0 => map[current_node].0,
                            _ => map[current_node].1,
                        };
                        if current_node.ends_with('Z') {
                            Some(i + 1)
                        } else {
                            None
                        }
                    })
                    .unwrap()
            })
            .fold(1, |acc, steps| steps.lcm(&acc))
    }
}

aoc_utils::run!(2023, 8, Day08);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    // Note: The automatic test input fetching retrieves incorrect data.
    aoc_utils::solution_tests!(2023, 8, Day08);

    #[rstest]
    fn part1_test1(data1: <Day08 as Solution>::Input<'_>) {
        assert_eq!(2, Day08::part1(&data1));
    }

    #[rstest]
    fn part1_test2(data2: <Day08 as Solution>::Input<'_>) {
        assert_eq!(6, Day08::part1(&data2));
    }

    #[rstest]
    fn part2_test1(data3: <Day08 as Solution>::Input<'_>) {
        assert_eq!(6, Day08::part2(&data3));
    }
}
