#![allow(unused_imports)]
#![allow(unused_variables)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use im::{Vector,vector};
use memoize::memoize;
use rustc_hash::FxHashMap;

#[memoize(CustomHasher: FxHashMap, HasherInit: FxHashMap::default(), Ignore: available)]
fn possible(requested: String, available: &[&str]) -> u64 {
    if requested.is_empty() {
        return 1;
    }

    let mut found = 0;
    for a in available {
        if requested.starts_with(a) {
            found += possible(requested[a.len()..].to_string(), available);
        }
    }
    found
}

#[advent_of_code(2024, 19, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let patterns_available = input[0].split(", ").collect_vec();
    let patterns_requested = &input[2..];

    patterns_requested.iter().filter(|r| possible(r.to_string(), &patterns_available) != 0).count()
}

#[advent_of_code(2024, 19, 2)]
pub fn solve_level2(input: &[&str]) -> u64 {
    let patterns_available = input[0].split(", ").collect_vec();
    let patterns_requested = &input[2..];

    patterns_requested.iter().map(|r| possible(r.to_string(), &patterns_available)).sum()
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
