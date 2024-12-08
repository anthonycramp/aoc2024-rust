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

struct Bounds {
    rows: i32,
    cols: i32,
}

impl Bounds {
    fn new(rows: i32, cols: i32) -> Self {
        Bounds { rows, cols }
    }

    fn in_bounds(&self, location: &Location) -> bool {
        location.row >= 0
            && location.row < self.rows
            && location.col >= 0
            && location.col < self.cols
    }
}

fn compute_antinodes(antenna_locations: &Vec<&Location>, map_bounds: &Bounds) -> Vec<Location> {
    let location_one = antenna_locations[0];
    let location_two = antenna_locations[1];
    let row_delta = location_one.row - location_two.row;
    let col_delta = location_one.col - location_two.col;

    let mut ret = vec![];

    let antinode_one = Location::new(
        location_one.row - 2 * row_delta,
        location_one.col - 2 * col_delta,
    );
    if map_bounds.in_bounds(&antinode_one) {
        ret.push(antinode_one);
    }

    let antinode_two = Location::new(
        location_two.row + 2 * row_delta,
        location_two.col + 2 * col_delta,
    );
    if map_bounds.in_bounds(&antinode_two) {
        ret.push(antinode_two);
    }

    ret
}

fn compute_antinodes_part_2(
    antenna_locations: &Vec<&Location>,
    map_bounds: &Bounds,
) -> Vec<Location> {
    let location_one = antenna_locations[0];
    let location_two = antenna_locations[1];
    let row_delta = location_one.row - location_two.row;
    let col_delta = location_one.col - location_two.col;

    let mut ret = vec![];
    let mut new_antinode_row = location_one.row;
    let mut new_antinode_col = location_one.col;

    loop {
        new_antinode_row -= row_delta;
        new_antinode_col -= col_delta;

        let new_antinode_location = Location::new(new_antinode_row, new_antinode_col);
        if !map_bounds.in_bounds(&new_antinode_location) {
            break;
        }

        ret.push(new_antinode_location);
    }

    let mut new_antinode_row = location_two.row;
    let mut new_antinode_col = location_two.col;

    loop {
        new_antinode_row += row_delta;
        new_antinode_col += col_delta;

        let new_antinode_location = Location::new(new_antinode_row, new_antinode_col);
        if !map_bounds.in_bounds(&new_antinode_location) {
            break;
        }

        ret.push(new_antinode_location);
    }

    ret
}

fn analyse_map(
    input: &str,
    compute_antinodes_fn: fn(&Vec<&Location>, &Bounds) -> Vec<Location>,
) -> i32 {
    // compile a map of frequencies to all antenna locations at that frequency
    let antenna_map = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let map_bounds = Bounds::new(antenna_map.len() as i32, antenna_map[0].len() as i32);

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
            let antinode_locations = compute_antinodes_fn(&antenna_pair, &map_bounds);
            // for antinodes within the map bounds, record their location and associated freq
            for antinode_location in antinode_locations.iter() {
                (*antinode_locations_by_frequency
                    .entry(frequency)
                    .or_insert(HashSet::new()))
                .insert(*antinode_location);
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
fn part1(input: &str) -> i32 {
    analyse_map(input, compute_antinodes)
}

// replace return type as required by the problem
fn part2(input: &str) -> i32 {
    analyse_map(input, compute_antinodes_part_2)
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
