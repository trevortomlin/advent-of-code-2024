advent_of_code::solution!(18);

use std::collections::{HashSet, VecDeque};

fn bfs(
    grid: &Vec<Vec<char>>,
    start: (usize, usize),
    target: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut queue: VecDeque<((usize, usize), Vec<(usize, usize)>)> = VecDeque::new();
    queue.push_back((start, vec![start]));

    let mut visited = vec![vec![false; cols]; rows];
    visited[start.0][start.1] = true;

    while let Some(((x, y), path)) = queue.pop_front() {
        if (x, y) == target {
            return Some(path);
        }

        for &(dx, dy) in &directions {
            let nx = (x as isize + dx) as usize;
            let ny = (y as isize + dy) as usize;

            if nx < rows && ny < cols && !visited[nx][ny] && grid[nx][ny] != '#' {
                visited[nx][ny] = true;
                let mut new_path = path.clone();
                new_path.push((nx, ny));
                queue.push_back(((nx, ny), new_path));
            }
        }
    }

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let pairs: Vec<(i32, i32)> = input
        .split('\n')
        .map(|e| e.split_once(',').unwrap())
        .map(|(r1, r2)| return (r1.parse().unwrap(), r2.parse().unwrap()))
        .collect();

    let mut grid = vec![vec!['.'; 71]; 71];

    for pair in &pairs[0..1024] {
        grid[pair.1 as usize][pair.0 as usize] = '#';
    }

    let start = (0, 0);
    let m = bfs(&grid, start, (70, 70));

    Some((m.unwrap().len() - 1) as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let pairs: Vec<(i32, i32)> = input
        .split('\n')
        .map(|e| e.split_once(',').unwrap())
        .map(|(r1, r2)| return (r1.parse().unwrap(), r2.parse().unwrap()))
        .collect();

    let mut grid = vec![vec!['.'; 71]; 71];
    let start = (0, 0);

    for pair in &pairs {
        grid[pair.1 as usize][pair.0 as usize] = '#';
        let m = bfs(&grid, start, (70, 70));
        match m {
            Some(_) => continue,
            None => {
                return Some(format!("{},{}", pair.0, pair.1));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(344));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("46,18".to_string()));
    }
}
