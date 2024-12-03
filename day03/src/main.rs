use regex::Regex;

const DAY_NUMBER: &str = "03";
const INPUT: &str = include_str!("../../inputs/day03.txt");
// const INPUT: &str = "";

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mul_iter = re.captures_iter(input);

    let mut sum = 0;
    for (_, [left, right]) in mul_iter.map(|c| c.extract()) {
        sum += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
    }
    sum
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let re = Regex::new(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)").unwrap();
    let re_mul = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mul_iter = re.find_iter(input);

    let mut sum = 0;
    let mut mul_enabled = true;
    for a in mul_iter {
        let mat_str = a.as_str();
        match mat_str {
            "do()" => mul_enabled = true,
            "don't()" => mul_enabled = false,
            _ => {
                if mul_enabled {
                    let (_, [left, right]) = re_mul.captures(mat_str).unwrap().extract();
                    sum += left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap();
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const TEST_INPUT_PART_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 161,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT_PART_2,
            expected: 48,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
