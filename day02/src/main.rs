use itertools::Itertools;

const DAY_NUMBER: &str = "02";
const INPUT: &str = include_str!("../../inputs/day02.txt");
//const INPUT: &str = "";

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|e| e.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

fn compute_number_of_safe_reports(
    reports: &Vec<Vec<i32>>,
    report_test_fn: fn(&Vec<i32>) -> bool,
) -> i32 {
    reports
        .iter()
        .map(|report| report_test_fn(report))
        .filter(|safe| *safe)
        .count() as i32
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    compute_number_of_safe_reports(&parse_input(&input), is_report_safe)
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let mut ascending = true;
    let mut descending = true;
    let mut correct_gaps = true;

    for (left, right) in report.iter().tuple_windows() {
        ascending &= left < right;
        descending &= left > right;
        let gap = (left - right).abs();
        correct_gaps &= gap >= 1 && gap <= 3;
    }

    (ascending || descending) && correct_gaps
}

fn is_report_safe_part2(report: &Vec<i32>) -> bool {
    let mut safe = vec![];
    for index in 0..report.len() {
        let mut sub_report = report.clone();
        sub_report.remove(index);
        let is_sub_report_safe = is_report_safe(&sub_report);
        safe.push(is_sub_report_safe);
    }

    safe.contains(&true)
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    compute_number_of_safe_reports(&parse_input(input), is_report_safe_part2)
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 2,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 4,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }

    #[test]
    fn test_is_report_safe() {
        let test_cases = [
            TestCase {
                input: vec![7, 6, 4, 2, 1],
                expected: true,
            },
            TestCase {
                input: vec![1, 2, 7, 8, 9],
                expected: false,
            },
            TestCase {
                input: vec![9, 7, 6, 2, 1],
                expected: false,
            },
            TestCase {
                input: vec![1, 3, 2, 4, 5],
                expected: false,
            },
            TestCase {
                input: vec![8, 6, 4, 4, 1],
                expected: false,
            },
            TestCase {
                input: vec![1, 3, 6, 7, 9],
                expected: true,
            },
        ];
        for (i, TestCase { input, expected }) in test_cases.iter().enumerate() {
            assert_eq!(is_report_safe(input), *expected, "Failed test case {}", i);
        }
    }

    #[test]
    fn test_is_report_safe_part2() {
        let test_cases = [
            TestCase {
                input: vec![7, 6, 4, 2, 1],
                expected: true,
            },
            TestCase {
                input: vec![1, 2, 7, 8, 9],
                expected: false,
            },
            TestCase {
                input: vec![9, 7, 6, 2, 1],
                expected: false,
            },
            TestCase {
                input: vec![1, 3, 2, 4, 5],
                expected: true,
            },
            TestCase {
                input: vec![8, 6, 4, 4, 1],
                expected: true,
            },
            TestCase {
                input: vec![1, 3, 6, 7, 9],
                expected: true,
            },
        ];
        for (i, TestCase { input, expected }) in test_cases.iter().enumerate() {
            assert_eq!(
                is_report_safe_part2(input),
                *expected,
                "Failed test case {}",
                i
            );
        }
    }
}
