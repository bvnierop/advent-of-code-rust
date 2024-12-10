#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::collections::VecDeque;

type Point = (usize, usize);

#[advent_of_code(2024, 10, 1)]
pub fn solve_level1(input: &[&str]) -> u32 {
    // parse input to 2d vec
    let map: Vec<Vec<i32>> = input.iter().map(|&line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();
    let width = map[0].len();
    let height = map.len();

    // find trailheads
    let mut trailheads: Vec<Point> = vec![];
    for x in 0..width {
        for y in 0..height {
            if map[y][x] == 0 { trailheads.push((x, y)); };
        }
    }

    let dx: [i32; 4] = [ -1, 0, 1, 0 ];
    let dy: [i32; 4] = [ 0, -1, 0, 1 ];

    let mut sum = 0;
    // for each trailhead
    for trailhead in trailheads {
        //   bfs to find all paths to a '9'
        let mut q: VecDeque<Point> = VecDeque::new();
        let mut seen = vec![vec![false; width]; height];
        q.push_back(trailhead);
        while let Some((cx, cy)) = q.pop_front() {
            if map[cy][cx] == 9 {
                sum += 1;
                continue;
            }

            for d in 0..4 {
                let nx = (cx as i32 + dx[d]) as usize;
                let ny = (cy as i32 + dy[d]) as usize;
                if nx < width && ny < height {
                    if map[ny][nx] - map[cy][cx] == 1 {
                        if !seen[ny][nx] {
                            q.push_back((nx , ny ));
                            seen[ny][nx] = true;
                        }
                    }
                }
            }

        }
    }
    sum
}

#[advent_of_code(2024, 10, 2)]
pub fn solve_level2(input: &[&str]) -> u32 {
    // parse input to 2d vec
    let map: Vec<Vec<i32>> = input.iter().map(|&line| line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()).collect();
    let width = map[0].len();
    let height = map.len();

    // find trailheads
    let mut trailheads: Vec<Point> = vec![];
    for x in 0..width {
        for y in 0..height {
            if map[y][x] == 0 { trailheads.push((x, y)); };
        }
    }

    let dx: [i32; 4] = [ -1, 0, 1, 0 ];
    let dy: [i32; 4] = [ 0, -1, 0, 1 ];

    let mut sum = 0;
    // for each trailhead
    for trailhead in trailheads {
        //   bfs to find all paths to a '9'
        let mut q: VecDeque<Point> = VecDeque::new();
        q.push_back(trailhead);
        while let Some((cx, cy)) = q.pop_front() {
            if map[cy][cx] == 9 {
                sum += 1;
                continue;
            }

            for d in 0..4 {
                let nx = (cx as i32 + dx[d]) as usize;
                let ny = (cy as i32 + dy[d]) as usize;
                if nx < width && ny < height {
                    if map[ny][nx] - map[cy][cx] == 1 {
                        q.push_back((nx , ny ));
                    }
                }
            }

        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/10-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/10-sample.out").unwrap());

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
