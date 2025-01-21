use regex::Regex;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 15);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &'static str) -> Vec<&'static str> {
    input.lines().next().unwrap().split(',').collect()
}

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

fn part1(instructions: &Vec<&str>) -> u64 {
    instructions
        .iter()
        .map(|instruction| hash(instruction) as u64)
        .sum()
}

fn part2(instructions: &Vec<&str>) -> u64 {
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
                .map(|(lens_index, lens)| (lens_index + 1) as u64 * u64::from(lens.focal_length))
                .sum::<u64>()
                * box_part
        })
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data<'a>() -> Vec<&'a str> {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<&str>) {
        assert_eq!(part1(&data), 1320);
    }

    #[rstest]
    fn part2_test(data: Vec<&str>) {
        assert_eq!(part2(&data), 145);
    }
}
