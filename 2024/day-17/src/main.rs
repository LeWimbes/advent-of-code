use winnow::ascii::{dec_uint, multispace0, multispace1};
use winnow::combinator::{delimited, separated};
use winnow::{PResult, Parser};

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 17);

type ParsedInput = (u64, u64, u64, Vec<u64>);
type ProcessedInput = (u64, u64, u64, Vec<u64>);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &mut &str) -> PResult<ParsedInput> {
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

fn process_input(input: &'static str) -> ProcessedInput {
    parse_input
        .parse(input)
        .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
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

fn part1((a, b, c, program): &ProcessedInput) -> String {
    run_program(*a, *b, *c, program)
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
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

fn part2((_a, b, c, program): &ProcessedInput) -> u64 {
    dfs_a(*b, *c, program)
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
        assert_eq!(part1(&data1), "4,6,3,5,6,3,5,2,1,0");
    }

    #[rstest]
    fn part2_test(data2: ProcessedInput) {
        assert_eq!(part2(&data2), 117_440);
    }
}
