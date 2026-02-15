use aoc_utils::Solution;
use winnow::ascii::{dec_uint, multispace0, multispace1};
use winnow::combinator::{delimited, separated};
use winnow::{Parser, Result};

struct Day17;

impl Solution for Day17 {
    type Input<'a> = (u64, u64, u64, Vec<u64>);
    type Output1 = String;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
    }

    fn part1((a, b, c, program): &Self::Input<'_>) -> Self::Output1 {
        run_program(*a, *b, *c, program)
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }

    fn part2((_a, b, c, program): &Self::Input<'_>) -> Self::Output2 {
        dfs_a(*b, *c, program)
    }
}

aoc_utils::run!(2024, 17, Day17);

fn parse_input<'a>(input: &mut &str) -> Result<<Day17 as Solution>::Input<'a>> {
    (
        delimited("Register A: ", dec_uint, multispace1),
        delimited("Register B: ", dec_uint, multispace1),
        delimited("Register C: ", dec_uint, multispace1),
        delimited(
            "Program: ",
            separated(1.., dec_uint::<_, u64, _>, ','),
            multispace0,
        ),
    )
        .parse_next(input)
}

fn get_combo(operand: u64, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        0..=3 => operand,
        4 => a,
        5 => b,
        6 => c,
        7 => panic!("Reserved operand {operand}"),
        _ => panic!("Unsupported operand {operand}"),
    }
}

fn run_program(a: u64, b: u64, c: u64, program: &[u64]) -> Vec<u64> {
    let mut a = a;
    let mut b = b;
    let mut c = c;

    let mut out: Vec<u64> = Vec::new();

    let mut pp = 0;
    while pp < program.len() {
        let opcode = program[pp];
        let operand = program[pp + 1];

        match opcode {
            0 => a /= 2u64.pow(get_combo(operand, a, b, c) as u32),
            1 => b ^= operand,
            2 => b = get_combo(operand, a, b, c) % 8,
            3 => {
                if a != 0 {
                    pp = operand as usize;
                    continue;
                }
            }
            4 => b ^= c,
            5 => out.push(get_combo(operand, a, b, c) % 8),
            6 => b = a / 2u64.pow(get_combo(operand, a, b, c) as u32),
            7 => c = a / 2u64.pow(get_combo(operand, a, b, c) as u32),
            _ => panic!("Unsupported opcode {opcode}"),
        }

        pp += 2;
    }

    out
}

fn dfs_a(b: u64, c: u64, program: &[u64]) -> u64 {
    let mut stack: Vec<(u64, usize)> = vec![(0, program.len() - 1)];
    let mut results = Vec::new();

    while let Some((a, pos)) = stack.pop() {
        let target = program[pos];

        for val in 0..8 {
            let mut new_a = a;
            let mask = val << (pos * 3);
            new_a ^= mask;

            let res = run_program(new_a, b, c, program);
            if res.len() == program.len() && res[pos] == target {
                if pos == 0 {
                    results.push(new_a);
                } else {
                    stack.push((new_a, pos - 1));
                }
            }
        }
    }

    results.sort_unstable();
    results[0]
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 17, Day17);

    #[rstest]
    fn part1_test(data1: <Day17 as Solution>::Input<'_>) {
        assert_eq!("4,6,3,5,6,3,5,2,1,0", Day17::part1(&data1));
    }

    #[rstest]
    fn part2_test(data2: <Day17 as Solution>::Input<'_>) {
        assert_eq!(117_440, Day17::part2(&data2));
    }
}
