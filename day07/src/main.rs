use core::num;

const DAY_NUMBER: &str = "07";
const INPUT: &str = include_str!("../../inputs/day07.txt");

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| inspect_equation(line, try_equation))
        .filter(|res| res.is_some())
        .map(|res| res.unwrap())
        .sum::<i64>()
}

fn try_equation(operator_map: i64, target: i64, operands: &Vec<i64>) -> bool {
    let binary_operator_string = format!("{:b}", operator_map);
    let operator_string =
        "0".repeat(operands.len() - 1 - binary_operator_string.len()) + &binary_operator_string;

    let mut accumulator = None;
    for (i, operator) in operator_string.chars().enumerate() {
        let left_operand = if accumulator.is_none() {
            operands[i]
        } else {
            accumulator.unwrap()
        };
        let right_operand = operands[i + 1];

        match operator {
            '0' => accumulator = Some(left_operand + right_operand),
            '1' => accumulator = Some(left_operand * right_operand),
            _ => (),
        }
    }

    accumulator.unwrap() == target
}

fn inspect_equation(
    equation: &str,
    try_equation_fn: fn(i64, i64, &Vec<i64>) -> bool,
) -> Option<i64> {
    println!("{}", equation);
    let mut equation_parts = equation.split(": ");
    let target_output = equation_parts.next().unwrap().parse::<i64>().unwrap();
    println!("{}", target_output);
    let operands = equation_parts
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .map(|val| val.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    println!("{:?}", &operands);

    let num_operands = operands.len() as i64;
    println!("Number of operands: {}", num_operands);
    let num_operators = i64::pow(2, (num_operands - 1) as u32);
    println!("Number of operators: {}", num_operators);

    for i in 0..num_operators {
        if try_equation_fn(i, target_output, &operands) {
            println!("Success: {}", target_output);
            return Some(target_output);
        }
    }

    None
}

// replace return type as required by the problem
fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 3749,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 11387,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
