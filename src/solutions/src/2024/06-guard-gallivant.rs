#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::collections::HashSet;

#[advent_of_code(2024, 6, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    // build grid
    let map: Vec<Vec<_>> = input.iter().map(|&line| line.chars().collect()).collect();

    // find guard
    let mut gx: i32 = 0; let mut gy: i32 = 0;
    for (y, row) in map.iter().cloned().enumerate() {
        for (x, cell) in row.iter().copied().enumerate() {
            if cell == '^' { gx = x as i32; gy = y as i32 }
        }
    }

    // north, east, south, west
    let dx: [i32; 4] = [0, 1, 0, -1];
    let dy: [i32; 4] = [-1, 0, 1, 0];

    // simulate
    let mut dir = 0;
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    loop {
        seen.insert((gx, gy));

        // step: move forward if possible
        let nx = gx + dx[dir];
        let ny = gy + dy[dir];

        // We're done if we're out of bounds
        if nx < 0 || nx >= map[0].len() as i32 || ny < 0 || ny >= map.len() as i32 { break; }
        let ahead = map[ny as usize][nx as usize];

        // turn if blocked
        if ahead == '#' {
            dir = (dir + 1) % 4;
        } else {
            gx = nx; gy = ny;
        }
    }
    seen.len()
}

#[advent_of_code(2024, 6, 2)]
pub fn solve_level2(input: &[&str]) -> usize {
    // build grid
    let map: Vec<Vec<_>> = input.iter().map(|&line| line.chars().collect()).collect();
    // find guard
    let mut gx: i32 = 0; let mut gy: i32 = 0;
    for (y, row) in map.iter().cloned().enumerate() {
        for (x, cell) in row.iter().copied().enumerate() {
            if cell == '^' { gx = x as i32; gy = y as i32 }
        }
    }

    let ogx = gx; let ogy = gy;

    // north, east, south, west
    let dx: [i32; 4] = [0, 1, 0, -1];
    let dy: [i32; 4] = [-1, 0, 1, 0];

    let mut dir = 0;
    let mut seen: HashSet<(i32, i32)> = HashSet::new();
    loop {
        seen.insert((gx, gy));

        // step: move forward if possible
        let nx = gx + dx[dir];
        let ny = gy + dy[dir];

        // We're done if we're out of bounds
        if nx < 0 || nx >= map[0].len() as i32 || ny < 0 || ny >= map.len() as i32 { break; }
        let ahead = map[ny as usize][nx as usize];

        // turn if blocked
        if ahead == '#' {
            dir = (dir + 1) % 4;
        } else {
            gx = nx; gy = ny;
        }
    }

    // simulate all the options
    let mut cycles = 0;
    for (x, y) in seen {
        // restore state
        gx = ogx; gy = ogy;
        let mut dir = 0;
        let mut seen: HashSet<(i32, i32, usize)> = HashSet::new();
        loop {
            // if we've seen the current position, we can move on
            if seen.contains(&(gx, gy, dir)) { cycles += 1; break; }
            seen.insert((gx, gy, dir));

            // step: move forward if possible
            let nx = gx + dx[dir];
            let ny = gy + dy[dir];

            // We're done if we're out of bounds
            if nx < 0 || nx >= map[0].len() as i32 || ny < 0 || ny >= map.len() as i32 { break; }
            let mut ahead = map[ny as usize][nx as usize];
            if ny == y as i32 && nx == x as i32 { ahead = '#' }

            // turn if blocked
            if ahead == '#' {
                dir = (dir + 1) % 4;
            } else {
                gx = nx; gy = ny;
            }
        }
    }
    cycles
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/06-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/06-sample.out").unwrap());

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
