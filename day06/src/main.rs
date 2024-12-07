use std::collections::HashSet;

const DAY_NUMBER: &str = "06";
const INPUT: &str = include_str!("../../inputs/day06.txt");

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash)]
enum Direction {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    row: i32,
    col: i32,
}

impl Default for Location {
    fn default() -> Self {
        Self {
            row: Default::default(),
            col: Default::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct GuardState {
    location: Location,
    direction: Direction,
}

impl Default for GuardState {
    fn default() -> Self {
        Self {
            location: Default::default(),
            direction: Direction::NORTH,
        }
    }
}

impl GuardState {
    fn new(row: i32, col: i32) -> Self {
        GuardState {
            location: Location { row, col },
            direction: Direction::NORTH,
        }
    }

    fn walk(&mut self) {
        self.location = self.peek();
    }

    fn peek(&self) -> Location {
        match self.direction {
            Direction::NORTH => Location {
                row: self.location.row - 1,
                col: self.location.col,
            },
            Direction::EAST => Location {
                row: self.location.row,
                col: self.location.col + 1,
            },
            Direction::SOUTH => Location {
                row: self.location.row + 1,
                col: self.location.col,
            },
            Direction::WEST => Location {
                row: self.location.row,
                col: self.location.col - 1,
            },
        }
    }

    fn turn(&mut self) {
        match self.direction {
            Direction::NORTH => self.direction = Direction::EAST,
            Direction::EAST => self.direction = Direction::SOUTH,
            Direction::SOUTH => self.direction = Direction::WEST,
            Direction::WEST => self.direction = Direction::NORTH,
        }
    }
}

#[derive(Clone)]
struct Map {
    map: Vec<Vec<char>>,
    map_width: usize,
    map_height: usize,
    guard: GuardState,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let mut guard = GuardState::default();

        for (i, map_row) in map.iter().enumerate() {
            for (j, map_cell) in map_row.iter().enumerate() {
                if *map_cell == '^' {
                    guard = GuardState::new(i as i32, j as i32);
                }
            }
        }

        Self {
            map: map.clone(),
            map_width: map[0].len(),
            map_height: map.len(),
            guard: guard,
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

    fn guard_walk(&mut self) -> Option<HashSet<GuardState>> {
        let mut guards_path = HashSet::new();

        loop {
            self.map[self.guard.location.row as usize][self.guard.location.col as usize] = 'X';
            let new_guard_position = self.guard.peek();

            if new_guard_position.row < 0
                || new_guard_position.row >= self.map_width as i32
                || new_guard_position.col < 0
                || new_guard_position.col >= self.map_height as i32
            {
                return Some(guards_path.clone());
            }

            let next_location =
                self.map[new_guard_position.row as usize][new_guard_position.col as usize];

            if next_location == '#' {
                self.guard.turn();
            } else {
                self.guard.walk();
                if guards_path.contains(&self.guard) {
                    // cycle detected
                    return None;
                }
                guards_path.insert(self.guard.clone());
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
    let mut original_map = Map::from(input);
    let mut map_for_guard_to_walk = original_map.clone();
    let guard_path = map_for_guard_to_walk.guard_walk();
    let mut number_of_obstacles_causing_loop = 0;

    if let Some(guard_states) = guard_path {
        let unique_locations = guard_states
            .iter()
            .map(|gs| gs.location.clone())
            .collect::<HashSet<_>>();
        for location in unique_locations {
            let mut obstacled_map = original_map.clone();
            obstacled_map.map[location.row as usize][location.col as usize] = '#';
            if obstacled_map.guard_walk().is_none() {
                number_of_obstacles_causing_loop += 1;
            }
        }
    }

    number_of_obstacles_causing_loop
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
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 6,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
