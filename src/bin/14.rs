use std::collections::HashSet;
use regex::Regex;

#[derive(Hash, PartialEq, Eq)]
struct Robot {
    p: (i32, i32),
    v: (i32, i32),
}

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-*\d+),(-*\d+)").unwrap();

    let mut robots = vec![];

    for chunk in input.split('\n') {
        for cap in re.captures_iter(chunk) {
            let p0: i32 = cap[1].parse().unwrap();
            let p1: i32 = cap[2].parse().unwrap();
            let v0: i32 = cap[3].parse().unwrap();
            let v1: i32 = cap[4].parse().unwrap();
            robots.push(Robot {
                p: (p0, p1),
                v: (v0, v1),
            });
        }
    }

    let width = 101;
    let height = 103;
    for robot in robots.iter_mut() {
        robot.p = (
            (robot.p.0 + robot.v.0 * 100).rem_euclid(width),
            (robot.p.1 + robot.v.1 * 100).rem_euclid(height),
        );
    }

    let mut q1 = 0;
    let mut q2 = 0;
    let mut q3 = 0;
    let mut q4 = 0;
    for (j, i) in robots.iter().map(|r| r.p) {
        if i == height / 2 || j == width / 2 {
            continue;
        }
        if i < height / 2 && j < width / 2 {
            q1 += 1;
        } else if i > height / 2 && j < width / 2 {
            q3 += 1;
        } else if i < height / 2 && j > width / 2 {
            q2 += 1;
        } else if i > height / 2 && j > width / 2 {
            q4 += 1;
        }
    }
    Some(q1 * q2 * q3 * q4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-*\d+),(-*\d+)").unwrap();

    let mut robots = vec![];

    for chunk in input.split('\n') {
        for cap in re.captures_iter(chunk) {
            let p0: i32 = cap[1].parse().unwrap();
            let p1: i32 = cap[2].parse().unwrap();
            let v0: i32 = cap[3].parse().unwrap();
            let v1: i32 = cap[4].parse().unwrap();
            robots.push(Robot {
                p: (p0, p1),
                v: (v0, v1),
            });
        }
    }

    let mut s = 0u32;
    let width = 101;
    let height = 103;
    loop {
        for robot in robots.iter_mut() {
            robot.p = (
                (robot.p.0 + robot.v.0).rem_euclid(width),
                (robot.p.1 + robot.v.1).rem_euclid(height),
            );
        }
        // Another way that could have been used:
        // let sum_distance = robots.iter().cartesian_product(&robots).map(|(r1, r2)| (r2.p.0 - r1.p.0).pow(2) + (r2.p.1 - r1.p.1).pow(2)).fold(0, |acc, x| acc + x);
        // println!("{} {}", s, sum_distance);

        let set: HashSet<(i32, i32)> = HashSet::from_iter(robots.iter().map(|r| r.p));

        if set.len() == robots.len() {
            break;
        }

        s += 1;
    }

    Some(s + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
