#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;

#[advent_of_code(2024, 1, 1)]
pub fn solve_level1(input: &[&str]) -> String {
    let parsed: Vec<(i32, i32)> = input.iter()
         .map(|s| scan_fmt!(*s, "{} {}", i32, i32).unwrap())
        .collect();

    let mut first_list: Vec<i32> = parsed.iter().map(|(f, _)| *f).collect();
    let mut second_list: Vec<i32> = parsed.iter().map(|(_, s)| *s).collect();

    first_list.sort();
    second_list.sort();

    first_list.iter().zip(second_list)
                     .map(|(f, s)| i32::abs(*f - s))
                     .sum::<i32>().to_string()
}

#[advent_of_code(2024, 1, 2)]
pub fn solve_level2(input: &[&str]) -> String {
    let parsed: Vec<(i32, i32)> = input.iter()
         .map(|s| scan_fmt!(*s, "{} {}", i32, i32).unwrap())
        .collect();

    let first_list: Vec<i32> = parsed.iter().map(|(f, _)| *f).collect();
    let second_list: Vec<i32> = parsed.iter().map(|(_, s)| *s).collect();

    first_list.iter()
        .map(|f| second_list.iter().filter(|s| *s == f).count() as i32 * (*f))
        .sum::<i32>()
        .to_string()
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
        assert_eq!(solve_level1(&input), expected);
    }

    #[test]
    fn test_level2() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().skip(3).next().unwrap();
        assert_eq!(solve_level2(&input), expected);
    }
}
