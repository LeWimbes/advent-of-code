use good_lp::{Expression, Solution, SolverModel, default_solver, variables};
use winnow::ascii::{dec_uint, multispace0, multispace1};
use winnow::combinator::{delimited, separated, seq, terminated};
use winnow::token::take_while;
use winnow::{Parser, Result};

use aoc_utils::Solution as AocSolution;

struct Day10;

impl AocSolution for Day10 {
    type Input<'a> = Vec<Machine>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        parse_input
            .parse(input)
            .unwrap_or_else(|err| panic!("Couldn't parse input:\n{err}"))
    }

    fn part1(machines: &Self::Input<'_>) -> Self::Output1 {
        machines
            .iter()
            .map(|machine| machine.bfs_light_diagram())
            .sum()
    }

    fn part2(machines: &Self::Input<'_>) -> Self::Output2 {
        machines
            .iter()
            .map(|machine| machine.solve_joltage_requirements())
            .sum()
    }
}

aoc_utils::run!(2025, 10, Day10);

/// Convert a light diagram string to a bitflag representation.
///
/// Example:
/// "##..#." -> 0b010011
fn light_diagram_to_bitflag(diagram: &str) -> u16 {
    diagram.chars().rev().fold(0u16, |acc, c| {
        (acc << 1)
            | match c {
                '#' => 1,
                '.' => 0,
                _ => panic!("Invalid character in light diagram: {}", c),
            }
    })
}

/// Convert a button wiring schematic to a bitflag representation.
///
/// Example:
/// [0, 2, 3] -> 0b1101
fn button_wiring_schematic_to_bitflag(schematic: &[u8]) -> u16 {
    schematic.iter().fold(0u16, |acc, &pos| acc | (1 << pos))
}

struct Machine {
    light_diagram: u16,
    button_wirings: Vec<u16>,
    joltage_requirements: Vec<u16>,
}

impl Machine {
    fn bfs_light_diagram(&self) -> usize {
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();

        queue.push_back((0u16, 0));
        visited.insert(0u16);

        while let Some((current, presses)) = queue.pop_front() {
            if current == self.light_diagram {
                return presses;
            }

            for &wiring in &self.button_wirings {
                let next = current ^ wiring;
                if visited.insert(next) {
                    queue.push_back((next, presses + 1));
                }
            }
        }

        usize::MAX
    }

    fn solve_joltage_requirements(&self) -> usize {
        let n_positions = self.joltage_requirements.len();

        // Create a variable for each button
        variables! {
            vars:
                0 <= button_vars[self.button_wirings.len()] (integer)
        }

        // Minimize the total number of presses
        let objective: Expression = button_vars.iter().sum();
        let mut problem = vars.minimise(objective).using(default_solver);

        // Add Constraints
        for pos in 0..n_positions {
            let constraint_row: Expression = button_vars
                .iter()
                .enumerate()
                .map(|(btn_idx, &var)| var * (self.button_wirings[btn_idx] >> pos & 1))
                .sum();

            problem.add_constraint(constraint_row.eq(self.joltage_requirements[pos]));
        }

        match problem.solve() {
            Ok(solution) => button_vars
                .iter()
                .map(|&v| solution.value(v).round() as usize)
                .sum(),
            Err(_) => usize::MAX,
        }
    }
}

fn parse_light_diagram(input: &mut &str) -> Result<u16> {
    delimited('[', take_while(1.., |c| c == '#' || c == '.'), ']')
        .map(light_diagram_to_bitflag)
        .parse_next(input)
}

fn parse_button_wiring(input: &mut &str) -> Result<u16> {
    delimited('(', separated(1.., dec_uint::<_, u8, _>, ','), ')')
        .map(|schematic: Vec<u8>| button_wiring_schematic_to_bitflag(&schematic))
        .parse_next(input)
}

fn parse_button_wirings(input: &mut &str) -> Result<Vec<u16>> {
    separated(0.., parse_button_wiring, multispace1).parse_next(input)
}

fn parse_joltage_requirements(input: &mut &str) -> Result<Vec<u16>> {
    delimited('{', separated(0.., dec_uint::<_, u16, _>, ','), '}').parse_next(input)
}

fn parse_machine(input: &mut &str) -> Result<Machine> {
    seq!(
        parse_light_diagram,
        _: multispace1,
        parse_button_wirings,
        _: multispace1,
        parse_joltage_requirements
    )
    .map(
        |(light_diagram, button_wirings, joltage_requirements)| Machine {
            light_diagram,
            button_wirings,
            joltage_requirements,
        },
    )
    .parse_next(input)
}

fn parse_input<'a>(input: &mut &str) -> Result<<Day10 as AocSolution>::Input<'a>> {
    terminated(separated(0.., parse_machine, multispace1), multispace0).parse_next(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2025, 10, Day10);

    #[rstest]
    fn part1_test(data1: <Day10 as AocSolution>::Input<'_>) {
        assert_eq!(7, Day10::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day10 as AocSolution>::Input<'_>) {
        assert_eq!(33, Day10::part2(&data1));
    }
}
