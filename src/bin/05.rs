use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (order, pages) = input.split_once("\n\n").unwrap();
    let mut order_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    order.split("\n").for_each(|e| {
        let (v, k) = e.split_once('|').unwrap();
        let k = k.parse::<i32>().unwrap();
        let v = v.parse::<i32>().unwrap();
        let mut set = order_map.get(&k).cloned().unwrap_or(HashSet::new());
        set.insert(v);
        order_map.insert(k, set);
    });
    let pages: Vec<Vec<i32>> = pages
        .split("\n")
        .map(|e| {
            e.split(",")
                .map(|m| m.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut total = 0;

    for page in pages {
        let mut safe = true;
        for (i, num) in page.iter().enumerate() {
            let prereqs = order_map.get(&num);
            match prereqs {
                Some(prereqs) => {
                    for prereq in prereqs {
                        if page.contains(prereq) && !page[0..i].contains(prereq) {
                            safe = false;
                        }
                    }
                }
                None => {}
            };
        }
        if safe {
            total += page[page.len() / 2];
        }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (order, pages) = input.split_once("\n\n").unwrap();
    let mut order_map: HashMap<i32, HashSet<i32>> = HashMap::new();
    order.split("\n").for_each(|e| {
        let (v, k) = e.split_once('|').unwrap();
        let k = k.parse::<i32>().unwrap();
        let v = v.parse::<i32>().unwrap();
        let mut set = order_map.get(&k).cloned().unwrap_or(HashSet::new());
        set.insert(v);
        order_map.insert(k, set);
    });
    let pages: Vec<Vec<i32>> = pages
        .split("\n")
        .map(|e| {
            e.split(",")
                .map(|m| m.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let mut total = 0;

    for page in pages {
        let mut safe = true;
        let mut copy = page.clone();
        for (i, num) in page.iter().enumerate() {
            let prereqs = order_map.get(&num);
            match prereqs {
                Some(prereqs) => {
                    for prereq in prereqs {
                        if page.contains(prereq) && !page[0..i].contains(prereq) {
                            safe = false;
                        }
                    }
                }
                None => {}
            };
        }

        if !safe {
            copy.sort_by(|a, b| {
                let prereqs = order_map.get(&a);
                match prereqs {
                    Some(prereq) => match prereq.contains(b) {
                        true => std::cmp::Ordering::Greater,
                        false => std::cmp::Ordering::Less,
                    },
                    None => std::cmp::Ordering::Equal,
                }
            });
            total += copy[copy.len() / 2];
        }
    }

    Some(total as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
