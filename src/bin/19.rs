advent_of_code::solution!(19);

fn dp(design: &str, patterns: &Vec<&str>) -> u64 {
    let mut dp = vec![0; design.len() + 1];
    dp[0] = 1;
    for i in 0..dp.len() {
        if dp[i] == 0 {
            continue;
        }
        for pattern in patterns {
            if design[i..].starts_with(pattern) {
                dp[i + pattern.len()] += dp[i];
            }
        }
    }
    dp[dp.len() - 1]
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_iter = input.split('\n');
    let patterns: Vec<&str> = input_iter.next().unwrap().split(", ").collect();
    let designs: Vec<&str> = input_iter.skip(1).collect();
    let mut total = 0;
    for design in designs {
        if dp(design, &patterns) >= 1 {
            total += 1;
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input_iter = input.split('\n');
    let patterns: Vec<&str> = input_iter.next().unwrap().split(", ").collect();
    let designs: Vec<&str> = input_iter.skip(1).collect();
    let mut total = 0;
    for design in designs {
        total += dp(design, &patterns);
    }

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(16));
    }
}
