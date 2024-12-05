#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::collections::{VecDeque, HashSet};

const WIDTH: i32 = 110;
const HEIGHT: i32 = 110;


#[advent_of_code(2021, 9, 1)]
pub fn solve_level1(input: &[&str]) -> u32 {
    let mut grid = [[9; WIDTH as usize]; HEIGHT as usize];

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[x + 1][y + 1] = c.to_digit(10).expect("Digit");
        }
    }

    let dx = [-1, 0, 1, 0];
    let dy = [0, -1, 0, 1];

    let mut sum = 0;
    for x in 1..WIDTH-1 {
        for y in 1..HEIGHT-1 {
            let mut low = true;
            for (dx, dy) in dx.iter().zip(dy.iter()) {
                if grid[(x + dx) as usize][(y + dy) as usize] <= grid[x as usize][y as usize] { low = false }
            }
            if low { sum += grid[x as usize][y as usize] + 1; }
        }
    }

    sum
}

fn ff(grid: &[[u32; WIDTH as usize]; HEIGHT as usize], start: (usize, usize)) -> u32 {

    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back(start);
    let mut s: HashSet<(usize, usize)> = HashSet::new();
    s.insert(start);
    let mut size = 1;

    let dx = [-1, 0, 1, 0];
    let dy = [0, -1, 0, 1];

    while !q.is_empty() {
        let (x, y) = q.pop_front().expect("Expected a front element");
        for (dx, dy) in dx.iter().zip(dy.iter()) {
            let nx = (x as i32 + dx) as usize;
            let ny = (y as i32 + dy) as usize;
            if !s.contains(&(nx, ny)) && grid[nx][ny] < 9 {
                q.push_back((nx, ny));
                s.insert((nx, ny));
                size += 1;
            }
        }
    }
    size
}

#[advent_of_code(2021, 9, 2)]
pub fn solve_level2(input: &[&str]) -> u32 {
    let mut grid = [[9; WIDTH as usize]; HEIGHT as usize];

    for (y, line) in input.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            grid[x + 1][y + 1] = c.to_digit(10).expect("Digit");
        }
    }

    let dx = [-1, 0, 1, 0];
    let dy = [0, -1, 0, 1];

    let mut lows: Vec<(usize, usize)> = vec![];
    for x in 1..WIDTH-1 {
        for y in 1..HEIGHT-1 {
            let mut low = true;
            for (dx, dy) in dx.iter().zip(dy.iter()) {
                if grid[(x + dx) as usize][(y + dy) as usize] <= grid[x as usize][y as usize] { low = false }
            }
            if low { lows.push((x as usize, y as usize)) };
        }
    }

    lows.iter().map(|pt| ff(&grid, *pt))
        .sorted()
        .rev()
        .take(3)
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/09-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/09-sample.out").unwrap());

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
