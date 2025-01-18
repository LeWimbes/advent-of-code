const INPUT: (u32, u32, &str) = aoc_macros::include_input!(2024, 6);

type ParsedInput = Vec<Vec<char>>;
type ProcessedInput = (Vec<Vec<Field>>, Agent);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Field {
    field_type: FieldType,
    visited: [bool; 4],
}

impl Field {
    fn new(field_type: FieldType) -> Field {
        Self {
            field_type,
            visited: [false; 4],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FieldType {
    Border,
    Floor,
    Object,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Agent {
    x: usize,
    y: usize,
    direction: Direction,
}

enum AgentError {
    AlreadyOnBorder,
    EncounteredObject,
    SteppedOnBorder,
    EncounteredLoop,
}

impl Agent {
    fn new(x: usize, y: usize, direction: Direction) -> Self {
        Self { x, y, direction }
    }

    fn rotate(&mut self) {
        match self.direction {
            Direction::Up => self.direction = Direction::Right,
            Direction::Right => self.direction = Direction::Down,
            Direction::Down => self.direction = Direction::Left,
            Direction::Left => self.direction = Direction::Up,
        }
    }

    fn step(&mut self, map: &mut [Vec<Field>]) -> Result<(), AgentError> {
        if map[self.y][self.x].field_type == FieldType::Border {
            return Err(AgentError::AlreadyOnBorder);
        }

        let (next_x, next_y) = match self.direction {
            Direction::Up => (self.x, self.y - 1),
            Direction::Right => (self.x + 1, self.y),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
        };

        if map[next_y][next_x].field_type == FieldType::Object {
            return Err(AgentError::EncounteredObject);
        }

        self.x = next_x;
        self.y = next_y;

        if map[self.y][self.x].visited[self.direction as usize] {
            Err(AgentError::EncounteredLoop)
        } else if map[next_y][next_x].field_type == FieldType::Border {
            Err(AgentError::SteppedOnBorder)
        } else {
            map[self.y][self.x].visited[self.direction as usize] = true;
            Ok(())
        }
    }
}

fn main() {
    let data = process_input(INPUT.2);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &'static str) -> ParsedInput {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn process_input(input: &'static str) -> ProcessedInput {
    let map = parse_input(input);

    let rows = map.len();
    let cols = if rows > 0 { map[0].len() } else { 0 };

    let new_rows = rows + 2;
    let new_cols = cols + 2;

    let mut new_map = vec![vec![Field::new(FieldType::Border); new_cols]; new_rows];
    let mut agent = Agent::new(0, 0, Direction::Up);

    for y in 0..rows {
        for x in 0..cols {
            match map[y][x] {
                '.' => new_map[y + 1][x + 1] = Field::new(FieldType::Floor),
                '#' => new_map[y + 1][x + 1] = Field::new(FieldType::Object),
                '^' => {
                    new_map[y + 1][x + 1] = Field::new(FieldType::Floor);
                    new_map[y + 1][x + 1].visited[Direction::Up as usize] = true;
                    agent = Agent::new(x + 1, y + 1, Direction::Up);
                }
                '>' => {
                    new_map[y + 1][x + 1] = Field::new(FieldType::Floor);
                    new_map[y + 1][x + 1].visited[Direction::Right as usize] = true;
                    agent = Agent::new(x + 1, y + 1, Direction::Right);
                }
                'v' => {
                    new_map[y + 1][x + 1] = Field::new(FieldType::Floor);
                    new_map[y + 1][x + 1].visited[Direction::Down as usize] = true;
                    agent = Agent::new(x + 1, y + 1, Direction::Down);
                }
                '<' => {
                    new_map[y + 1][x + 1] = Field::new(FieldType::Floor);
                    new_map[y + 1][x + 1].visited[Direction::Left as usize] = true;
                    agent = Agent::new(x + 1, y + 1, Direction::Left);
                }
                _ => panic!("Invalid character in input"),
            }
        }
    }

    (new_map, agent)
}

fn part1((map, agent): &ProcessedInput) -> usize {
    let mut map = map.clone();
    let mut agent = *agent;

    loop {
        match agent.step(&mut map) {
            Ok(()) => {}
            Err(AgentError::AlreadyOnBorder) => panic!("Shouldn't be standing on border"),
            Err(AgentError::EncounteredObject) => agent.rotate(),
            Err(AgentError::SteppedOnBorder) => break,
            Err(AgentError::EncounteredLoop) => panic!("Shouldn't encounter loop"),
        }
    }

    map.iter()
        .flatten()
        .filter(|field| field.visited.iter().any(|&visited| visited))
        .count()
}

fn part2((map, agent): &ProcessedInput) -> usize {
    let mut loops = 0;

    for y in 1..map.len() - 1 {
        for x in 1..map[0].len() - 1 {
            let mut map = map.clone();
            let mut agent = *agent;

            map[y][x] = Field::new(FieldType::Object);

            loop {
                match agent.step(&mut map) {
                    Ok(()) => {}
                    Err(AgentError::AlreadyOnBorder) => panic!("Shouldn't be standing on border"),
                    Err(AgentError::EncounteredObject) => agent.rotate(),
                    Err(AgentError::SteppedOnBorder) => break,
                    Err(AgentError::EncounteredLoop) => {
                        loops += 1;
                        break;
                    }
                }
            }
        }
    }

    loops
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
        assert_eq!(part1(&data), 41);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2(&data), 6);
    }
}
