#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use regex::Regex;

#[advent_of_code(2024, 3, 1)]
pub fn solve_level1(input: &[&str]) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum = 0;
    for (_, [multiplicand, multiplier]) in re.captures_iter(&input.join("")).map(|c| c.extract()) {
        let m1 = multiplicand.parse::<u32>().unwrap();
        let m2 = multiplier.parse::<u32>().unwrap();
        sum += m1 * m2;
    }
    sum
}

#[advent_of_code(2024, 3, 2)]
pub fn solve_level2(input: &[&str]) -> u32 {
    let re = Regex::new(r"(mul\((\d{1,3}),(\d{1,3})\)|do\(()()\)|don't\(()()\))").unwrap();

    let mut sum = 0;
    let mut enabled = true;
    for c in re.captures_iter(&input.join("")) {
        match c.extract() {
            ("do()", [_, _, _]) => enabled = true,
            ("don't()", [_, _, _]) => enabled = false,
            (_, [_, multiplicand, multiplier]) => {
                let m1 = multiplicand.parse::<u32>().unwrap();
                let m2 = multiplier.parse::<u32>().unwrap();
                if enabled {
                    sum += m1 * m2;
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/03-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/03-sample.out").unwrap());

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
