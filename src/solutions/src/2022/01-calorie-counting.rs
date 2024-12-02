#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;

fn parse_elf(lines: &[&str]) -> u32 {
    lines.iter().map(|&line| line.parse::<u32>().unwrap())
        .sum()
}

#[advent_of_code(2022, 1, 1)]
pub fn solve_level1(input: &[&str]) -> u32 {
    input.split(|&line| line.is_empty())
        .map(parse_elf)
        .max()
        .unwrap()
}

#[advent_of_code(2022, 1, 2)]
pub fn solve_level2(input: &[&str]) -> u32 {
    input.split(|&line| line.is_empty())
        .map(parse_elf)
        .sorted()
        .rev()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2022/01-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2022/01-sample.out").unwrap());

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
