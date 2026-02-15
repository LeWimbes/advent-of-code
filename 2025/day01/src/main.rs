use aoc_utils::Solution;
use winnow::ascii::{dec_int, multispace0, multispace1};
use winnow::combinator::{separated, terminated};
use winnow::token::one_of;
use winnow::{Parser, Result};

struct Day01;

impl Solution for Day01 {
    type Input<'a> = Vec<i32>;
    type Output1 = u32;
    type Output2 = u32;

    fn process(input: &str) -> Self::Input<'_> {
        parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
            .into_iter()
            .map(|(dir, dist)| match dir {
                'L' => -dist,
                'R' => dist,
                _ => unreachable!(),
            })
            .collect()
    }

    fn part1(instructions: &Self::Input<'_>) -> Self::Output1 {
        count_zeros(instructions)
    }

    fn part2(instructions: &Self::Input<'_>) -> Self::Output2 {
        let expanded_instructions: Self::Input<'_> = instructions
            .iter()
            .flat_map(|instruction| vec![instruction.signum(); instruction.unsigned_abs() as usize])
            .collect();

        count_zeros(&expanded_instructions)
    }
}

aoc_utils::run!(2025, 1, Day01);

fn parse_input(input: &mut &str) -> Result<Vec<(char, i32)>> {
    terminated(
        separated(0.., (one_of(['L', 'R']), dec_int), multispace1),
        multispace0,
    )
    .parse_next(input)
}

fn count_zeros(instructions: &<Day01 as Solution>::Input<'_>) -> u32 {
    let mut at_zero: u32 = 0;
    let mut pos: i32 = 50;
    for instruction in instructions {
        pos = (pos + *instruction).rem_euclid(100);
        if pos == 0 {
            at_zero += 1;
        }
    }
    at_zero
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2025, 1, Day01);

    #[rstest]
    fn part1_test(data1: <Day01 as Solution>::Input<'_>) {
        assert_eq!(3, Day01::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day01 as Solution>::Input<'_>) {
        assert_eq!(6, Day01::part2(&data1));
    }
}
