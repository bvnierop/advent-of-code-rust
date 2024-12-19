#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use im::{Vector,vector};
use memoize::memoize;
use rustc_hash::FxHashMap;

fn possible(requested: String, available: &Vec<String>) -> bool {
    if requested.is_empty() {
        return true;
    }

    let mut found = false;
    for a in available {
        if requested.starts_with(a) {
            if possible(requested[a.len()..].to_string(), available) {
                found = true;
                break;
            }
        }
    }
    found
}

#[memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default())]
fn possible2(requested: String, available: Vector<String>) -> u64 {
    if requested.is_empty() {
        return 1;
    }

    let mut found = 0;
    for a in available.clone() {
        if requested.starts_with(&a) {
            found += possible2(requested[a.len()..].to_string(), available.clone());
        }
    }
    found
}

#[advent_of_code(2024, 19, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let patterns_available: Vec<String> = input[0].split(", ").map(|word| word.to_string()).collect();
    let patterns_requested: Vec<String> = input.iter().skip(2).map(|&line| line.to_string()).collect();

    // println!("{:?}", patterns_available);

    patterns_requested.iter().filter(|r| possible((**r).clone(), &patterns_available)).count()
}

#[advent_of_code(2024, 19, 2)]
pub fn solve_level2(input: &[&str]) -> u64 {
    let patterns_available: Vec<String> = input[0].split(", ").map(|word| word.to_string()).collect();
    let patterns_requested: Vec<String> = input.iter().skip(2).map(|&line| line.to_string()).collect();

    // println!("{:?}", patterns_available);

    patterns_requested.iter().map(|r| possible2((*r).clone(), Vector::from(&patterns_available))).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/19-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/19-sample.out").unwrap());

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
