advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let mut input_iter = input.split("\n\n");
    let mut grid: Vec<Vec<char>> = input_iter
        .next()
        .unwrap()
        .split('\n')
        .map(|r| r.chars().collect::<Vec<char>>())
        .collect();
    let moves: Vec<char> = input_iter.next().unwrap().chars().collect();

    let mut pos = (1i32, 1i32);
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '@' {
                pos = (i as i32, j as i32);
                grid[i][j] = '.';
                break;
            }
        }
    }
    for m in &moves {
        let dir = match m {
            '^' => (-1, 0),
            '>' => (0, 1),
            'v' => (1, 0),
            '<' => (0, -1),
            _ => (0, 0),
        };
        let nx = pos.0 as i32 + dir.0;
        let ny = pos.1 as i32 + dir.1;

        if grid[nx as usize][ny as usize] == '#' {
        } else if grid[nx as usize][ny as usize] == 'O' {
            let mut nx2 = nx;
            let mut ny2 = ny;
            while grid[nx2 as usize][ny2 as usize] == 'O' {
                nx2 += dir.0;
                ny2 += dir.1;
            }
            if grid[nx2 as usize][ny2 as usize] == '#' {
                continue;
            }
            grid[nx as usize][ny as usize] = '.';
            grid[nx2 as usize][ny2 as usize] = 'O';
            pos = (nx, ny);
        } else {
            pos = (nx, ny);
        }
    }

    let mut total = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                total += 100 * i + j;
            }
        }
    }

    Some(total as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input_iter = input.split("\n\n");
    let mut grid: Vec<Vec<char>> = input_iter
        .next()
        .unwrap()
        .split('\n')
        .map(|r| r.chars().collect::<Vec<char>>())
        .collect();
    let moves: Vec<char> = input_iter.next().unwrap().chars().collect();

    let mut new_grid: Vec<Vec<char>> = vec![vec![]];

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if new_grid.get(i).is_none() {
                new_grid.push(vec![]);
            }
            if grid[i][j] == '#' {
                new_grid[i].push('#');
                new_grid[i].push('#');
            } else if grid[i][j] == 'O' {
                new_grid[i].push('[');
                new_grid[i].push(']');
            } else if grid[i][j] == '.' {
                new_grid[i].push('.');
                new_grid[i].push('.');
            } else if grid[i][j] == '@' {
                new_grid[i].push('@');
                new_grid[i].push('.');
            } else {
                new_grid[i].push(grid[i][j]);
            }
        }
    }

    let mut expanded_grid = new_grid.clone();
    let mut pos = (0i32, 0i32);

    for i in 0..expanded_grid.len() {
        for j in 0..expanded_grid[i].len() {
            if expanded_grid[i][j] == '@' {
                pos = (i as i32, j as i32);
                expanded_grid[i][j] = '.';
                expanded_grid[i][j + 1] = '.';
                break;
            }
        }
    }

    for m in &moves {
        let dir = match m {
            '^' => (-1, 0),
            'v' => (1, 0),
            '>' => (0, 1),
            '<' => (0, -1),
            _ => (0, 0),
        };

        let nx = pos.0 + dir.0;
        let ny = pos.1 + dir.1;

        let nx2 = nx;
        let ny2 = ny + 1;

        if nx < 0
            || nx >= expanded_grid.len() as i32
            || ny < 0
            || ny >= expanded_grid[0].len() as i32
            || ny2 >= expanded_grid[0].len() as i32
        {
            continue;
        }

        if expanded_grid[nx as usize][ny as usize] == '#'
            || expanded_grid[nx2 as usize][ny2 as usize] == '#'
        {
            continue;
        } else if (expanded_grid[nx as usize][ny as usize] == '['
            || expanded_grid[nx as usize][ny as usize] == ']')
            && (expanded_grid[nx2 as usize][ny2 as usize] == '['
                || expanded_grid[nx2 as usize][ny2 as usize] == ']')
        {
            let box_nx = nx + dir.0;
            let box_ny = ny + dir.1;

            let box_nx2 = box_nx;
            let box_ny2 = box_ny + 1;

            if box_nx < 0
                || box_nx >= expanded_grid.len() as i32
                || box_ny < 0
                || box_ny >= expanded_grid[0].len() as i32
                || box_nx2 < 0
                || box_nx2 >= expanded_grid.len() as i32
                || box_ny2 < 0
                || box_ny2 >= expanded_grid[0].len() as i32
            {
                continue;
            }

            if expanded_grid[box_nx as usize][box_ny as usize] == '.'
                && expanded_grid[box_nx2 as usize][box_ny2 as usize] == '.'
            {
                expanded_grid[box_nx as usize][box_ny as usize] =
                    expanded_grid[nx as usize][ny as usize];
                expanded_grid[box_nx2 as usize][box_ny2 as usize] =
                    expanded_grid[nx2 as usize][ny2 as usize];

                expanded_grid[nx as usize][ny as usize] = '.';
                expanded_grid[nx2 as usize][ny2 as usize] = '.';

                pos = (nx, ny);
            }
        } else {
            expanded_grid[nx as usize][ny as usize] = '@';
            expanded_grid[nx2 as usize][ny2 as usize] = '.';

            expanded_grid[pos.0 as usize][pos.1 as usize] = '.';
            expanded_grid[pos.0 as usize][pos.1 as usize + 1] = '.';

            pos = (nx, ny);
        }
    }

    let mut total = 0;
    for i in 0..expanded_grid.len() {
        for j in 0..expanded_grid[i].len() {
            if expanded_grid[i][j] == '['
                && j + 1 < expanded_grid[i].len()
                && expanded_grid[i][j + 1] == ']'
            {
                total += 100 * i + j;
            }
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
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9021));
    }
}
