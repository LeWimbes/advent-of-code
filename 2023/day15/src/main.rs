use aoc_utils::Solution;
use regex::Regex;

struct Day15;

impl Solution for Day15 {
    type Input<'a> = Vec<&'a str>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        input.lines().next().unwrap().split(',').collect()
    }

    fn part1(instructions: &Self::Input<'_>) -> Self::Output1 {
        instructions
            .iter()
            .map(|instruction| hash(instruction) as u64)
            .sum()
    }

    fn part2(instructions: &Self::Input<'_>) -> Self::Output2 {
        let re = Regex::new(r"(?<label>.+)[-=](?<focal_length>\d*)").unwrap();
        let instructions: Vec<_> = instructions
            .iter()
            .map(|instruction| {
                let caps = re.captures(instruction).unwrap();
                let label = caps.get(1).unwrap().as_str();
                let box_index = hash(label) as usize;
                let focal_length = caps.get(2).unwrap().as_str();
                if focal_length.is_empty() {
                    Instruction::Remove(box_index, label)
                } else {
                    Instruction::Add(box_index, label, focal_length.parse().unwrap())
                }
            })
            .collect();

        let mut boxes: Vec<Vec<Lens>> = vec![Vec::new(); 256];

        instructions
            .iter()
            .for_each(|instruction| match instruction {
                Instruction::Remove(box_index, label) => {
                    let index = boxes[*box_index]
                        .iter()
                        .position(|lens| &lens.label == label);
                    if let Some(index) = index {
                        boxes[*box_index].remove(index);
                    }
                }
                Instruction::Add(box_index, label, focal_length) => {
                    let index = boxes[*box_index]
                        .iter()
                        .position(|lens| &lens.label == label);
                    let new_lens = Lens::from(label, *focal_length);
                    if let Some(index) = index {
                        boxes[*box_index][index] = new_lens;
                    } else {
                        boxes[*box_index].push(new_lens);
                    }
                }
            });

        boxes
            .iter()
            .enumerate()
            .map(|(box_index, lens_box)| {
                let box_part = (box_index + 1) as u64;
                lens_box
                    .iter()
                    .enumerate()
                    .map(|(lens_index, lens)| {
                        (lens_index + 1) as u64 * u64::from(lens.focal_length)
                    })
                    .sum::<u64>()
                    * box_part
            })
            .sum::<u64>()
    }
}

aoc_utils::run!(2023, 15, Day15);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Instruction<'a> {
    Remove(usize, &'a str),
    Add(usize, &'a str, u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Lens<'a> {
    label: &'a str,
    focal_length: u8,
}

impl<'a> Lens<'a> {
    fn from(label: &'a str, focal_length: u8) -> Self {
        Lens {
            label,
            focal_length,
        }
    }
}

fn hash(string: &str) -> u8 {
    let mut current_value: u8 = 0;

    string.chars().for_each(|char| {
        let ascii = char as u8;
        current_value = current_value.wrapping_add(ascii);
        current_value = current_value.wrapping_mul(17);
    });

    current_value
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 15, Day15);

    #[rstest]
    fn part1_test(data1: <Day15 as Solution>::Input<'_>) {
        assert_eq!(1320, Day15::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day15 as Solution>::Input<'_>) {
        assert_eq!(145, Day15::part2(&data1));
    }
}
