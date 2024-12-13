use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(12);

fn dfs(
    grid: &Vec<Vec<char>>,
    i: i32,
    j: i32,
    c: char,
    visited: &mut HashSet<(i32, i32)>,
) -> (i32, i32) {
    if visited.contains(&(i, j)) {
        return (0, 0);
    }
    if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
        return (0, 1);
    }
    if grid[i as usize][j as usize] != c {
        return (0, 1);
    }

    visited.insert((i, j));

    let mut total = (1, 0);
    for (dx, dy) in &[(-1, 0), (1, 0), (0, 1), (0, -1)] {
        let (new_area, new_perim) = dfs(grid, i + dx, j + dy, c, visited);
        total = (total.0 + new_area, total.1 + new_perim);
    }

    total
}

fn dfs2(
    grid: &Vec<Vec<char>>,
    i: i32,
    j: i32,
    c: char,
    visited: &mut HashSet<(i32, i32)>,
    points: &mut Vec<(i32, i32)>,
) {
    if visited.contains(&(i, j)) {
        // points.push((i, j));
        return;
    }
    if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
        points.push((i, j));
        return;
    }
    if grid[i as usize][j as usize] != c {
        // println!("{}, {}, {}, {}", i, j, c, grid[i as usize][j as usize]);
        points.push((i, j));
        return;
    }

    visited.insert((i, j));

    // let mut total = (1, 0);
    for (dx, dy) in &[(-1, 0), (1, 0), (0, 1), (0, -1)] {
        dfs2(grid, i + dx, j + dy, c, visited, points);
        // total = (total.0 + new_area, total.1 + new_perim);
    }

    // total
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.split('\n').map(|r| r.chars().collect()).collect();

    let mut char_visited: HashSet<(i32, i32)> = HashSet::new();
    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if char_visited.contains(&(i as i32, j as i32)) {
                continue;
            }
            let visited = &mut HashSet::new();
            let p = dfs(&grid, i as i32, j as i32, grid[i][j], visited);
            char_visited.extend(visited.iter());
            total += p.0 * p.1;
        }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.split('\n').map(|r| r.chars().collect()).collect();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            let p = &mut vec![];
            let visited = &mut HashSet::new();
            dfs2(&grid, i as i32, j as i32, grid[i][j], visited, p);

            println!("{:?}", p);

            let mut sides = 1;

            p.sort();

            println!("{:?}", p);

            let mut last = (i32::MIN, i32::MIN);

            for m in 0..p.len() {
                if p[m].0 != last.0 && p[m].1 != last.1 {
                    sides+=1;
                }
                last = p[m];
                
            }
            // sides += 1;
            println!("{} {}", grid[i][j], sides);
            // println!("{:?}", p);
            // println!("{}, {:?}", grid[i][j], p);
            // println!("{:?}", m);
            // println!("{:?}", p.iter().map(|&e| e.0).unique().collect::<Vec<i32>>());
            // println!("{:?}", p.iter().map(|&e| e.1).unique().collect::<Vec<i32>>());
            // break;
        }
        // break;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}
