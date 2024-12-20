type ParsedInput = (Vec<String>, Vec<String>);
type ProcessedInput = (Vec<String>, Vec<String>);

fn main() {
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &str) -> ParsedInput {
    let parts = input.split_once("\n\n").unwrap();

    (
        parts
            .0
            .split(", ")
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        parts
            .1
            .lines()
            .filter(|line| !line.is_empty())
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
    )
}

fn process_input(input: &str) -> ProcessedInput {
    parse_input(input)
}

fn possible_arrangements(design: &str, patterns: &[String]) -> usize {
    let mut arrangements = vec![0; design.len()];

    for index in (0..design.len()).rev() {
        let sub_design = &design[index..];

        for pattern in patterns {
            if sub_design == pattern {
                arrangements[index] += 1;
            } else if sub_design.starts_with(pattern) {
                arrangements[index] += arrangements[index + pattern.len()];
            }
        }
    }

    arrangements[0]
}

fn part1((patterns, designs): &ProcessedInput) -> usize {
    designs
        .iter()
        .filter(|design| possible_arrangements(design, patterns) > 0)
        .count()
}

fn part2((patterns, designs): &ProcessedInput) -> usize {
    designs
        .iter()
        .map(|design| possible_arrangements(design, patterns))
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn data() -> ProcessedInput {
        let input = include_str!("input_test.txt");
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
