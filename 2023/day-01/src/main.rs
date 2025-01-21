const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 1);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &'static str) -> Vec<&'static str> {
    input.lines().collect()
}

fn part1(lines: &Vec<&str>) -> u32 {
    lines
        .iter()
        .map(|line| {
            let mut it = line.chars().filter_map(|char| char.to_digit(10));

            let first = it.next().expect("First digit expected!");
            let last = it.last().unwrap_or(first);

            first * 10 + last
        })
        .sum()
}

fn part2(lines: &Vec<&str>) -> u32 {
    let mapping = [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ];

    lines
        .iter()
        .map(|line| {
            let mut it = (0..line.len()).filter_map(|start| {
                let sub_line = &line[start..];
                for (value_str, value) in mapping {
                    if sub_line.starts_with(value_str) {
                        return Some(value);
                    }
                }
                sub_line.chars().next().unwrap().to_digit(10)
            });

            let first = it.next().expect("First digit expected!");
            let last = it.last().unwrap_or(first);

            first * 10 + last
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
    fn data1<'a>() -> Vec<&'a str> {
        let input = include_str!("test_input1.txt");
        process_input(input)
    }

    #[fixture]
    fn data2<'a>() -> Vec<&'a str> {
        let input = include_str!("test_input2.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data1: Vec<&str>) {
        assert_eq!(part1(&data1), 142);
    }

    #[rstest]
    fn part2_test(data2: Vec<&str>) {
        assert_eq!(part2(&data2), 281);
    }
}
