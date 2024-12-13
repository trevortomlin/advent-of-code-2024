use regex::Regex;

advent_of_code::solution!(13);

fn intersect(a1: i64, b1: i64, c1: i64, a2: i64, b2: i64, c2: i64) -> (f64, f64) {
    let x = -(b1 * c2 - b2 * c1) as f64 / (a1 * b2 - a2 * b1) as f64;
    let y = -(a2 * c1 - a1 * c2) as f64 / (a1 * b2 - a2 * b1) as f64;
    return (x, y);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    for chunk in input.split("\n\n") {
        let re = Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .unwrap();

        for cap in re.captures_iter(chunk) {
            let a1: i64 = cap[1].parse().unwrap();
            let a2: i64 = cap[2].parse().unwrap();
            let b1: i64 = cap[3].parse().unwrap();
            let b2: i64 = cap[4].parse().unwrap();
            let c1: i64 = cap[5].parse().unwrap();
            let c2: i64 = cap[6].parse().unwrap();
            let l = intersect(a1, b1, c1, a2, b2, c2);
            if l.0 <= 100.0 && l.1 <= 100.0 && l.0.fract() == 0.0 && l.1.fract() == 0.0 {
                total += 3 * l.0 as u32 + l.1 as u32;
            }
        }
    }
    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0u64;
    for chunk in input.split("\n\n") {
        let re = Regex::new(
            r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
        )
        .unwrap();

        for cap in re.captures_iter(chunk) {
            let a1: i64 = cap[1].parse().unwrap();
            let a2: i64 = cap[2].parse().unwrap();
            let b1: i64 = cap[3].parse().unwrap();
            let b2: i64 = cap[4].parse().unwrap();
            let c1: i64 = cap[5].parse::<i64>().unwrap() + 10000000000000;
            let c2: i64 = cap[6].parse::<i64>().unwrap() + 10000000000000;
            let l = intersect(a1, b1, c1, a2, b2, c2);
            if l.0.fract() == 0.0 && l.1.fract() == 0.0 {
                total += 3 * l.0 as u64 + l.1 as u64;
            }
        }
    }
    Some(total as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(459236326669));
    }
}
