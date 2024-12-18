#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::collections::VecDeque;

fn parse(line: &str) -> (usize, usize) {
    scan_fmt!(line, "{},{}", usize, usize).unwrap()
}

#[derive(Debug, Clone)]
struct State {
    x: usize,
    y: usize,
    c: usize,
}

pub fn solve1(input: &[&str], dim: usize, drop: usize) -> usize {
    let mem: Vec<_> = input.iter().copied().map(parse).collect();

    let dx = [0, 1, 0, -1];
    let dy = [-1, 0, 1, 0];

    let mut q: VecDeque<State> = VecDeque::new();
    let mut seen = vec![vec![false; dim]; dim];

    let mut nmap = FxHashSet::default();
    for i in 0..drop {
        let x = mem[i].0;
        let y = mem[i].1;
        nmap.insert((x, y));
    }


    q.push_back(State { x: 0, y: 0, c: 0 });
    while let Some(State { x, y, c }) = q.pop_front() {
        if x == dim - 1 && y == dim - 1 {
            return c;
        }

        // visit neighbours
        for d in 0..4 {
            let nx = (x as i32 + dx[d]) as usize;
            let ny = (y as i32 + dy[d]) as usize;
            if nx < dim && ny < dim && !seen[nx][ny] && !nmap.contains(&(nx, ny)) {
                seen[nx][ny] = true;
                q.push_back(State { x: nx, y: ny, c: c + 1 });
            }
        }
    }

    0
}

#[advent_of_code(2024, 18, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    solve1(input, 71, 1024)
}

pub fn solve2(input: &[&str], dim: usize, start_at: usize) -> String {
    for i in start_at..input.len() {
        if solve1(input, dim, i) == 0 {
            return input[i-1].to_string();
        }
    }
    "Not found".to_string()
}

#[advent_of_code(2024, 18, 2)]
pub fn solve_level2(input: &[&str]) -> String {
    solve2(input, 71, 1023)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/18-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/18-sample.out").unwrap());

    #[test]
    fn test_level1() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().next().unwrap();
        assert_eq!(format!("{}", solve1(&input, 7, 12)), expected);
    }

    #[test]
    fn test_level2() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().skip(3).next().unwrap();
        assert_eq!(format!("{}", solve2(&input, 7, 12)), expected);
    }
}
