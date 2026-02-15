use aoc_utils::Solution;
use regex::Regex;

struct Day03;

impl Solution for Day03 {
    type Input<'a> = Vec<Instruction>;
    type Output1 = u32;
    type Output2 = u32;

    fn process(input: &str) -> Self::Input<'_> {
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

    fn part1(instructions: &Self::Input<'_>) -> Self::Output1 {
        instructions
            .iter()
            .filter_map(|instr| match instr {
                Instruction::Mul(x, y) => Some(x * y),
                Instruction::Do | Instruction::Dont => None,
            })
            .sum()
    }

    fn part2(instructions: &Self::Input<'_>) -> Self::Output2 {
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
}

enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

aoc_utils::run!(2024, 3, Day03);

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 3, Day03);

    #[rstest]
    fn part1_test(data1: <Day03 as Solution>::Input<'_>) {
        assert_eq!(161, Day03::part1(&data1));
    }

    #[rstest]
    fn part2_test(data2: <Day03 as Solution>::Input<'_>) {
        assert_eq!(48, Day03::part2(&data2));
    }
}
