use std::collections::HashMap;

use num::Integer;
use regex::Regex;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 8);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(
    input: &'static str,
) -> (
    Vec<usize>,
    HashMap<&'static str, (&'static str, &'static str)>,
) {
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

fn part1(instructions_map: &(Vec<usize>, HashMap<&str, (&str, &str)>)) -> u64 {
    let (instructions, map) = instructions_map;

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
        .unwrap() as u64
}

fn part2(instructions_map: &(Vec<usize>, HashMap<&str, (&str, &str)>)) -> u64 {
    let (instructions, map) = instructions_map;

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
                .unwrap() as u64
        })
        .fold(1, |acc, steps| steps.lcm(&acc))
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data11<'a>() -> (Vec<usize>, HashMap<&'a str, (&'a str, &'a str)>) {
        let input = include_str!("test_input1.txt");
        process_input(input)
    }

    #[fixture]
    fn data12<'a>() -> (Vec<usize>, HashMap<&'a str, (&'a str, &'a str)>) {
        let input = include_str!("test_input2.txt");
        process_input(input)
    }

    #[fixture]
    fn data2<'a>() -> (Vec<usize>, HashMap<&'a str, (&'a str, &'a str)>) {
        let input = include_str!("test_input3.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test1(data11: (Vec<usize>, HashMap<&str, (&str, &str)>)) {
        assert_eq!(part1(&data11), 2);
    }

    #[rstest]
    fn part1_test2(data12: (Vec<usize>, HashMap<&str, (&str, &str)>)) {
        assert_eq!(part1(&data12), 6);
    }

    #[rstest]
    fn part2_test1(data2: (Vec<usize>, HashMap<&str, (&str, &str)>)) {
        assert_eq!(part2(&data2), 6);
    }
}
