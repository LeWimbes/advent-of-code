use std::collections::HashMap;
use std::collections::hash_map::Entry;

use aoc_utils::Solution;

struct Day14;

impl Solution for Day14 {
    type Input<'a> = Vec<Vec<char>>;
    type Output1 = usize;
    type Output2 = usize;

    fn process(input: &str) -> Self::Input<'_> {
        input.lines().map(|line| line.chars().collect()).collect()
    }

    fn part1(platform: &Self::Input<'_>) -> Self::Output1 {
        let mut platform = platform.to_vec();
        tilt_north(&mut platform);
        calculate_load_on_north_beam(&platform)
    }

    fn part2(platform: &Self::Input<'_>) -> Self::Output2 {
        let mut platform = platform.to_vec();

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
}

aoc_utils::run!(2023, 14, Day14);

fn tilt_north(platform: &mut <Day14 as Solution>::Input<'_>) {
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

fn tilt_west(platform: &mut <Day14 as Solution>::Input<'_>) {
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

fn tilt_south(platform: &mut <Day14 as Solution>::Input<'_>) {
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

fn tilt_east(platform: &mut <Day14 as Solution>::Input<'_>) {
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

fn calculate_load_on_north_beam(platform: &<Day14 as Solution>::Input<'_>) -> usize {
    platform
        .iter()
        .zip((1..=platform.len()).rev())
        .map(|(row, load_caused_by_one)| {
            row.iter().filter(|tile| **tile == 'O').count() * load_caused_by_one
        })
        .sum::<usize>()
}

fn platform_to_string(platform: &<Day14 as Solution>::Input<'_>) -> String {
    platform
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2023, 14, Day14);

    #[rstest]
    fn part1_test(data1: <Day14 as Solution>::Input<'_>) {
        assert_eq!(136, Day14::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day14 as Solution>::Input<'_>) {
        assert_eq!(64, Day14::part2(&data1));
    }
}
