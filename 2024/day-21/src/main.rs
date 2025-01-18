use std::collections::VecDeque;

const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 21);

type ParsedInput = Vec<&'static str>;
type ProcessedInput = Vec<&'static str>;

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    input.lines().filter(|line| !line.is_empty()).collect()
}

fn process_input(input: &'static str) -> ProcessedInput {
    parse_input(input)
}

trait KeyIndex {
    fn as_index(&self) -> usize;

    fn count() -> usize;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum NumericalKey {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    A = 10,
}

impl NumericalKey {
    fn from_char(c: char) -> Self {
        match c {
            '0' => NumericalKey::Zero,
            '1' => NumericalKey::One,
            '2' => NumericalKey::Two,
            '3' => NumericalKey::Three,
            '4' => NumericalKey::Four,
            '5' => NumericalKey::Five,
            '6' => NumericalKey::Six,
            '7' => NumericalKey::Seven,
            '8' => NumericalKey::Eight,
            '9' => NumericalKey::Nine,
            'A' => NumericalKey::A,
            _ => panic!("Invalid char"),
        }
    }
}

impl KeyIndex for NumericalKey {
    fn as_index(&self) -> usize {
        *self as usize
    }

    fn count() -> usize {
        11
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum DirectionalKey {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    A = 4,
}

impl KeyIndex for DirectionalKey {
    fn as_index(&self) -> usize {
        *self as usize
    }

    fn count() -> usize {
        5
    }
}

fn find_shortest_paths_bfs<T>(
    grid: &[Vec<Option<T>>],
    from: (usize, usize),
    to: (usize, usize),
) -> Vec<Vec<DirectionalKey>> {
    let mut distance_parents =
        vec![vec![(usize::MAX, Vec::<(usize, usize)>::new()); grid[0].len()]; grid.len()];
    let mut queue = VecDeque::new();
    queue.push_back(from);
    distance_parents[from.1][from.0] = (0, Vec::new());

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == to {
            continue;
        }

        let current_distance = distance_parents[y][x].0;

        for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let new_x = x as i32 + dx;
            let new_y = y as i32 + dy;
            let new = (new_x as usize, new_y as usize);

            if grid[new.1][new.0].is_none() {
                continue;
            }

            let old_distance = distance_parents[new.1][new.0].0;
            let new_distance = current_distance + 1;

            if new_distance > old_distance {
                continue;
            }

            if old_distance == usize::MAX {
                distance_parents[new.1][new.0].0 = new_distance;
                distance_parents[new.1][new.0].1.push((x, y));
                queue.push_back(new);
            } else if old_distance == new_distance {
                distance_parents[new.1][new.0].1.push((x, y));
            } else {
                unreachable!("This actually shouldn't happen");
                // distance_parents[new.1][new.0].0 = new_distance;
                // distance_parents[new.1][new.0].1.clear();
                // distance_parents[new.1][new.0].1.push((x, y));
            }
        }
    }

    let mut stack = vec![(to, Vec::new())];
    let mut all_paths = Vec::new();

    while let Some((cur, path_so_far)) = stack.pop() {
        if cur == from {
            let mut forward_path = path_so_far;
            forward_path.reverse();
            forward_path.push(DirectionalKey::A);
            all_paths.push(forward_path);
            continue;
        }

        let parents = &distance_parents[cur.1][cur.0].1;
        for &(px, py) in parents {
            let dx = cur.0 as isize - px as isize;
            let dy = cur.1 as isize - py as isize;

            let step = match (dx, dy) {
                (0, 1) => DirectionalKey::Down,
                (0, -1) => DirectionalKey::Up,
                (1, 0) => DirectionalKey::Right,
                (-1, 0) => DirectionalKey::Left,
                _ => unreachable!("Invalid parent -> child step"),
            };

            let mut new_path = path_so_far.clone();
            new_path.push(step);
            stack.push(((px, py), new_path));
        }
    }

    all_paths
}

fn find_all_shortest_paths<T: Eq + Copy + KeyIndex>(
    grid: &[Vec<Option<T>>],
) -> Vec<Vec<Vec<Vec<DirectionalKey>>>> {
    let mut paths = vec![vec![Vec::new(); T::count()]; T::count()];

    for (y1, row1) in grid.iter().enumerate() {
        for (x1, key1) in row1.iter().enumerate() {
            if let Some(key1) = key1 {
                let key1_idx = key1.as_index();
                for (y2, row2) in grid.iter().enumerate() {
                    for (x2, key2) in row2.iter().enumerate() {
                        if let Some(key2) = key2 {
                            let key2_idx = key2.as_index();
                            if key1 == key2 {
                                paths[key1_idx][key2_idx] = vec![vec![DirectionalKey::A]];
                                continue;
                            }
                            paths[key1_idx][key2_idx] =
                                find_shortest_paths_bfs(grid, (x1, y1), (x2, y2));
                        }
                    }
                }
            }
        }
    }

    paths
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct NumericKeypad {
    paths: Vec<Vec<Vec<Vec<DirectionalKey>>>>,
}

impl NumericKeypad {
    fn new() -> Self {
        let grid = vec![
            vec![None, None, None, None, None],
            vec![
                None,
                Some(NumericalKey::Seven),
                Some(NumericalKey::Eight),
                Some(NumericalKey::Nine),
                None,
            ],
            vec![
                None,
                Some(NumericalKey::Four),
                Some(NumericalKey::Five),
                Some(NumericalKey::Six),
                None,
            ],
            vec![
                None,
                Some(NumericalKey::One),
                Some(NumericalKey::Two),
                Some(NumericalKey::Three),
                None,
            ],
            vec![
                None,
                None,
                Some(NumericalKey::Zero),
                Some(NumericalKey::A),
                None,
            ],
            vec![None, None, None, None, None],
        ];
        Self {
            paths: find_all_shortest_paths(&grid),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct DirectionalKeypad {
    paths: Vec<Vec<Vec<Vec<DirectionalKey>>>>,
}

impl DirectionalKeypad {
    fn new() -> Self {
        let grid = vec![
            vec![None, None, None, None, None],
            vec![
                None,
                None,
                Some(DirectionalKey::Up),
                Some(DirectionalKey::A),
                None,
            ],
            vec![
                None,
                Some(DirectionalKey::Left),
                Some(DirectionalKey::Down),
                Some(DirectionalKey::Right),
                None,
            ],
            vec![None, None, None, None, None],
        ];
        Self {
            paths: find_all_shortest_paths(&grid),
        }
    }
}

fn code_to_key_sequence(code: &str) -> Vec<Vec<DirectionalKey>> {
    let num_pad = NumericKeypad::new();

    let mut sequences = vec![Vec::new()];

    let mut current_key = NumericalKey::A.as_index();
    for next_key in code.chars() {
        let next_key = NumericalKey::from_char(next_key).as_index();
        let mut extended_sequences = Vec::new();

        let paths = &num_pad.paths[current_key][next_key];
        for seq in sequences {
            for path in paths {
                let mut new_seq = seq.clone();
                new_seq.extend(path.iter().copied());
                extended_sequences.push(new_seq);
            }
        }

        sequences = extended_sequences;
        current_key = next_key;
    }

    sequences
}

fn get_indirection_costs(indirections: usize) -> Vec<Vec<usize>> {
    let dir_pad = DirectionalKeypad::new();
    let extended_paths: Vec<Vec<Vec<Vec<DirectionalKey>>>> = dir_pad
        .paths
        .iter()
        .map(|to| {
            to.iter()
                .map(|paths| {
                    paths
                        .iter()
                        .map(|path| {
                            std::iter::once(DirectionalKey::A)
                                .chain(path.iter().copied())
                                .collect()
                        })
                        .collect()
                })
                .collect()
        })
        .collect();

    let mut costs = vec![vec![1; DirectionalKey::count()]; DirectionalKey::count()];

    for _ in 0..indirections {
        let mut new_costs =
            vec![vec![usize::MAX; DirectionalKey::count()]; DirectionalKey::count()];

        for from in 0..DirectionalKey::count() {
            for to in 0..DirectionalKey::count() {
                let paths = &extended_paths[from][to];
                new_costs[from][to] = paths
                    .iter()
                    .map(|path| {
                        path.iter()
                            .zip(path.iter().skip(1))
                            .map(|(&from, &to)| costs[from.as_index()][to.as_index()])
                            .sum::<usize>()
                    })
                    .min()
                    .unwrap();
            }
        }

        costs = new_costs;
    }

    costs
}

fn get_shortest_button_sequence(code: &str, indirections: usize) -> usize {
    let sequences = code_to_key_sequence(code);
    let indirection_costs = get_indirection_costs(indirections);

    sequences
        .iter()
        .map(|seq| {
            let mut current_key = DirectionalKey::A.as_index();
            let mut cost = 0;
            for next_key in seq {
                let next_key = next_key.as_index();
                cost += indirection_costs[current_key][next_key];
                current_key = next_key;
            }
            cost
        })
        .min()
        .unwrap()
}

fn get_complexity_sum(codes: &ProcessedInput, indirections: usize) -> usize {
    codes
        .iter()
        .map(|code| {
            let shortest = get_shortest_button_sequence(code, indirections);
            let num_part = code[0..code.len() - 1].parse::<usize>().unwrap();
            shortest * num_part
        })
        .sum()
}

fn part1(codes: &ProcessedInput) -> usize {
    get_complexity_sum(codes, 2)
}

fn part2(codes: &ProcessedInput) -> usize {
    get_complexity_sum(codes, 25)
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
        assert_eq!(part1(&data), 126_384);
    }
}
