advent_of_code::solution!(7);

struct Row {
    result: u64,
    numbers: Vec<u64>,
}

fn dfs(row: &Row, index: usize, total: u64) -> bool {
    if total == row.result {
        return true;
    }
    if index >= row.numbers.len() {
        return false;
    }
    let l = dfs(row, index + 1, total * row.numbers[index]);
    let r = dfs(row, index + 1, total + row.numbers[index]);
    l || r
}

fn dfs2(row: &Row, index: usize, total: u64) -> bool {
    if index >= row.numbers.len() {
        if total == row.result {
            return true;
        }
        return false;
    }
    let l = dfs2(row, index + 1, total * row.numbers[index]);
    let r = dfs2(row, index + 1, total + row.numbers[index]);
    let m = dfs2(
        row,
        index + 1,
        (total.to_string() + &row.numbers[index].to_string())
            .parse()
            .unwrap(),
    );
    l || r || m
}

pub fn part_one(input: &str) -> Option<u64> {
    let rows: Vec<Row> = input
        .split('\n')
        .map(|r| {
            let (result, nums_str) = r.split_once(':').unwrap();
            let nums = nums_str
                .split(' ')
                .skip(1)
                .map(|e| e.parse::<u64>().unwrap())
                .collect();
            Row {
                result: result.parse().unwrap(),
                numbers: nums,
            }
        })
        .collect();

    let l: u64 = rows
        .iter()
        .filter(|&r| dfs(r, 0, 0))
        .fold(0, |acc, x| acc + x.result as i64) as u64;
    Some(l)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rows: Vec<Row> = input
        .split('\n')
        .map(|r| {
            let (result, nums_str) = r.split_once(':').unwrap();
            let nums = nums_str
                .split(' ')
                .skip(1)
                .map(|e| e.parse::<u64>().unwrap())
                .collect();
            Row {
                result: result.parse().unwrap(),
                numbers: nums,
            }
        })
        .collect();

    let l: u64 = rows
        .iter()
        .filter(|&r| dfs2(r, 0, 0))
        .fold(0, |acc, x| acc + x.result as i64) as u64;
    Some(l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
