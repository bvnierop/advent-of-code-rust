#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

type Point = (usize, usize);

#[advent_of_code(2024, 12, 1)]
pub fn solve_level1(input: &[&str]) -> u64 {
    let garden: Vec<Vec<_>> = input.iter().map(|&line| line.chars().collect()).collect();
    let width = garden[0].len();
    let height = garden.len();

    let mut seen = vec![vec![false; width]; height];

    let dx: [i32; 4] = [ -1, 0, 1, 0 ];
    let dy: [i32; 4] = [ 0, -1, 0, 1 ];

    let mut total = 0;

    for x in 0..width {
        for y in 0..height {
            if seen[y][x] { continue; }

            let mut area = 0;
            let mut perimiter = 0;
            let mut q: VecDeque<Point> = VecDeque::new();
            q.push_back((x, y));
            seen[y][x] = true;
            area += 1;
            // update perimiter
            for d in 0..4 {
                let nnx = (x as i32 + dx[d]) as usize;
                let nny = (y as i32 + dy[d]) as usize;
                if  nnx >= width || nny >= height || garden[y][x] != garden[nny][nnx] { perimiter += 1; }
            } // for d...

            while let Some((cx, cy)) = q.pop_front() {

                // check neighbours
                for d in 0..4 {
                    let nx = (cx as i32 + dx[d]) as usize;
                    let ny = (cy as i32 + dy[d]) as usize;
                    if nx < width && ny < height && !seen[ny][nx] && garden[ny][nx] == garden[y][x] {
                        q.push_back((nx, ny));
                        seen[ny][nx] = true;
                        area += 1;
                        // update perimiter
                        for d in 0..4 {
                            let nnx = (nx as i32 + dx[d]) as usize;
                            let nny = (ny as i32 + dy[d]) as usize;
                            if  nnx >= width || nny >= height || garden[ny][nx] != garden[nny][nnx] { perimiter += 1; }
                        } // for d...
                    }
                } // for d

            } // while q
            // println!("Found region for {}: area {}, perimiter {}", garden[y][x], area, perimiter);
            total += area * perimiter;
        } // for y
    } // for x


    total
}

#[advent_of_code(2024, 12, 2)]
pub fn solve_level2(input: &[&str]) -> u64 {
    let garden: Vec<Vec<_>> = input.iter().map(|&line| line.chars().collect()).collect();
    let width = garden[0].len();
    let height = garden.len();

    let mut seen = vec![vec![false; width]; height];

    // unusual order: west, east, north, south
    let dx: [i32; 4] = [ -1, 1, 0, 0 ];
    let dy: [i32; 4] = [ 0, 0, -1, 1 ];

    let mut total = 0;


    for y in 0..height {
        for x in 0..width {
            if seen[y][x] { continue; }

            let mut area = 0;
            let mut perimiter: Vec<Vec<Point>> = vec![vec![]; 4];
            let mut q: VecDeque<Point> = VecDeque::new();
            q.push_back((x, y));
            seen[y][x] = true;
            area += 1;
            // update sides
            // as either the top tile, or the left tile, or both of an area, this tile
            // _always_ increments the sides
            for d in 0..4 {
                let nnx = (x as i32 + dx[d]) as usize;
                let nny = (y as i32 + dy[d]) as usize;
                if  nnx >= width || nny >= height || garden[y][x] != garden[nny][nnx] {
                    perimiter[d].push((x, y));
                }
            } // for d...

            while let Some((cx, cy)) = q.pop_front() {

                // check neighbours
                for d in 0..4 {
                    let nx = (cx as i32 + dx[d]) as usize;
                    let ny = (cy as i32 + dy[d]) as usize;
                    if nx < width && ny < height && !seen[ny][nx] && garden[ny][nx] == garden[y][x] {
                        q.push_back((nx, ny));
                        seen[ny][nx] = true;
                        area += 1;
                        // update perimiter
                        for d in 0..4 {
                            let nnx = (nx as i32 + dx[d]) as usize;
                            let nny = (ny as i32 + dy[d]) as usize;
                            if  nnx >= width || nny >= height || garden[ny][nx] != garden[nny][nnx] {
                                perimiter[d].push((nx, ny));
                            }
                        } // for d...
                    }
                } // for d
            } // while q

            let mut sides = 0;
            for d in 0..2 {
                let mut lookup: FxHashSet<Point> = perimiter[d].iter().copied().collect();

                for i in 0..perimiter[d].len() {
                    let c = perimiter[d][i];
                    if !lookup.contains(&c) { continue; } // we already collapsed this point
                    lookup.remove(&c);

                    sides += 1;

                    // look at neighbours on the X-axis
                    let (cx, cy) = c;
                    for dd in 2..4 {
                        let mut nx = (cx as i32 + dx[dd]) as usize;
                        let mut ny = (cy as i32 + dy[dd]) as usize;

                        while lookup.contains(&(nx, ny)) {
                            lookup.remove(&(nx, ny));
                            nx = (nx as i32 + dx[dd]) as usize;
                            ny = (ny as i32 + dy[dd]) as usize;
                        }
                    }
                }
            }

            for d in 2..4 {
                let mut lookup: FxHashSet<Point> = perimiter[d].iter().copied().collect();

                for i in 0..perimiter[d].len() {
                    let c = perimiter[d][i];
                    if !lookup.contains(&c) { continue; } // we already collapsed this point
                    lookup.remove(&c);

                    sides += 1;

                    // look at neighbours on the Y-axis
                    let (cx, cy) = c;
                    for dd in 0..2 {
                        let mut nx = (cx as i32 + dx[dd]) as usize;
                        let mut ny = (cy as i32 + dy[dd]) as usize;


                        while lookup.contains(&(nx, ny)) {
                            lookup.remove(&(nx, ny));
                            nx = (nx as i32 + dx[dd]) as usize;
                            ny = (ny as i32 + dy[dd]) as usize;
                        }
                    }
                }
            }
            total += area * sides;
        } // for y
    } // for x

    total as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/12-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/12-sample.out").unwrap());

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
