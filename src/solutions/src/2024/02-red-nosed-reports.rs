#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;

pub fn parse_report(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|n| n.parse::<i32>().unwrap())
        .collect()
}

pub fn verify_report(report: &Vec<i32>) -> bool {
    report.windows(2).all(|pair| {pair[0] - pair[1] <= 3 && pair[0] - pair[1] > 0}) ||
    report.windows(2).all(|pair| {pair[1] - pair[0] <= 3 && pair[1] - pair[0] > 0})
}

pub fn verify_report_with_safety_dampener(report: &Vec<i32>) -> bool {
    verify_report(&report) ||
    (0..report.len()).any(|skip| {
        let rep: Vec<_> =
            report.iter()
                .enumerate()
                .filter_map(|(idx, &level)| if idx != skip { Some(level) } else { None })
                .collect();
        verify_report(&rep)
    })
}

#[advent_of_code(2024, 2, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let reports: Vec<Vec<i32>> = input.iter().map(|&line| parse_report(line)).collect();
    let valid = reports.iter().filter(|r| verify_report(r));

    valid.count()
}

#[advent_of_code(2024, 2, 2)]
pub fn solve_level2(input: &[&str]) -> usize {
    let reports: Vec<Vec<i32>> = input.iter().map(|&line| parse_report(line)).collect();
    let valid = reports.iter().filter(|r| verify_report_with_safety_dampener(r));

    valid.count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/02-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/02-sample.out").unwrap());

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

    #[test]
    fn test_verify() {
        assert_eq!(verify_report(&vec![1, 3, 2, 4, 5]), false);
    }
}
