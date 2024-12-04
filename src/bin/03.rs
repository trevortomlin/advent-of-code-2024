advent_of_code::solution!(3);

use regex::Regex;

pub fn part_one(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let matches = re.find_iter(input);
    let matches_str: Vec<&str> = matches.map(|m| m.as_str()).collect();
    let mut total = 0;
    matches_str.iter().for_each(|&e| {
        let mut new = e.replace("mul(", "");
        new = new.replace(")", "");
        let (first, second) = new.split_once(",").expect("Must have comma");
        total =  total + first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap();
    });

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let matches = re.find_iter(&input);
    let matches_str: Vec<&str> = matches.map(|m| m.as_str()).collect();
    let mut total = 0;
    let mut enabled = true;
    for m in matches_str {
        match m {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => if enabled {total += part_one(m).unwrap()} else {continue},
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
