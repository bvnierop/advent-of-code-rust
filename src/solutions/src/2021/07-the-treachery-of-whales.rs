#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::cmp::min;

#[advent_of_code(2021, 7, 1)]
pub fn solve_level1(input: &[&str]) -> i32 {
    let positions: Vec<_> = input[0].split(",").map(|s| s.parse::<i32>().unwrap()).collect();

    solve(&positions, std::convert::identity)
}

fn solve(positions: &Vec<i32>, cost_fn: fn(i32) -> i32) -> i32 {
    let min_pos = *positions.iter().min().unwrap();
    let max_pos = *positions.iter().max().unwrap();

    let mut best = i32::max_value();
    for target_pos in min_pos..=max_pos {
        let mut cost = 0;
        for source_pos in positions {
            cost += cost_fn((source_pos - target_pos).abs());
        }
        best = min(best, cost);
    }

    best
}

#[advent_of_code(2021, 7, 2)]
pub fn solve_level2(input: &[&str]) -> i32 {
    let positions: Vec<_> = input[0].split(",").map(|s| s.parse::<i32>().unwrap()).collect();

    solve(&positions, |d| (d * (d + 1)) / 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/07-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/07-sample.out").unwrap());

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
