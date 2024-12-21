#![allow(unused_imports)]
#![allow(unused_variables)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::collections::VecDeque;
use memoize::memoize;
use im::{Vector, vector};

fn find(haystack: &Vec<Vec<char>>, needle: char) -> (usize, usize) {
    for x in 0..haystack[0].len() {
        for y in 0..haystack.len() {
            if haystack[y][x] == needle { return (x, y); }
        }
    }
    panic!("Did not find needle");
}

pub fn best_numpad(path: Vector<char>, ndirpads: usize) -> usize {
    let mut sum = 0;
    let mut cur = 'A';
    for t in path {
        sum += numpad(cur, t, ndirpads);
        cur = t;
    }
    sum
}


#[memoize]
pub fn numpad(from: char, to: char, ndirpads: usize) -> usize {
    // println!("numpad {}, {}, {}", from, to, ndirpads);
    let kp: Vec<Vec<char>> = vec![vec!['7', '8', '9'], vec!['4', '5', '6'], vec!['1', '2', '3'], vec![' ', '0', 'A']];
    let mut q: VecDeque<((usize, usize), usize, Vector<char>)> = VecDeque::new();
    q.push_back((find(&kp, from), 0, vector![]));

    let (ex, ey) = find(&kp, to);

    let dx = [0, 1, 0, -1];
    let dy = [-1, 0, 1, 0];
    let dc = ['^', '>', 'v', '<'];

    // println!("Going from ({:?}) to ({}, {})", find(&kp, from), ex, ey);

    let mut ans = usize::max_value();
    let mut dist = vec![vec![usize::max_value(); 3]; 4];
    while let Some(((x, y), c, p)) = q.pop_front() {
        // println!("{},{}", x, y);
        if x == ex && y == ey {
            // println!("  ans!");
            let mut pp = p.clone();
            pp.push_back('A');
            let res = best_dirpad(pp, ndirpads);
            ans = ans.min(res);
        } else {
            for d in 0..4 {
                let nx = (x as i64 + dx[d]) as usize;
                let ny = (y as i64 + dy[d]) as usize;
                if nx < 3 && ny < 4 && kp[ny][nx] != ' ' && c+1 <= dist[ny][nx] {
                    // println!("  {},{}", nx, ny);
                    dist[ny][nx] = c+1;
                    let mut np = p.clone();
                    np.push_back(dc[d]);
                    q.push_back(((nx, ny), c+1, np));
                }
            }
        }
    }

    ans
}

#[memoize]
pub fn best_dirpad(path: Vector<char>, ndirpads: usize) -> usize {
    if ndirpads == 1 { return path.len(); }

    let mut sum = 0;
    let mut cur = 'A';
    for t in path {
        sum += dirpad(cur, t, ndirpads);
        cur = t;
    }
    sum
}

#[memoize]
pub fn dirpad(from: char, to: char, ndirpads: usize) -> usize {
    // println!("dirpad {}, {}, {}", from, to, ndirpads);
    let kp: Vec<Vec<char>> = vec![vec![' ', '^', 'A'], vec!['<', 'v', '>']];
    let mut q: VecDeque<((usize, usize), usize, Vector<char>)> = VecDeque::new();
    q.push_back((find(&kp, from), 0, vector![]));

    let (ex, ey) = find(&kp, to);

    let dx = [0, 1, 0, -1];
    let dy = [-1, 0, 1, 0];
    let dc = ['^', '>', 'v', '<'];

    // println!("Going from ({:?}) to ({}, {})", find(&kp, from), ex, ey);

    let mut ans = usize::max_value();
    let mut dist = vec![vec![usize::max_value(); 3]; 2];
    while let Some(((x, y), c, p)) = q.pop_front() {
        // println!("{},{}", x, y);
        if x == ex && y == ey {
            // println!("  ans!");
            let mut pp = p.clone();
            pp.push_back('A');
            let res = best_dirpad(pp, ndirpads - 1);
            ans = ans.min(res);
        } else {
            for d in 0..4 {
                let nx = (x as i64 + dx[d]) as usize;
                let ny = (y as i64 + dy[d]) as usize;
                if nx < 3 && ny < 2 && kp[ny][nx] != ' ' && c+1 <= dist[ny][nx] {
                    // println!("  {},{}", nx, ny);
                    dist[ny][nx] = c+1;
                    let mut np = p.clone();
                    np.push_back(dc[d]);
                    q.push_back(((nx, ny), c+1, np));
                }
            }
        }
    }
    ans
}

#[advent_of_code(2024, 21, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let mut ans = 0;
    for seq in input {
        let s: Vector<char> = seq.chars().collect();
        let len = best_numpad(s, 3);
        let num_part = seq[0..seq.len() - 1].parse::<usize>().unwrap();
        ans += len * num_part;
    }
    ans
}

#[advent_of_code(2024, 21, 2)]
pub fn solve_level2(input: &[&str]) -> usize {
    let mut ans = 0;
    for seq in input {
        let s: Vector<char> = seq.chars().collect();
        let len = best_numpad(s, 26);
        let num_part = seq[0..seq.len() - 1].parse::<usize>().unwrap();
        ans += len * num_part;

    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/21-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/21-sample.out").unwrap());

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
