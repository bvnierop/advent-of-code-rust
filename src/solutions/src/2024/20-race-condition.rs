#![allow(unused_imports)]
#![allow(unused_variables)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::collections::{BinaryHeap, VecDeque};
use std::cmp::Ordering;
use rustc_hash::FxHashSet;

fn dist_from(grid: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<Vec<usize>> {
    let dx = [0, 1, 0, -1];
    let dy = [-1, 0, 1, 0];
    let width = grid[0].len();
    let height = grid.len();
    let (sx, sy) = start;

    let mut q = VecDeque::new();
    q.push_back((sx, sy, 0));
    let mut dists = vec![vec![usize::max_value(); width]; height];
    dists[sy][sx] = 0;
    while let Some((x, y, c)) = q.pop_front() {
        for d in 0..4 {
            let nx = (x as i64 + dx[d]) as usize;
            let ny = (y as i64 + dy[d]) as usize;
            if nx < width && ny < height && dists[ny][nx] > c + 1 && grid[ny][nx] != '#' {
                dists[ny][nx] = c + 1;
                q.push_back((nx, ny, c + 1));
            }
        }
    }
    dists
}

pub fn solve(input: &[&str], diff: usize, jump: usize) -> usize {
    let grid = input.into_iter().map(|line| line.chars().collect_vec()).collect_vec();

    let width = grid[0].len();
    let height = grid.len();

    let mut sx = 0; let mut sy = 0;
    let mut ex = 0; let mut ey = 0;
    for y in 0..height {
        for x in 0..width {
            if grid[y][x] == 'S' {
                sx = x; sy = y;
            }
            if grid[y][x] == 'E' {
                ex = x; ey = y;
            }
        }
    }

    let dists = dist_from(&grid, (sx, sy));
    let diste = dist_from(&grid, (ex, ey));

    let mut count = 0;
    for y in 0..height {
        for x in 0..width {
            for ny in y.saturating_sub(jump)..height.min(y+jump+1) {
                for nx in x.saturating_sub(jump)..width.min(x+jump+1) {
                    let dist = ((y as i64 - ny as i64).abs() + (x as i64 - nx as i64).abs()) as usize;
                    if grid[y][x] != '#' && grid[ny][nx] != '#' && dist <= jump {
                        let cheated = dists[y][x] + diste[ny][nx] + (dist as usize) - 1;
                        if cheated + diff < dists[ey][ex] {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

#[advent_of_code(2024, 20, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    solve(input, 100, 2)
}

#[advent_of_code(2024, 20, 2)]
pub fn solve_level2(input: &[&str]) -> usize {
    solve(input, 100, 20)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/20-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/20-sample.out").unwrap());

    #[test]
    fn test_level1() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().next().unwrap();
        assert_eq!(format!("{}", solve_level1(&input)), expected);
    }

    #[test]
    fn test_level2() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().skip(3).next().unwrap();
        assert_eq!(format!("{}", solve_level2(&input)), expected);
    }
}
