#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;

fn solve(initial: &str, days: i32) -> u128 {
    let mut fish = [0 as u128; 9];

    for f in initial.split(",").map(|s| s.parse::<usize>().unwrap()) {
        fish[f] += 1;
    }

    for _ in 0..days {
        let spawning_fish = fish[0];
        fish.rotate_left(1);
        fish[6] += spawning_fish;
    }

    fish.iter().sum()
}

#[advent_of_code(2021, 6, 1)]
pub fn solve_level1(input: &[&str]) -> u128 {
    solve(input[0], 80)
}

#[advent_of_code(2021, 6, 2)]
pub fn solve_level2(input: &[&str]) -> u128 {
    solve(input[0], 256)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/06-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/06-sample.out").unwrap());

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
