use std::collections::hash_map::Entry;
use std::collections::HashMap;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2023, 14);

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &'static str) -> Vec<Vec<char>> {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn tilt_north(platform: &mut [Vec<char>]) {
    for x in 0..platform[0].len() {
        let mut next_y = 0;
        for y in 0..platform.len() {
            match platform[y][x] {
                '#' => next_y = y + 1,
                'O' => {
                    platform[y][x] = '.';
                    platform[next_y][x] = 'O';
                    next_y += 1;
                }
                _ => {}
            }
        }
    }
}

fn tilt_west(platform: &mut [Vec<char>]) {
    for y in 0..platform.len() {
        let mut next_x = 0;
        for x in 0..platform[0].len() {
            match platform[y][x] {
                '#' => next_x = x + 1,
                'O' => {
                    platform[y][x] = '.';
                    platform[y][next_x] = 'O';
                    next_x += 1;
                }
                _ => {}
            }
        }
    }
}

fn tilt_south(platform: &mut [Vec<char>]) {
    for x in 0..platform[0].len() {
        let mut next_y = platform.len() - 1;
        for y in (0..platform.len()).rev() {
            match platform[y][x] {
                '#' => next_y = y.saturating_sub(1),
                'O' => {
                    platform[y][x] = '.';
                    platform[next_y][x] = 'O';
                    next_y = next_y.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
}

fn tilt_east(platform: &mut [Vec<char>]) {
    for y in 0..platform.len() {
        let mut next_x = platform[0].len() - 1;
        for x in (0..platform[0].len()).rev() {
            match platform[y][x] {
                '#' => next_x = x.saturating_sub(1),
                'O' => {
                    platform[y][x] = '.';
                    platform[y][next_x] = 'O';
                    next_x = next_x.saturating_sub(1);
                }
                _ => {}
            }
        }
    }
}

fn calculate_load_on_north_beam(platform: &[Vec<char>]) -> u64 {
    platform
        .iter()
        .zip((1..=platform.len()).rev())
        .map(|(row, load_caused_by_one)| {
            row.iter().filter(|tile| **tile == 'O').count() * load_caused_by_one
        })
        .sum::<usize>() as u64
}

fn platform_to_string(platform: &[Vec<char>]) -> String {
    platform
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn part1(platform: &Vec<Vec<char>>) -> u64 {
    let mut platform = platform.clone();
    tilt_north(&mut platform);
    calculate_load_on_north_beam(&platform)
}

fn part2(platform: &Vec<Vec<char>>) -> u64 {
    let mut platform = platform.clone();

    let mut patterns: HashMap<String, usize> = HashMap::new();
    patterns.insert(platform_to_string(&platform), 0);

    let total_tilting_cycles = 1_000_000_000;

    let mut tilting_cycle: usize = 1;
    while tilting_cycle <= total_tilting_cycles {
        tilt_north(&mut platform);
        tilt_west(&mut platform);
        tilt_south(&mut platform);
        tilt_east(&mut platform);

        let string = platform_to_string(&platform);
        match patterns.entry(string) {
            Entry::Occupied(entry) => {
                let cycle_length = tilting_cycle - *entry.get();
                let cycles = (total_tilting_cycles - tilting_cycle) / cycle_length;
                if cycles != 0 {
                    tilting_cycle += cycles * cycle_length;
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(tilting_cycle);
            }
        }
        tilting_cycle += 1;
    }

    calculate_load_on_north_beam(&platform)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[cfg(feature = "test-answers")]
    aoc_macros::test_answers!(true);

    #[fixture]
    fn data() -> Vec<Vec<char>> {
        let input = include_str!("test_input.txt");
        process_input(input)
    }

    #[rstest]
    fn part1_test(data: Vec<Vec<char>>) {
        assert_eq!(part1(&data), 136);
    }

    #[rstest]
    fn part2_test(data: Vec<Vec<char>>) {
        assert_eq!(part2(&data), 64);
    }
}
