use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let reports: Vec<Vec<u32>> = input
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|e| {
                    e.parse::<u32>()
                        .expect("Report should contain valid numbers")
                })
                .collect()
        })
        .collect();

    let increasing = |v: &Vec<u32>| -> bool {
        for i in 1..v.len() {
            if v[i] <= v[i - 1] {
                return false;
            }
        }
        true
    };

    let decreasing = |v: &Vec<u32>| -> bool {
        for i in 1..v.len() {
            if v[i] >= v[i - 1] {
                return false;
            }
        }
        true
    };

    let is_safe = |vec: &Vec<u32>| -> bool { increasing(vec) || decreasing(vec) };

    let mut total = 0;

    for i in 0..reports.len() {
        let diff = reports[i]
            .iter()
            .tuple_windows()
            .map(|(&e1, &e2)| (e1 as i32 - e2 as i32).abs())
            .filter(|&x| x < 1 || x > 3)
            .count()
            == 0;
        if is_safe(&reports[i]) && diff {
            total += 1;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports: Vec<Vec<u32>> = input
        .split("\n")
        .map(|line| {
            line.split(" ")
                .map(|e| {
                    e.parse::<u32>()
                        .expect("Report should contain valid numbers")
                })
                .collect()
        })
        .collect();

    let increasing = |v: &Vec<u32>| -> bool {
        for i in 1..v.len() {
            if v[i] <= v[i - 1] {
                return false;
            }
        }
        true
    };

    let decreasing = |v: &Vec<u32>| -> bool {
        for i in 1..v.len() {
            if v[i] >= v[i - 1] {
                return false;
            }
        }
        true
    };

    let is_safe = |vec: &Vec<u32>| -> bool { increasing(vec) || decreasing(vec) };

    let mut total = 0;

    for i in 0..reports.len() {
        let diff = reports[i]
            .iter()
            .tuple_windows()
            .map(|(&e1, &e2)| (e1 as i32 - e2 as i32).abs())
            .filter(|&x| x < 1 || x > 3)
            .count()
            == 0;
        if is_safe(&reports[i]) && diff {
            total += 1;
        } else {
            for j in 0..reports[i].len() {
                let new: Vec<u32> = reports[i]
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| *i != j)
                    .map(|(_, &e)| e)
                    .collect();
                let is_safe_now = new
                    .iter()
                    .tuple_windows()
                    .map(|(&e1, &e2)| (e1 as i32 - e2 as i32).abs())
                    .filter(|&x| x < 1 || x > 3)
                    .count()
                    == 0;
                if is_safe(&new.clone()) && is_safe_now {
                    total += 1;
                    break;
                }
            }
        }
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
