use itertools::Itertools;
use std::collections::{HashMap, HashSet};

const DAY_NUMBER: &str = "08";
const INPUT: &str = include_str!("../../inputs/day08.txt");

fn main() {
    println!("Day {} Part 1: {:?}", DAY_NUMBER, part1(INPUT));
    println!("Day {} Part 2: {:?}", DAY_NUMBER, part2(INPUT));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Location {
    row: i32,
    col: i32,
}

impl Location {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
}

fn compute_antinodes(antenna_locations: &Vec<&Location>) -> Vec<Location> {
    let location_one = antenna_locations[0];
    let location_two = antenna_locations[1];
    let row_delta = location_one.row - location_two.row;
    let col_delta = location_one.col - location_two.col;

    let antinode_one = Location::new(
        location_one.row - 2 * row_delta,
        location_one.col - 2 * col_delta,
    );
    let antinode_two = Location::new(
        location_two.row + 2 * row_delta,
        location_two.col + 2 * col_delta,
    );
    vec![antinode_one, antinode_two]
}

fn is_location_in_map(location: &Location, number_of_rows: usize, number_of_cols: usize) -> bool {
    location.row >= 0
        && location.row < number_of_rows as i32
        && location.col >= 0
        && location.col < number_of_cols as i32
}

// replace return type as required by the problem
fn part1(input: &str) -> i32 {
    // compile a map of frequencies to all antenna locations at that frequency
    let antenna_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let number_of_rows = antenna_map.len();
    let number_of_cols = antenna_map[0].len();

    // for each frequency, take antenna locations pairwise and compute antinodes
    let mut antenna_locations_by_frequency: HashMap<char, Vec<Location>> = HashMap::new();
    for (row_index, row) in antenna_map.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            if *cell != '.' {
                (*antenna_locations_by_frequency
                    .entry(*cell)
                    .or_insert(vec![]))
                .push(Location::new(row_index as i32, col_index as i32));
            }
        }
    }

    let mut antinode_locations_by_frequency: HashMap<char, HashSet<Location>> = HashMap::new();

    for (frequency, antenna_locations) in antenna_locations_by_frequency {
        for antenna_pair in antenna_locations.iter().permutations(2) {
            let antinode_locations = compute_antinodes(&antenna_pair);
            // for antinodes within the map bounds, record their location and associated freq
            for antinode_location in antinode_locations.iter() {
                if is_location_in_map(antinode_location, number_of_rows, number_of_cols) {
                    (*antinode_locations_by_frequency
                        .entry(frequency)
                        .or_insert(HashSet::new()))
                    .insert(*antinode_location);
                }
            }
        }
    }

    // return the number of unique antinode locations
    let mut all_unique_antinode_locations: HashSet<Location> = HashSet::new();
    for (_, antinode_locations) in antinode_locations_by_frequency {
        for location in antinode_locations {
            all_unique_antinode_locations.insert(location.clone());
        }
    }

    all_unique_antinode_locations.len() as i32
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
    use super::*;
    use test_support::test_support::TestCase;

    #[test]
    fn test_part1() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 14,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part1(*input), *expected);
        }
    }

    #[test]
    fn test_part2() {
        let test_cases = [TestCase {
            input: TEST_INPUT,
            expected: 34,
        }];
        for TestCase { input, expected } in test_cases.iter() {
            assert_eq!(part2(*input), *expected);
        }
    }
}
