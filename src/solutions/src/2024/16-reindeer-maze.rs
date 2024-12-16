#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use rustc_hash::FxHashSet;

#[derive(Clone, Eq, PartialEq)]
struct State {
    c: usize,
    x: usize,
    y: usize,
    o: usize,
    p: FxHashSet<(usize, usize)>
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.c.cmp(&self.c)
            .then_with(|| self.o.cmp(&other.o))
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[advent_of_code(2024, 16, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let map: Vec<Vec<char>> = input.iter().map(|&line| line.chars().collect()).collect();

    let width = map[0].len();
    let height = map.len();

    let mut rx = 0; let mut ry = 0;
    let mut sx = 0; let mut sy = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 'E' {
                rx = x; ry = y;
            }
            if map[y][x] == 'S' {
                sx = x; sy = y;
            }
        }
    }

    // s, e, n, w
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    let mut pq = BinaryHeap::new();
    let mut dist = vec![vec![usize::max_value(); width]; height];

    pq.push(State { x: rx, y: ry, c: 0, o: 1, p: FxHashSet::default() });
    dist[ry][rx] = 0;

    while let Some(State { c, x, y, o, p }) = pq.pop() {
        if x == sx && y == sy { return c; }
        // println!("Looking at ({}, {})", x, y);

        for d in 0..4 {
            let nx = (x as i32 + dx[d]) as usize;
            let ny = (y as i32 + dy[d]) as usize;
            // println!("   Looking at ({}, {})", nx, ny);
            if nx < width && ny < height {
                // println!("    In range");
                // println!("{}, {}", map[ny][nx], dist[ny][nx]);
                if map[ny][nx] != '#' && dist[ny][nx] > c + 1 {
                    dist[ny][nx] = c + 1;
                    let cost = if o == d { 1 } else { 1001 };
                    pq.push(State { x: nx, y: ny, c: c + cost, o: d, p: p.clone() });
                }
            }
        }
    }

    0
}

#[advent_of_code(2024, 16, 2)]
pub fn solve_level2(input: &[&str]) -> usize {
    let map: Vec<Vec<char>> = input.iter().map(|&line| line.chars().collect()).collect();

    let width = map[0].len();
    let height = map.len();

    let mut sx = 0; let mut sy = 0;
    let mut ex = 0; let mut ey = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 'E' {
                ex = x; ey = y;
            }
            if map[y][x] == 'S' {
                sx = x; sy = y;
            }
        }
    }

    // s, e, n, w
    let dx = [0, 1, 0, -1];
    let dy = [1, 0, -1, 0];

    let mut pq = BinaryHeap::new();
    let mut dist = vec![vec![vec![usize::max_value(); 4]; width]; height];

    let mut path = FxHashSet::default();
    path.insert((ex, ey));
    path.insert((sx, sy));
    pq.push(State { x: sx, y: sy, c: 0, o: 1, p: path.clone() });
    dist[sy][sx][1] = 0;

    let mut best_path_cost = usize::max_value();
    let mut all_paths: FxHashSet<(usize, usize)> = FxHashSet::default();
    while let Some(State { c, x, y, o, p }) = pq.pop() {
        // println!("Looking at ({}, {}), dir: {}, cost: {}", x, y, o, c);
        if x == ex && y == ey { // this is one of the best paths
            if best_path_cost == usize::max_value() { best_path_cost = c };
            if c == best_path_cost {
                // println!("Found a path!");
                all_paths.extend(&p);
            }
        }

        for d in 0..4 {
            let nx = (x as i32 + dx[d]) as usize;
            let ny = (y as i32 + dy[d]) as usize;
            // println!("   Looking at ({}, {})", nx, ny);
            if nx < width && ny < height {
                // println!("    In range");
                // println!("{}, {}", map[ny][nx], dist[ny][nx]);
                if map[ny][nx] != '#' {
                    let cost = if o == d { 1 } else { 1001 };
                    // println!("      Will cost {} (known: {})", c + cost, dist[ny][nx][d]);
                    if dist[ny][nx][d] >= c + cost {
                        // println!("     Pushed ({}, {})", nx, ny);
                        let mut pp = p.clone();
                        pp.insert((nx, ny));
                        dist[ny][nx][d] = c + cost;
                        pq.push(State { x: nx, y: ny, c: c + cost, o: d, p: pp });
                    }
                }
            }
        }
    }

    all_paths.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/16-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/16-sample.out").unwrap());

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
