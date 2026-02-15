use std::iter::once;

use aoc_utils::Solution;

struct Day06;

impl Solution for Day06 {
    type Input<'a> = Vec<TaskNew>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        let lines: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let operator_line = lines.last().unwrap();

        // Find column starts in last row (position of the operators)
        let starting_indices = operator_line
            .iter()
            .enumerate()
            .filter(|(_, c)| **c != ' ')
            .map(|(i, _)| i)
            .chain(once(operator_line.len() + 1)) // End of last column
            .collect::<Vec<_>>();

        starting_indices
            .iter()
            .zip(starting_indices.iter().skip(1))
            .map(|(&start, &end)| {
                let end = end - 1; // Skip the blank column between blocks
                let mut nums = Vec::new();
                for line in &lines[..lines.len() - 1] {
                    let slice = &line[start..end];
                    let num = slice
                        .iter()
                        .map(|c| c.to_digit(10).unwrap_or(10) as u8) // Use 10 to represent blank field
                        .collect();
                    nums.push(num);
                }

                match operator_line[start] {
                    '+' => TaskNew {
                        digits: nums,
                        operator: Operator::Addition,
                    },
                    '*' => TaskNew {
                        digits: nums,
                        operator: Operator::Multiplication,
                    },
                    _ => unreachable!(),
                }
            })
            .collect()
    }

    fn part1(tasks: &Self::Input<'_>) -> Self::Output1 {
        tasks.iter().map(|t| t.execute_row_wise()).sum()
    }

    fn part2(tasks: &Self::Input<'_>) -> Self::Output2 {
        tasks.iter().map(|t| t.execute_column_wise()).sum()
    }
}

aoc_utils::run!(2025, 6, Day06);

enum Operator {
    Addition,
    Multiplication,
}

impl Operator {
    fn apply(&self, nums: impl Iterator<Item = u64>) -> u64 {
        match self {
            Operator::Addition => nums.sum(),
            Operator::Multiplication => nums.product(),
        }
    }
}

struct TaskNew {
    digits: Vec<Vec<u8>>,
    operator: Operator,
}

impl TaskNew {
    fn execute_row_wise(&self) -> u64 {
        let nums = self.digits.iter().map(|r| {
            r.iter()
                .filter(|&&d| d != 10)
                .fold(0u64, |acc, &d| acc * 10 + d as u64)
        });
        self.operator.apply(nums)
    }

    fn execute_column_wise(&self) -> u64 {
        let nums = (0..self.digits.first().unwrap().len()).map(|col| {
            self.digits
                .iter()
                .filter(|row| row[col] != 10)
                .fold(0u64, |acc, row| acc * 10 + row[col] as u64)
        });
        self.operator.apply(nums)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2025, 6, Day06);

    #[rstest]
    fn part1_test(data1: <Day06 as Solution>::Input<'_>) {
        assert_eq!(4_277_556, Day06::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day06 as Solution>::Input<'_>) {
        assert_eq!(3_263_827, Day06::part2(&data1));
    }
}
