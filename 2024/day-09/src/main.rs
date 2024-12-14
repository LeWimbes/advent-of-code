use std::cmp::Ordering;

type ParsedInput = Vec<char>;
type ProcessedInput = Vec<(Option<u64>, u64)>;

fn main() {
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &str) -> ParsedInput {
    input.trim().chars().collect()
}

fn process_input(input: &str) -> ProcessedInput {
    let map = parse_input(input);

    let mut enhanced_map = Vec::new();

    let mut data = true;
    let mut next_id = 0;

    for c in map {
        let value = u64::from(c.to_digit(10).unwrap());

        if data {
            enhanced_map.push((Some(next_id), value));
            next_id += 1;
        } else {
            enhanced_map.push((None, value));
        }

        data = !data;
    }

    enhanced_map
}

fn map_to_blocks(map: &Vec<(Option<u64>, u64)>) -> Vec<Option<u64>> {
    let mut blocks = Vec::new();

    for (value, amount) in map {
        for _i in 0..*amount {
            blocks.push(*value);
        }
    }

    blocks
}

fn blocks_to_checksum(blocks: &[Option<u64>]) -> u64 {
    blocks
        .iter()
        .map(|block| block.unwrap_or(0))
        .enumerate()
        .map(|(i, value)| i as u64 * value)
        .sum()
}

fn part1(map: &ProcessedInput) -> u64 {
    let mut blocks = map_to_blocks(map);

    let mut next_empty = blocks
        .iter()
        .enumerate()
        .find(|(_i, block)| block.is_none())
        .map(|(i, _block)| i)
        .unwrap();
    let mut next_full = blocks
        .iter()
        .enumerate()
        .rev()
        .find_map(|(i, block)| block.map(|_| i))
        .unwrap();

    while next_empty < next_full {
        blocks.swap(next_empty, next_full);

        next_empty += 1;
        while blocks[next_empty].is_some() {
            next_empty += 1;
        }

        next_full -= 1;
        while blocks[next_full].is_none() {
            next_full -= 1;
        }
    }

    blocks_to_checksum(&blocks)
}

fn map_to_checksum(map: &[(Option<u64>, u64)]) -> u64 {
    let mut checksum = 0;
    let mut index = 0;

    for (value, amount) in map {
        let value = value.unwrap_or(0);
        for _ in 0..*amount {
            checksum += index * value;
            index += 1;
        }
    }

    checksum
}

fn part2(map: &ProcessedInput) -> u64 {
    let mut map = map.clone();

    let mut index = map.len() - 1;
    while index > 0 {
        let block = map[index];
        let (value, amount) = block;

        if value.is_some() {
            for new_index in 0..index {
                let (new_value, new_amount) = map[new_index];
                if new_value.is_none() {
                    match amount.cmp(&new_amount) {
                        Ordering::Less => {
                            map[new_index].1 = new_amount - amount;
                            map[index].0 = None;
                            map.insert(new_index, block);
                            break;
                        }
                        Ordering::Equal => {
                            map.swap(index, new_index);
                            break;
                        }
                        Ordering::Greater => {}
                    }
                }
            }
        }

        index -= 1;
    }

    map_to_checksum(&map)
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
        assert_eq!(part1(&data), 1928);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2(&data), 2858);
    }
}
