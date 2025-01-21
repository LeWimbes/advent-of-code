const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 6);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &'static str) -> (Vec<u64>, Vec<u64>) {
    let lines = input.split_once('\n').unwrap();

    (
        lines
            .0
            .split_whitespace()
            .skip(1)
            .map(|num| num.parse().unwrap())
            .collect(),
        lines
            .1
            .split_whitespace()
            .skip(1)
            .map(|num| num.parse().unwrap())
            .collect(),
    )
}

fn part1(races: &(Vec<u64>, Vec<u64>)) -> u64 {
    races
        .0
        .iter()
        .enumerate()
        .map(|(i, time)| (*time, races.1[i]))
        .map(|(time, distance)| {
            (0..=time)
                .filter(|charge| (time - charge) * charge > distance)
                .count() as u64
        })
        .product()
}

fn part2(races: &(Vec<u64>, Vec<u64>)) -> u64 {
    let time = races.0.iter().fold(0u64, |acc, part| {
        acc * (10u64.pow(part.checked_ilog10().unwrap_or(0) + 1)) + part
    });
    let distance = races.1.iter().fold(0u64, |acc, part| {
        acc * (10u64.pow(part.checked_ilog10().unwrap_or(0) + 1)) + part
    });

    (0..=time)
        .filter(|charge| (time - charge) * charge > distance)
        .count() as u64
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> (Vec<u64>, Vec<u64>) {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: (Vec<u64>, Vec<u64>)) {
        assert_eq!(part1(&data), 288);
    }

    #[rstest]
    fn part2_test(data: (Vec<u64>, Vec<u64>)) {
        assert_eq!(part2(&data), 71503);
    }
}
