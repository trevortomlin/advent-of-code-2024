use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    let mut gr = 0i32;
    let mut gc = 00i32;
    let mut gdx = 00i32;
    let mut gdy = 00i32;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            match grid[r][c] {
                '^' => {
                    gr = r as i32;
                    gc = c as i32;
                    gdx = 0;
                    gdy = -1
                }
                'v' => {
                    gr = r as i32;
                    gc = c as i32;
                    gdx = 0;
                    gdy = 1
                }
                '<' => {
                    gr = r as i32;
                    gc = c as i32;
                    gdx = -1;
                    gdy = 0
                }
                '>' => {
                    gr = r as i32;
                    gc = c as i32;
                    gdx = 1;
                    gdy = 0
                }
                _ => {}
            }
        }
    }

    while gr >= 0
        && gr < grid.len() as i32
        && gc >= 0
        && gc < grid[0].len() as i32
        && (gr + gdy) >= 0
        && (gr + gdy) < grid.len() as i32
        && (gc + gdx) >= 0
        && (gc + gdx) < grid[0].len() as i32
    {
        visited.insert((gr, gc));
        while grid[(gr + gdy) as usize][(gc + gdx) as usize] == '#' {
            let tmp = gdx;
            gdx = -gdy;
            gdy = tmp;
        }
        gc += gdx;
        gr += gdy;
    }

    Some(visited.len() as u32 + 1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input
        .split("\n")
        .into_iter()
        .map(|s| s.chars().collect::<Vec<char>>())
        .collect();

        let mut visited: HashSet<(i32, i32)> = HashSet::new();

        let mut gr = 0i32;
        let mut gc = 00i32;
        let mut gdx = 00i32;
        let mut gdy = 00i32;
    
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                match grid[r][c] {
                    '^' => {
                        gr = r as i32;
                        gc = c as i32;
                        gdx = 0;
                        gdy = -1
                    }
                    'v' => {
                        gr = r as i32;
                        gc = c as i32;
                        gdx = 0;
                        gdy = 1
                    }
                    '<' => {
                        gr = r as i32;
                        gc = c as i32;
                        gdx = -1;
                        gdy = 0
                    }
                    '>' => {
                        gr = r as i32;
                        gc = c as i32;
                        gdx = 1;
                        gdy = 0
                    }
                    _ => {}
                }
            }
        }

        let gri = gr;
        let gci = gc;
    
        while gr >= 0
            && gr < grid.len() as i32
            && gc >= 0
            && gc < grid[0].len() as i32
            && (gr + gdy) >= 0
            && (gr + gdy) < grid.len() as i32
            && (gc + gdx) >= 0
            && (gc + gdx) < grid[0].len() as i32
        {
            visited.insert((gr, gc));
            while grid[(gr + gdy) as usize][(gc + gdx) as usize] == '#' {
                let tmp = gdx;
                gdx = -gdy;
                gdy = tmp;
            }
            gc += gdx;
            gr += gdy;
        }
        visited.insert((gr, gc));

    // let l = loop_or_not(&grid);
    // println!("{:?}", l);
    let mut total = 0;

    for location in visited {
        if location == (gri, gci) {
            continue
        }
        let mut grid_clone = grid.clone();
        grid_clone[location.0 as usize][location.1 as usize] = '#';
        let l = loop_or_not(&grid_clone);
        // println!("{:?}", l);
        if l {
            // println!("{:?}", location);
            total +=1;
        }
    }
    Some(total)
}

fn loop_or_not(grid: &Vec<Vec<char>>) -> bool {
    let mut visited: HashSet<(i32, i32, i32, i32)> = HashSet::new();

    let mut gr = 0i32;
    let mut gc = 00i32;
    let mut gdx = 00i32;
    let mut gdy = 00i32;

    for r in 0..grid.len() {
        for c in 0..grid[0].len() {
            match grid[r][c] {
                '^' => {
                    gr = r as i32;
                    gc = c as i32;
                    gdx = 0;
                    gdy = -1
                }
                'v' => {
                    gr = r as i32;
                    gc = c as i32;
                    gdx = 0;
                    gdy = 1
                }
                '<' => {
                    gr = r as i32;
                    gc = c as i32;
                    gdx = -1;
                    gdy = 0
                }
                '>' => {
                    gr = r as i32;
                    gc = c as i32;
                    gdx = 1;
                    gdy = 0
                }
                _ => {}
            }
        }
    }

    while gr >= 0
        && gr < grid.len() as i32
        && gc >= 0
        && gc < grid[0].len() as i32
        && (gr + gdy) >= 0
        && (gr + gdy) < grid.len() as i32
        && (gc + gdx) >= 0
        && (gc + gdx) < grid[0].len() as i32
    {   
        if visited.contains(&(gr, gc, gdx, gdy)) {
            return true;
        }
        visited.insert((gr, gc, gdx, gdy));
        while grid[(gr + gdy) as usize][(gc + gdx) as usize] == '#' {
            let tmp = gdx;
            gdx = -gdy;
            gdy = tmp;
        }
        gc += gdx;
        gr += gdy;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
