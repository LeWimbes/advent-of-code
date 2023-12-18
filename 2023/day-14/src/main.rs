use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn process_input(input: &str) -> Vec<Vec<char>> {
    input.lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn tilt_north(platform: &mut Vec<Vec<char>>) {
    (0..platform[0].len()).for_each(|x| {
        let mut next_y = 0;
        (0..platform.len()).for_each(|y| {
            match platform[y][x] {
                '#' => next_y = y + 1,
                'O' => {
                    platform[y][x] = '.';
                    platform[next_y][x] = 'O';
                    next_y += 1;
                }
                _ => {}
            }
        })
    })
}

fn tilt_west(platform: &mut Vec<Vec<char>>) {
    (0..platform.len()).for_each(|y| {
        let mut next_x = 0;
        (0..platform[0].len()).for_each(|x| {
            match platform[y][x] {
                '#' => next_x = x + 1,
                'O' => {
                    platform[y][x] = '.';
                    platform[y][next_x] = 'O';
                    next_x += 1;
                }
                _ => {}
            }
        })
    })
}

fn tilt_south(platform: &mut Vec<Vec<char>>) {
    (0..platform[0].len()).for_each(|x| {
        let mut next_y = platform.len() - 1;
        (0..platform.len()).rev().for_each(|y| {
            match platform[y][x] {
                '#' => next_y = y.saturating_sub(1),
                'O' => {
                    platform[y][x] = '.';
                    platform[next_y][x] = 'O';
                    next_y = next_y.saturating_sub(1);
                }
                _ => {}
            }
        })
    })
}

fn tilt_east(platform: &mut Vec<Vec<char>>) {
    (0..platform.len()).for_each(|y| {
        let mut next_x = platform[0].len() - 1;
        (0..platform[0].len()).rev().for_each(|x| {
            match platform[y][x] {
                '#' => next_x = x.saturating_sub(1),
                'O' => {
                    platform[y][x] = '.';
                    platform[y][next_x] = 'O';
                    next_x = next_x.saturating_sub(1);
                }
                _ => {}
            }
        })
    })
}

fn calculate_load_on_north_beam(platform: &Vec<Vec<char>>) -> u64 {
    platform.iter().zip((1..=platform.len()).rev()).map(|(row, load_caused_by_one)| {
        row.iter().filter(|tile| **tile == 'O').count() * load_caused_by_one
    }).sum::<usize>() as u64
}

fn platform_to_string(platform: &Vec<Vec<char>>) -> String {
    platform.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n")
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
        if patterns.contains_key(&string) {
            let cycle_length = tilting_cycle - patterns[&string];
            let cycles = (total_tilting_cycles - tilting_cycle) / cycle_length;
            if cycles != 0 {
                tilting_cycle = tilting_cycle + cycles * cycle_length;
            }
        } else {
            patterns.insert(string, tilting_cycle);
        }
        tilting_cycle += 1;
    }

    calculate_load_on_north_beam(&platform)
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    use super::*;

    #[fixture]
    fn data() -> Vec<Vec<char>> {
        let input = include_str!("input_test.txt");
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
