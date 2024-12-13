use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let stones: Vec<i64> = input
        .split(' ')
        .map(|e| e.parse::<i64>().unwrap())
        .collect();
    let mut prev = stones.clone();

    for _ in 0..25 {
        let mut tmp = prev.clone();
        let mut di = 0;
        for i in 0..prev.len() {
            if prev[i] == 0 {
                tmp[i + di] = 1;
            } else if prev[i].to_string().len() % 2 == 0 {
                let m = prev[i].to_string();
                let (l, r) = m.split_at(m.len() / 2);
                tmp[i + di] = l.parse().unwrap();
                tmp.insert(di + i + 1, r.parse().unwrap());
                di += 1;
            } else {
                tmp[i + di] = prev[i] * 2024;
            }
        }
        prev = tmp;
    }

    println!("{:?}", prev.len());

    Some(prev.len() as u32)
}

fn dfs(val: i64, n: i64, cache: &mut HashMap<(i64, i64), i64>) -> i64 {
    let mut ret = 0;
    let val_str = val.to_string();

    if cache.contains_key(&(val, n)) {
        return cache[&(val, n)];
    }
    if n == 0 {
        return 1;
    } else if val == 0 {
        ret = dfs(1, n - 1, cache);
    } else if val_str.len() % 2 == 0 {
        let (l, r) = val_str.split_at(val_str.len() / 2);
        ret = dfs(l.parse().unwrap(), n - 1, cache) + dfs(r.parse().unwrap(), n - 1, cache);
    } else {
        ret = dfs(val * 2024, n - 1, cache);
    }
    cache.insert((val, n), ret);
    return ret;
}

pub fn part_two(input: &str) -> Option<i64> {
    let stones: Vec<i64> = input
        .split(' ')
        .map(|e| e.parse::<i64>().unwrap())
        .collect();

    let mut total = 0;
    let mut cache = HashMap::new();
    for stone in stones {
        total += dfs(stone, 75, &mut cache);
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
