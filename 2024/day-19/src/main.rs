const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 19);

type ParsedInput = (Vec<&'static str>, Vec<&'static str>);
type ProcessedInput = Vec<usize>;

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    let parts = input.split_once("\n\n").unwrap();

    (
        parts.0.split(", ").collect::<Vec<_>>(),
        parts
            .1
            .lines()
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>(),
    )
}

fn possible_arrangements(design: &str, patterns: &[&str]) -> usize {
    let mut arrangements = vec![0; design.len()];

    for index in (0..design.len()).rev() {
        let sub_design = &design[index..];

        for &pattern in patterns {
            if sub_design == pattern {
                arrangements[index] += 1;
            } else if sub_design.starts_with(pattern) {
                arrangements[index] += arrangements[index + pattern.len()];
            }
        }
    }

    arrangements[0]
}

fn process_input(input: &'static str) -> ProcessedInput {
    let (patterns, designs) = parse_input(input);

    designs
        .iter()
        .map(|design| possible_arrangements(design, &patterns))
        .collect()
}

fn part1(arrangements: &ProcessedInput) -> usize {
    arrangements.iter().filter(|&&a| a > 0).count()
}

fn part2(arrangements: &ProcessedInput) -> usize {
    arrangements.iter().sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> ProcessedInput {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: ProcessedInput) {
        assert_eq!(part1(&data), 6);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2(&data), 16);
    }
}
