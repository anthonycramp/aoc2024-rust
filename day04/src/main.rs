const DAY_NUMBER: &str = "04";
const INPUT: &str = include_str!("../../inputs/day04.txt");

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    const XMAS: &str = "XMAS";

    let lines = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let number_of_rows = lines.len();
    let number_of_columns = lines[0].len();

    let mut number_of_xmas = 0;

    // count horizontals
    for line in &lines {
        let forward_str = line.iter().collect::<String>();
        number_of_xmas += forward_str.match_indices(XMAS).count();

        let backward_str = line.iter().rev().collect::<String>();
        number_of_xmas += backward_str.match_indices(XMAS).count();
    }

    // count columns
    for column_index in 0..number_of_columns {
        let mut column_chars = vec![];
        for row in &lines {
            column_chars.push(row[column_index]);
        }

        let forward_str = column_chars.iter().collect::<String>();
        number_of_xmas += forward_str.match_indices(XMAS).count();

        let backward_str = column_chars.iter().rev().collect::<String>();
        number_of_xmas += backward_str.match_indices(XMAS).count();
    }

    // 'backslash' diagonals starting from column 0, going down the rows
    for row_index in 0..=(number_of_rows - XMAS.len()) {
        let diagonal_length = usize::min(number_of_rows - row_index, number_of_columns);
        let mut diagonal_chars = vec![];
        for diagonal_index in 0..diagonal_length {
            diagonal_chars.push(lines[row_index + diagonal_index][diagonal_index]);
        }

        let forward_str = diagonal_chars.iter().collect::<String>();
        number_of_xmas += forward_str.match_indices(XMAS).count();

        let backward_str = diagonal_chars.iter().rev().collect::<String>();
        number_of_xmas += backward_str.match_indices(XMAS).count();
    }

    // 'backslash' diagonals starting along the first row, from column 1
    for column_index in 1..=(number_of_columns - XMAS.len()) {
        let diagonal_length = usize::min(number_of_columns - column_index, number_of_rows);
        let mut diagonal_chars = vec![];
        for diagonal_index in 0..diagonal_length {
            diagonal_chars.push(lines[diagonal_index][column_index + diagonal_index]);
        }

        let forward_str = diagonal_chars.iter().collect::<String>();
        number_of_xmas += forward_str.match_indices(XMAS).count();

        let backward_str = diagonal_chars.iter().rev().collect::<String>();
        number_of_xmas += backward_str.match_indices(XMAS).count();
    }

    // 'slash' diagonals starting along the last column, going down the rows
    for row_index in 0..=(number_of_rows - XMAS.len()) {
        let diagonal_length = usize::min(number_of_rows - row_index, number_of_columns);
        let mut diagonal_chars = vec![];
        for diagonal_index in 0..diagonal_length {
            diagonal_chars
                .push(lines[row_index + diagonal_index][number_of_columns - 1 - diagonal_index]);
        }

        let forward_str = diagonal_chars.iter().collect::<String>();
        number_of_xmas += forward_str.match_indices(XMAS).count();

        let backward_str = diagonal_chars.iter().rev().collect::<String>();
        number_of_xmas += backward_str.match_indices(XMAS).count();
    }

    // 'slash' diagonals starting in the first row, from the XMAS.len() column to the second to last column
    for column_index in XMAS.len()..(number_of_columns) {
        let diagonal_length = usize::min(column_index, number_of_rows);
        let mut diagonal_chars = vec![];
        for diagonal_index in 0..diagonal_length {
            diagonal_chars.push(lines[diagonal_index][column_index - diagonal_index - 1]);
        }

        let forward_str = diagonal_chars.iter().collect::<String>();
        number_of_xmas += forward_str.match_indices(XMAS).count();

        let backward_str = diagonal_chars.iter().rev().collect::<String>();
        number_of_xmas += backward_str.match_indices(XMAS).count();
    }

    number_of_xmas as i32
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let input_chars = input
        .lines()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let number_of_rows = input_chars.len();
    let number_of_columns = input_chars[0].len();

    let mut number_of_xmas = 0;

    for row_index in 1..(number_of_rows - 1) {
        for col_index in 1..(number_of_columns - 1) {
            let centre_char = input_chars[row_index][col_index];
            if centre_char != 'A' {
                continue;
            }

            let upper_left_char = input_chars[row_index - 1][col_index - 1];
            let upper_right_char = input_chars[row_index - 1][col_index + 1];
            let lower_left_char = input_chars[row_index + 1][col_index - 1];
            let lower_right_char = input_chars[row_index + 1][col_index + 1];

            let mas_upper_left_bottom_right = upper_left_char == 'M' && lower_right_char == 'S';
            let mas_bottom_right_upper_left = lower_right_char == 'M' && upper_left_char == 'S';
            let mas_upper_right_bottom_left = upper_right_char == 'M' && lower_left_char == 'S';
            let mas_bottom_left_upper_right = lower_left_char == 'M' && upper_right_char == 'S';

            let mas_backslash = mas_upper_left_bottom_right || mas_bottom_right_upper_left;
            let mas_slash = mas_upper_right_bottom_left || mas_bottom_left_upper_right;

            let x_mas = mas_backslash && mas_slash;

            if x_mas {
                number_of_xmas += 1;
            }
        }
    }
    number_of_xmas
}

#[cfg(test)]
mod tests {
    const TEST_INPUT_0: &str = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";

    const TEST_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part0() {
        let test_cases = [TestCase {
            input: TEST_INPUT_0,
            expected: 4,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 18,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 9,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
