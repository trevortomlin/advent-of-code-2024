use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut total = 0;

    fn dfs(grid: &Vec<Vec<char>>, i: i32, j: i32, dir_i: i32, dir_j: i32, l: i32) -> bool {
        let in_grid = |grid: &Vec<Vec<char>>, i: i32, j: i32| -> bool {
            return i >= 0 && i < grid.len() as i32 && j >= 0 && j < grid[0].len() as i32;
        };

        let xmas = vec!['X', 'M', 'A', 'S'];

        if !in_grid(grid, i, j) || l >= 4 || grid[i as usize][j as usize] != xmas[l as usize] {
            return false;
        }
        if l == 3 && grid[i as usize][j as usize] == 'S' {
            return true;
        }
        return dfs(grid, i + dir_i, j + dir_j, dir_i, dir_j, l + 1);
    }

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] != 'X' {
                continue;
            }
            for x in -1..=1 {
                for y in -1..=1 {
                    if dfs(&grid, i as i32, j as i32, x, y, 0) {
                        total += 1;
                    }
                }
            }
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let m = vec![
        vec!['M', '.', 'S'],
        vec!['.', 'A', '.'],
        vec!['M', '.', 'S'],
    ];

    let transposed: Vec<Vec<_>> = (0..m[0].len())
        .map(|col| (0..m.len()).map(|row| m[row][col]).collect())
        .collect();

    let flipped_h: Vec<Vec<char>> = m
        .iter()
        .map(|row| row.iter().cloned().rev().collect())
        .collect();

    let flipped_h_transposed: Vec<Vec<_>> = (0..flipped_h[0].len())
        .map(|col| {
            (0..flipped_h.len())
                .map(|row| flipped_h[row][col])
                .collect()
        })
        .collect();

    let all_combinations = vec![m, transposed, flipped_h, flipped_h_transposed];

    let grid: Vec<Vec<char>> = input
        .split("\n")
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut total = 0;

    for i in 0..grid[0].len() - 2 {
        for k in 0..grid.len() - 2 {
            let mut tmp = vec![];
            for j in 0..3 {
                tmp.push(&grid[k + j][i..i + 3]);
            }

            for combo in &all_combinations {
                let mut matches = true;
                for row in 0..3 {
                    for col in 0..3 {
                        if ![(0, 0), (1, 1), (2, 2), (0, 2), (2, 0)].contains(&(row, col)) {
                            continue;
                        }
                        if tmp[row][col] != combo[row][col] {
                            matches = false;
                        }
                    }
                }
                if matches {
                    total += 1;
                    break;
                }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(9));
    }
}
