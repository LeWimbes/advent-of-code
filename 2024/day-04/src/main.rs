type ParsedInput = Vec<Vec<char>>;
type ProcessedInput = Vec<Vec<char>>;

fn main() {
    let input = include_str!("input.txt");

    let data = process_input(input);
    println!("Part1: {}", part1(&data));
    println!("Part2: {}", part2(&data));
}

fn parse_input(input: &str) -> ParsedInput {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn process_input(input: &str) -> ProcessedInput {
    let matrix = parse_input(input);

    let rows = matrix.len();
    let cols = if rows > 0 { matrix[0].len() } else { 0 };

    let new_rows = rows + 6;
    let new_cols = cols + 6;

    let mut new_matrix = vec![vec!['.'; new_cols]; new_rows];

    for i in 0..rows {
        for j in 0..cols {
            new_matrix[i + 3][j + 3] = matrix[i][j];
        }
    }

    new_matrix
}

fn xmas_starts(matrix: &ProcessedInput, row: usize, col: usize) -> u32 {
    if matrix[row][col] != 'X' {
        return 0;
    }

    let mut xmas = 0;
    let word = ['X', 'M', 'A', 'S'];
    let directions = [
        (-1, 0),  // Up
        (-1, 1),  // Up-Right
        (0, 1),   // Right
        (1, 1),   // Down-Right
        (1, 0),   // Down
        (1, -1),  // Down-Left
        (0, -1),  // Left
        (-1, -1), // Up-Left
    ];

    for &(dx, dy) in &directions {
        let mut found = true;
        for k in 1..4 {
            let new_row = row as isize + dx * k as isize;
            let new_col = col as isize + dy * k as isize;

            if matrix[new_row as usize][new_col as usize] != word[k] {
                found = false;
                break;
            }
        }
        if found {
            xmas += 1;
        }
    }
    xmas
}

fn part1(matrix: &ProcessedInput) -> u32 {
    let mut xmas = 0;

    for row in 3..matrix.len() - 3 {
        for col in 3..matrix[row].len() - 3 {
            xmas += xmas_starts(matrix, row, col);
        }
    }

    xmas
}

fn is_x_mas_middle(matrix: &ProcessedInput, row: usize, col: usize) -> bool {
    matrix[row][col] == 'A'
        && (((matrix[row - 1][col - 1] == 'M' && matrix[row + 1][col + 1] == 'S')
            || (matrix[row - 1][col - 1] == 'S' && matrix[row + 1][col + 1] == 'M'))
            && ((matrix[row - 1][col + 1] == 'M' && matrix[row + 1][col - 1] == 'S')
                || (matrix[row - 1][col + 1] == 'S' && matrix[row + 1][col - 1] == 'M')))
}

fn part2(matrix: &ProcessedInput) -> u32 {
    let mut x_mas = 0;

    for row in 4..matrix.len() - 4 {
        for col in 4..matrix[row].len() - 4 {
            if is_x_mas_middle(matrix, row, col) {
                x_mas += 1;
            }
        }
    }

    x_mas
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
        assert_eq!(part1(&data), 18);
    }

    #[rstest]
    fn part2_test(data: ProcessedInput) {
        assert_eq!(part2(&data), 9);
    }
}
