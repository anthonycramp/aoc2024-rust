const DAY_NUMBER: &str = "05";
const INPUT: &str = include_str!("../../inputs/day05.txt");
// const INPUT: &str = "";

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let mut rules_and_print_orders = input.split("\n\n");
    let rules = rules_and_print_orders
        .next()
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    let print_orders = rules_and_print_orders
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for print_order in &print_orders {
        if is_valid_print_order(print_order, &rules) {
            let mid_number = print_order[print_order.len() / 2].parse::<i32>().unwrap();

            sum += mid_number;
        }
    }

    sum
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    let mut rules_and_print_orders = input.split("\n\n");
    let rules = rules_and_print_orders
        .next()
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    let print_orders = rules_and_print_orders
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(",").collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut sum = 0;
    for print_order in &print_orders {
        if !is_valid_print_order(print_order, &rules) {
            let mut invalid_print_order = print_order.clone();

            'outer: while !is_valid_print_order(&invalid_print_order, &rules) {
                for i in 0..(invalid_print_order.len() - 1) {
                    for j in i..invalid_print_order.len() {
                        let broken_rule =
                            format!("{}|{}", invalid_print_order[j], invalid_print_order[i]);
                        let rule_broken = rules.contains(&broken_rule.as_str());
                        if rule_broken {
                            let tmp = invalid_print_order[i];
                            invalid_print_order[i] = invalid_print_order[j];
                            invalid_print_order[j] = tmp;
                            continue 'outer;
                        }
                    }
                }
            }

            let mid_number = invalid_print_order[invalid_print_order.len() / 2]
                .parse::<i32>()
                .unwrap();

            sum += mid_number;
        }
    }

    sum
}

fn is_valid_print_order(print_order: &Vec<&str>, rules: &Vec<&str>) -> bool {
    let mut valid_print_order = true;
    for i in 0..(print_order.len() - 1) {
        for j in i..print_order.len() {
            let broken_rule = format!("{}|{}", print_order[j], print_order[i]);
            let rule_broken = rules.contains(&broken_rule.as_str());
            valid_print_order &= !rule_broken;
        }
    }
    valid_print_order
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 143,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 123,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
