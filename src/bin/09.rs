advent_of_code::solution!(9);

#[derive(Debug, Clone, PartialEq)]
enum BlockType {
    File(u32),
    Free,
}

#[derive(Debug, Clone)]
struct Block {
    block_type: BlockType,
    length: u32,
}

pub fn part_one(input: &str) -> Option<u64> {
    let v: Vec<Option<u64>> = input
        .chars()
        .enumerate()
        .map(|(i, v)| {
            if i % 2 == 0 {
                return vec![Some((i / 2) as u64); v.to_digit(10).unwrap() as usize];
            } else {
                return vec![None; v.to_digit(10).unwrap() as usize];
            }
        })
        .flatten()
        .collect();

    let mut l = 0;
    let mut r = v.len() - 1;

    let mut new_v = v.clone();
    while l < r {
        while v[r].is_none() {
            r -= 1;
        }
        while v[l].is_some() {
            l += 1;
        }
        if l >= r {
            break;
        }
        new_v.swap(l, r);

        r -= 1;
        l += 1;
    }

    let result = new_v
        .iter()
        .enumerate()
        .map(|(i, &e)| match e {
            Some(v) => v as u64 * i as u64,
            None => 0,
        })
        .fold(0, |acc, x| acc + x);

    return Some(result);
}

pub fn part_two(input: &str) -> Option<u64> {
    let v: Vec<Block> = input
        .chars()
        .enumerate()
        .map(|(i, v)| Block {
            block_type: if i % 2 == 0 {
                BlockType::File((i / 2) as u32)
            } else {
                BlockType::Free
            },
            length: v.to_digit(10).unwrap(),
        })
        .collect();

    let mut l = 0;
    let mut r = v.len() - 1;

    let mut new_v = v.clone();

    while r > 0 {
        while new_v[r].block_type == BlockType::Free {
            r -= 1;
        }
        while new_v[l].block_type != BlockType::Free {
            l += 1;
        }
        if l >= r {
            l = 0;
            r -= 1;
            continue;
        }

        if new_v[l].length == new_v[r].length {
            new_v.swap(l, r);
            r -= 1;
            l = 0;
        } else if new_v[l].length < new_v[r].length {
            l += 1;
        } else {
            new_v[l].length -= new_v[r].length;
            let tmp = new_v[r].clone();
            new_v.remove(r);
            new_v.insert(
                r,
                Block {
                    block_type: BlockType::Free,
                    length: tmp.length,
                },
            );
            new_v.insert(l, tmp);
            r -= 1;
            l = 0;
        }
    }

    let test = new_v
        .iter()
        .map(|e| match e.block_type {
            BlockType::File(m) => vec![m.to_string(); e.length as usize],
            _ => vec![0.to_string(); e.length as usize],
        })
        .flatten()
        .collect::<Vec<String>>();

    let result = test
        .iter()
        .enumerate()
        .map(|(i, e)| i as u64 * e.parse::<u64>().unwrap())
        .fold(0, |acc, x: u64| acc + x);

    Some(result as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
