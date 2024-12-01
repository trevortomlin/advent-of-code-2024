advent_of_code::solution!(1);

use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let mut heap1: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let mut heap2: BinaryHeap<Reverse<i32>>  = BinaryHeap::new();

    for (v1, v2) in input.split_whitespace().tuples() {
        let v1: i32 = v1.parse().ok()?;
        let v2: i32 = v2.parse().ok()?;
        heap1.push(Reverse(v1));
        heap2.push(Reverse(v2));
    }

    let mut total = 0;

    while !heap1.is_empty() && !heap2.is_empty() {
        let l1 = heap1.pop()?.0;
        let l2 = heap2.pop()?.0;
        total += (l2-l1).abs();
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut left: Vec<i32> = Vec::new();
    let mut right = HashMap::new();

    for (v1, v2) in input.split_whitespace().tuples() {
        let v1: i32 = v1.parse().ok()?;
        let v2: i32 = v2.parse().ok()?;
        left.push(v1);
        right
            .entry(v2)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut total = 0;

    for v in left {
        total += v * right.get(&v).unwrap_or(&0);
    }

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
