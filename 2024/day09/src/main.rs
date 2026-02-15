use std::cmp::Ordering;

use aoc_utils::Solution;

struct Day09;

impl Solution for Day09 {
    type Input<'a> = Vec<(Option<u64>, u64)>;
    type Output1 = u64;
    type Output2 = u64;

    fn process(input: &str) -> Self::Input<'_> {
        let map: Vec<_> = input
            .trim()
            .chars()
            .map(|c| u64::from(c.to_digit(10).unwrap()))
            .collect();

        let mut enhanced_map = Vec::new();

        let mut data = true;
        let mut next_id = 0;

        for value in map {
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

    fn part1(map: &Self::Input<'_>) -> Self::Output1 {
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

    fn part2(map: &Self::Input<'_>) -> Self::Output2 {
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
}

aoc_utils::run!(2024, 9, Day09);

fn map_to_blocks(map: &<Day09 as Solution>::Input<'_>) -> Vec<Option<u64>> {
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

fn map_to_checksum(map: &<Day09 as Solution>::Input<'_>) -> u64 {
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

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 9, Day09);

    #[rstest]
    fn part1_test(data1: <Day09 as Solution>::Input<'_>) {
        assert_eq!(1928, Day09::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day09 as Solution>::Input<'_>) {
        assert_eq!(2858, Day09::part2(&data1));
    }
}
