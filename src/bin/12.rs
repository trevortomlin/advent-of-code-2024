use std::collections::HashSet;

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
        return;
    }
    if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
        return;
    }
    if grid[i as usize][j as usize] != c {
        return;
    }

    visited.insert((i, j));
    points.push((i, j));

    for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
        dfs2(grid, i + dx, j + dy, c, visited, points);
    }
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

fn dfs3(
    grid: &Vec<Vec<char>>,
    i: i32,
    j: i32,
    c: char,
    visited: &mut HashSet<(i32, i32)>,
    points: &mut Vec<(i32, i32)>,
) {
    if visited.contains(&(i, j)) {
        return;
    }
    if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
        return;
    }

    if grid[i as usize][j as usize] != c {
        return;
    }

    visited.insert((i, j));
    points.push((i, j));

    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dx, dy) in directions.iter() {
        dfs3(grid, i + dx, j + dy, c, visited, points);
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.split('\n').map(|r| r.chars().collect()).collect();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut total = 0;

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if visited.contains(&(i as i32, j as i32)) {
                continue;
            }

            let mut visited2 = HashSet::new();

            let mut points = Vec::new();
            dfs3(
                &grid,
                i as i32,
                j as i32,
                grid[i][j],
                &mut visited2,
                &mut points,
            );

            visited.extend(points.iter());

            let mut area = points.len() as u32;
            let mut sides = 0;

            let mut t = 0;
            for (x, y) in &points {
                t += is_corner(*x as i32, *y as i32, &grid);
            }

            println!("{} ({}) = {}", grid[i][j], points.len(), t);

            total += t as u32 * area;
        }
    }

    Some(total)
}

fn is_corner(x: i32, y: i32, grid: &Vec<Vec<char>>) -> i32 {
    let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    let mut external_count = 0;
    let mut internal_count = 0;

    let perpendicular_pairs = [
        [(-1, 0), (0, 1)],
        [(-1, 0), (0, -1)],
        [(1, 0), (0, -1)],
        [(1, 0), (0, 1)],
    ];

    for pair in perpendicular_pairs.iter() {
        let mut pair_valid = true;
        let mut pair_is_external = false;
        let mut pair_is_internal = true;
        let mut diagonal_valid = true;

        for (dx, dy) in pair.iter() {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || nx >= grid.len() as i32 || ny < 0 || ny >= grid[0].len() as i32 {
                pair_is_external = true;
            } else if grid[nx as usize][ny as usize] != grid[x as usize][y as usize] {
                pair_is_external = true;
            } else {
                pair_is_internal = false;
                pair_valid = false;
                break;
            }
        }

        if pair_valid {
            let diagonal_x = x + pair[0].0 + pair[1].0;
            let diagonal_y = y + pair[0].1 + pair[1].1;

            if diagonal_x >= 0
                && diagonal_x < grid.len() as i32
                && diagonal_y >= 0
                && diagonal_y < grid[0].len() as i32
                && grid[diagonal_x as usize][diagonal_y as usize] != grid[x as usize][y as usize]
            {
                diagonal_valid = true;
            }
        }

        if pair_valid && pair_is_external {
            external_count += 1;
        }

        if pair_valid && pair_is_internal && diagonal_valid {
            internal_count += 1;
        }
    }

    external_count + internal_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(80));
    }
}
