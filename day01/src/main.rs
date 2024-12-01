const DAY_NUMBER: &str = "01";
const INPUT: &str = include_str!("../../inputs/day01.txt");

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in input.lines() {
        let mut location_ids = line.split_ascii_whitespace();
        left_list.push(location_ids.next().unwrap().parse::<i32>().unwrap());
        right_list.push(location_ids.next().unwrap().parse::<i32>().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let mut sum = 0;
    for i in 0..left_list.len() {
        sum += (left_list[i] - right_list[i]).abs();
    }

    sum
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in input.lines() {
        let mut location_ids = line.split_ascii_whitespace();
        left_list.push(location_ids.next().unwrap().parse::<i32>().unwrap());
        right_list.push(location_ids.next().unwrap().parse::<i32>().unwrap());
    }

    let mut sum = 0;
    for v in left_list {
        sum += right_list.iter().filter(|s| **s == v).count() as i32 * v;
    }

    sum
}

#[cfg(test)]
mod tests {
    // const TEST_INPUT: &str = include_str!("dayNN_test.txt");
    const TEST_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 11,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 31,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
