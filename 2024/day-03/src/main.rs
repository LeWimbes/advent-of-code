use regex::Regex;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 3);

enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

type ParsedInput = Vec<Instruction>;
type ProcessedInput = Vec<Instruction>;

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    let re = Regex::new(
        r"(?<mul>(mul\((?<x>\d{1,3}),(?<y>\d{1,3})\)))|(?<do>do\(\))|(?<dont>don't\(\))",
    )
    .unwrap();

    re.captures_iter(input)
        .map(|caps| {
            if caps.name("mul").is_some() {
                Instruction::Mul(caps["x"].parse().unwrap(), caps["y"].parse().unwrap())
            } else if caps.name("do").is_some() {
                Instruction::Do
            } else {
                Instruction::Dont
            }
        })
        .collect()
}

fn process_input(input: &'static str) -> ProcessedInput {
    parse_input(input)
}

fn part1(instructions: &ProcessedInput) -> u32 {
    instructions
        .iter()
        .filter_map(|instr| match instr {
            Instruction::Mul(x, y) => Some(x * y),
            Instruction::Do | Instruction::Dont => None,
        })
        .sum()
}

fn part2(instructions: &ProcessedInput) -> u32 {
    let mut enabled = true;

    instructions
        .iter()
        .filter_map(|instr| match instr {
            Instruction::Mul(x, y) => {
                if enabled {
                    Some(x * y)
                } else {
                    None
                }
            }
            Instruction::Do => {
                enabled = true;
                None
            }
            Instruction::Dont => {
                enabled = false;
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data1() -> ProcessedInput {
        let input = include_str!("test_input1.txt");
        process_input(input)
    }

    #[fixture]
    fn data2() -> ProcessedInput {
        let input = include_str!("test_input2.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data1: ProcessedInput) {
        assert_eq!(part1(&data1), 161);
    }

    #[rstest]
    fn part2_test(data2: ProcessedInput) {
        assert_eq!(part2(&data2), 48);
    }
}
