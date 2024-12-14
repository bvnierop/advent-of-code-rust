#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use rustc_hash::FxHashSet;


fn parse(line: &str) -> ((i32, i32), (i32, i32)) {
    let (x1,y1,x2,y2) = scan_fmt!(line, "p={},{} v={},{}", i32, i32, i32, i32).unwrap();
    ((x1, y1), (x2, y2))
}

#[advent_of_code(2024, 14, 1)]
pub fn solve_level1(input: &[&str]) -> i64 {
    let mut robots: Vec<_> = input.iter().map(|&line| parse(line)).collect();

    let width = 101;
    let height = 103;

    // let height = 7;
    // let width = 11;

    for _sec in 0..100 {
        robots = robots.iter().copied().map(|((x, y), (dx, dy))| {
            let nx = (x + dx + width) % width;
            let ny = (y + dy + height) % height;
            ((nx, ny), (dx, dy))
        }).collect();
    }

    let mut q1 = 0; let mut q2 = 0; let mut q3 = 0; let mut q4 = 0;
    for ((x, y), (_, _)) in robots {
        if x < width / 2 && y < height / 2 { q1 += 1 }
        if x < width / 2 && y > height / 2 { q2 += 1 }
        if x > width / 2 && y < height / 2 { q3 += 1 }
        if x > width / 2 && y > height / 2 { q4 += 1 }
    }

    q1 * q2 * q3 * q4
}

#[advent_of_code(2024, 14, 2)]
pub fn solve_level2(input: &[&str]) -> u32 {
    let mut robots: Vec<_> = input.iter().map(|&line| parse(line)).collect();

    let width = 101;
    let height = 103;

    // let height = 7;
    // let width = 11;

    let mut secs = 0;
    loop {
        secs += 1;
        robots = robots.iter().copied().map(|((x, y), (dx, dy))| {
            let nx = (x + dx + width) % width;
            let ny = (y + dy + height) % height;
            ((nx, ny), (dx, dy))
        }).collect();

        let seen: FxHashSet<(i32, i32)> = robots.iter().copied().map(|((x, y), (_dx, _dy))| (x, y)).collect();

        if seen.len() == robots.len() { break; }

    }

    // for y in 0..height {
    //     for x in 0..width {
    //         let mut found = false;
    //         for ((rx, ry), (_, _)) in robots.clone() {
    //             if rx == x && ry == y {found = true; }
    //         }
    //         if found { print!("#"); }
    //         else { print!("."); }
    //     }
    //     println!("");
    // }

    secs
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/14-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/14-sample.out").unwrap());

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
