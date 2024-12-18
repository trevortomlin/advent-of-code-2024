advent_of_code::solution!(16);

use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Wall,
    Open,
    Start,
    End,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Node {
    x: usize,
    y: usize,
    dist: u32,
    dx: i32,
    dy: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(grid: &Vec<Vec<Cell>>, start: (usize, usize), end: (usize, usize)) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut dist: HashMap<(usize, usize), u32> = HashMap::new();
    let mut pq = BinaryHeap::new();

    for i in 0..rows {
        for j in 0..cols {
            dist.insert((i, j), u32::MAX);
        }
    }
    dist.insert(start, 0);

    pq.push(Node {
        x: start.0,
        y: start.1,
        dist: 0,
        dx: 0,
        dy: 1,
    });

    while let Some(Node { x, y, dist: d , dx, dy}) = pq.pop() {
        if (x, y) == end {
            return d;
        }

        for (dirx, diry) in DIRECTIONS.iter() {
            let nx = x as i32 + dirx;
            let ny = y as i32 + diry;

            if nx >= 0 && nx < rows as i32 && ny >= 0 && ny < cols as i32 {
                let nx = nx as usize;
                let ny = ny as usize;

                if grid[nx][ny] != Cell::Wall {
                    let mut new_dist = d;
                    if (dx == *dirx && dy == *diry) || (dx == -dirx && dy == -diry) {
                        new_dist += 1;
                    } else {
                        new_dist += 1001;
                    }

                    if new_dist < *dist.get(&(nx, ny)).unwrap() {
                        dist.insert((nx, ny), new_dist);
                        pq.push(Node {
                            x: nx,
                            y: ny,
                            dist: new_dist,
                            dx: *dirx as i32,
                            dy: *diry as i32,
                        });
                    }
                }
            }
        }
    }

    u32::MAX
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Cell::Wall,
                    '.' => Cell::Open,
                    'S' => Cell::Start,
                    'E' => Cell::End,
                    _ => panic!("Invalid character"),
                })
                .collect()
        })
        .collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == Cell::Start {
                start = (i, j);
            } else if grid[i][j] == Cell::End {
                end = (i, j);
            }
        }
    }

    let result = dijkstra(&grid, start, end);

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
