use aoc_utils::Solution;

struct Day04;

impl Solution for Day04 {
    type Input<'a> = Vec<Vec<char>>;
    type Output1 = u32;
    type Output2 = u32;

    fn process(input: &str) -> Self::Input<'_> {
        let matrix: Vec<Vec<_>> = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect())
            .collect();

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

    fn part1(matrix: &Self::Input<'_>) -> Self::Output1 {
        let mut xmas = 0;

        for row in 3..matrix.len() - 3 {
            for col in 3..matrix[row].len() - 3 {
                xmas += xmas_starts(matrix, row, col);
            }
        }

        xmas
    }

    fn part2(matrix: &Self::Input<'_>) -> Self::Output2 {
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
}

aoc_utils::run!(2024, 4, Day04);

fn xmas_starts(matrix: &<Day04 as Solution>::Input<'_>, row: usize, col: usize) -> u32 {
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
        for (k, c) in word.iter().enumerate().skip(1) {
            let new_row = row as isize + dx * k as isize;
            let new_col = col as isize + dy * k as isize;

            if matrix[new_row as usize][new_col as usize] != *c {
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

fn is_x_mas_middle(matrix: &<Day04 as Solution>::Input<'_>, row: usize, col: usize) -> bool {
    matrix[row][col] == 'A'
        && (((matrix[row - 1][col - 1] == 'M' && matrix[row + 1][col + 1] == 'S')
            || (matrix[row - 1][col - 1] == 'S' && matrix[row + 1][col + 1] == 'M'))
            && ((matrix[row - 1][col + 1] == 'M' && matrix[row + 1][col - 1] == 'S')
                || (matrix[row - 1][col + 1] == 'S' && matrix[row + 1][col - 1] == 'M')))
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    aoc_utils::solution_tests!(2024, 4, Day04);

    #[rstest]
    fn part1_test(data1: <Day04 as Solution>::Input<'_>) {
        assert_eq!(18, Day04::part1(&data1));
    }

    #[rstest]
    fn part2_test(data1: <Day04 as Solution>::Input<'_>) {
        assert_eq!(9, Day04::part2(&data1));
    }
}
