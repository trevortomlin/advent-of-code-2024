use std::{
    cmp::{max, min},
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != '.' {
                antennas
                    .entry(grid[row][col])
                    .or_insert_with(|| vec![])
                    .push((row as i32, col as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for frequency in antennas.values() {
        for (&p1, &p2) in frequency.iter().cartesian_product(frequency) {
            if p1 == p2 {
                continue;
            }
            let dx = p2.0 - p1.0;
            let dy = p2.1 - p1.1;
            let new_x = p2.0 + dx;
            let new_y = p2.1 + dy;
            if new_x < 0 || new_x >= grid.len() as i32 || new_y < 0 || new_y >= grid[0].len() as i32
            {
                continue;
            }
            antinodes.insert((new_x, new_y));
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] != '.' {
                antennas
                    .entry(grid[row][col])
                    .or_insert_with(|| vec![])
                    .push((row as i32, col as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for frequency in antennas.values() {
        for (&p1, &p2) in frequency.iter().cartesian_product(frequency) {
            if p1 == p2 {
                continue;
            }
            let dx = p2.0 - p1.0;
            let dy = p2.1 - p1.1;
            let mut antinode = p2;
            antinodes.insert(antinode);
            loop {
                antinode = (antinode.0 + dx, antinode.1 + dy);
                if antinode.0 < 0
                    || antinode.0 >= grid.len() as i32
                    || antinode.1 < 0
                    || antinode.1 >= grid[0].len() as i32
                {
                    break;
                }
                antinodes.insert(antinode);
            }
        }
    }
    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
