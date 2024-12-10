use std::collections::{HashMap, HashSet};

advent_of_code::solution!(10);

fn dfs(grid: &Vec<Vec<u32>>, i: i32, j: i32, last: i32, visited: &mut HashSet<(i32, i32)>) -> u32 {
    if i < 0
        || j < 0
        || i >= grid.len() as i32
        || j >= grid[0].len() as i32
        || visited.contains(&(i, j))
    {
        return 0;
    }
    if (grid[i as usize][j as usize] as i32 - last as i32) != 1 {
        return 0;
    }
    if grid[i as usize][j as usize] == 9 {
        visited.insert((i, j));
        return 1;
    }
    return dfs(grid, i + 1, j, grid[i as usize][j as usize] as i32, visited)
        + dfs(grid, i - 1, j, grid[i as usize][j as usize] as i32, visited)
        + dfs(grid, i, j + 1, grid[i as usize][j as usize] as i32, visited)
        + dfs(grid, i, j - 1, grid[i as usize][j as usize] as i32, visited);
}

fn dfs2(
    grid: &Vec<Vec<u32>>,
    i: i32,
    j: i32,
    last: i32,
    visited: &mut HashMap<(i32, i32), i32>,
) -> u32 {
    if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
        return 0;
    }
    if (grid[i as usize][j as usize] as i32 - last as i32) != 1 {
        return 0;
    }
    if grid[i as usize][j as usize] == 9 {
        let v = *visited.entry((i, j)).or_insert_with(|| 0) + 1;

        return v as u32;
    }
    return dfs2(grid, i + 1, j, grid[i as usize][j as usize] as i32, visited)
        + dfs2(grid, i - 1, j, grid[i as usize][j as usize] as i32, visited)
        + dfs2(grid, i, j + 1, grid[i as usize][j as usize] as i32, visited)
        + dfs2(grid, i, j - 1, grid[i as usize][j as usize] as i32, visited);
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .split("\n")
        .map(|r| r.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                let h = &mut HashSet::new();
                let l = dfs(&grid, i as i32, j as i32, -1, h);
                total += l;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<u32>> = input
        .split("\n")
        .map(|r| r.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 0 {
                let h = &mut HashMap::new();
                let l = dfs2(&grid, i as i32, j as i32, -1, h);
                total += l;
            }
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
