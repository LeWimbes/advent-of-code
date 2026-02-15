use aoc_utils::Solution;

struct Day03;

impl Solution for Day03 {
    type Input<'a> = Vec<Vec<u8>>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        input
            .lines()
            .map(|line| {
                line.trim()
                    .chars()
                    .map(|battery| battery as u8 - b'0')
                    .collect::<Vec<_>>()
            })
            .filter(|bank| !bank.is_empty())
            .collect()
    }

    fn part1(banks: &Self::Input<'_>) -> Self::Output1 {
        banks
            .iter()
            .map(|bank| biggest_num_of_n_digits(bank, 2))
            .sum()
    }

    fn part2(banks: &Self::Input<'_>) -> Self::Output2 {
        banks
            .iter()
            .map(|bank| biggest_num_of_n_digits(bank, 12))
            .sum()
    }
}

aoc_utils::run!(2025, 3, Day03);

fn biggest_num_of_n_digits(nums: &[u8], n: usize) -> u64 {
    // Start with last n digits
    let mut digits = nums[nums.len() - n..].to_vec();

    for &num in nums.iter().take(nums.len() - n).rev() {
        // Check if num doesn't make the number smaller
        // If so, insert it at the start and shift the other digits if needed
        if num >= digits[0] {
            let mut replaced = digits[0];
            digits[0] = num;
            for digit in digits.iter_mut().skip(1) {
                if replaced >= *digit {
                    std::mem::swap(digit, &mut replaced);
                } else {
                    break;
                }
            }
        }
    }

    // Convert digits to number
    digits.iter().fold(0, |acc, &d| acc * 10 + d as u64)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2025, 3, Day03);

    #[rstest]
    fn part1_test(data1: <Day03 as Solution>::Input<'_>) {
        assert_eq!(357, Day03::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day03 as Solution>::Input<'_>) {
        assert_eq!(3_121_910_778_619, Day03::part2(&data1));
    }
}
