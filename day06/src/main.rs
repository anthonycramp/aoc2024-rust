const DAY_NUMBER: &str = "06";
const INPUT: &str = include_str!("../../inputs/day06.txt");

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

struct Map {
    map: Vec<Vec<char>>,
    map_width: usize,
    map_height: usize,
    guard_position: (i32, i32),
    guard_direction: (i8, i8),
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut guard_position = (0, 0);

        for (i, map_row) in map.iter().enumerate() {
            for (j, map_cell) in map_row.iter().enumerate() {
                if *map_cell == '^' {
                    guard_position = (i as i32, j as i32);
                }
            }
        }

        Self {
            map: map.clone(),
            map_width: map[0].len(),
            map_height: map.len(),
            guard_position: guard_position,
            guard_direction: (-1, 0),
        }
    }
}

impl Map {
    fn count_visited_positions(&self) -> i32 {
        self.map
            .iter()
            .map(|row| {
                row.iter()
                    .map(|c| if *c == 'X' { 1 } else { 0 })
                    .sum::<i32>()
            })
            .sum::<i32>()
    }

    fn guard_walk(&mut self) {
        loop {
            self.map[self.guard_position.0 as usize][self.guard_position.1 as usize] = 'X';
            let new_guard_position = (
                self.guard_position.0 + self.guard_direction.0 as i32,
                self.guard_position.1 + self.guard_direction.1 as i32,
            );

            if new_guard_position.0 < 0
                || new_guard_position.0 >= self.map_width as i32
                || new_guard_position.1 < 0
                || new_guard_position.1 >= self.map_height as i32
            {
                break;
            }

            let next_location =
                self.map[new_guard_position.0 as usize][new_guard_position.1 as usize];

            if next_location == '#' {
                self.guard_direction = match self.guard_direction {
                    (-1, 0) => (0, 1),
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    _ => (0, 0),
                }
            } else {
                self.guard_position = new_guard_position;
            }
        }
    }
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    let mut map = Map::from(input);
    map.guard_walk();
    map.count_visited_positions()
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 41,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    #[ignore = "not yet implemented"]
    fn test_part2() {
        let test_cases = [
            TestCase {
                input: TEST_INPUT,
                expected: 123,
            },
            TestCase {
                input: "abc",
                expected: 345,
            },
        ];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
