#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;

#[advent_of_code(2024, 1, 1)]
pub fn solve_level1(input: &[&str]) -> i32 {
    let file: Vec<(i32, i32)> =
        input.into_iter()
             .map(|s| scan_fmt!(s, "{} {}", i32, i32).unwrap())
             .collect();

    let (mut first, mut second): (Vec<i32>, Vec<i32>) = file.into_iter().unzip();

    first.sort();
    second.sort();

    first.into_iter().zip(second)
        .map(|(f, s)| (f - s).abs())
        .sum()
}

#[advent_of_code(2024, 1, 2)]
pub fn solve_level2(input: &[&str]) -> i32 {
    let file: Vec<(i32, i32)> = input.iter()
         .map(|s| scan_fmt!(*s, "{} {}", i32, i32).unwrap())
        .collect();

    let (first, second): (Vec<i32>, Vec<i32>) = file.into_iter().unzip();

    first.iter()
        .map(|f| second.iter().filter(|s| *s == f).count() as i32 * (*f))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/01-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/01-sample.out").unwrap());

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
