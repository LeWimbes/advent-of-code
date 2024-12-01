use std::collections::HashMap;
use winnow::ascii::{dec_uint, multispace0, space1};
use winnow::combinator::{repeat, separated_pair, terminated};
use winnow::{PResult, Parser};

fn main() {
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &mut &str) -> PResult<Vec<(u32, u32)>> {
    repeat(
        0..,
        terminated(separated_pair(dec_uint, space1, dec_uint), multispace0),
    )
    .parse_next(input)
}

fn process_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    parse_input.parse(input).unwrap().into_iter().unzip()
}

fn part1(lists: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut list0 = lists.0.clone();
    let mut list1 = lists.1.clone();
    list0.sort_unstable();
    list1.sort_unstable();

    list0
        .into_iter()
        .zip(list1)
        .map(|(a, b)| a.abs_diff(b))
        .sum()
}

fn part2(lists: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut frequency_map: HashMap<u32, u32> = HashMap::new();

    for &num in &lists.1 {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    lists
        .0
        .iter()
        .map(|&num| num * *frequency_map.entry(num).or_insert(0))
        .sum()
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn data() -> (Vec<u32>, Vec<u32>) {
        let input = include_str!("input_test.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: (Vec<u32>, Vec<u32>)) {
        assert_eq!(part1(&data), 11);
    }

    #[rstest]
    fn part2_test(data: (Vec<u32>, Vec<u32>)) {
        assert_eq!(part2(&data), 31);
    }
}
