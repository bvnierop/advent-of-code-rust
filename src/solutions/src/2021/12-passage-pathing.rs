#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use std::collections::VecDeque;

#[advent_of_code(2021, 12, 1)]
pub fn solve_level1(input: &[&str]) -> i32 {
    // build adjacency list
    //   paths[src].contains(dst);
    let mut paths: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();
    for line in input {
        let (from, to) = scan_fmt!(line, "{}-{}", String, String).expect("Expected direction");
        paths.entry(from.clone()).or_default().insert(to.clone());
        paths.entry(to).or_default().insert(from);
    }

    let mut q: VecDeque<(String, FxHashSet<String>)> = VecDeque::new();
    let s: FxHashSet<String> = ["start".to_string()].iter().cloned().collect();
    q.push_back(("start".to_string(), s));
    let mut path_count = 0;
    while let Some((cave, seen)) = q.pop_front() {
        if cave == "end" {
            path_count += 1;
        } else {
            for n in paths[&cave].iter() {
                if *n == n.to_uppercase() {
                    q.push_back((n.clone(), seen.clone()))
                } else if !seen.contains(n) {
                    let mut s = seen.clone();
                    s.insert(n.clone());
                    q.push_back((n.clone(), s))
                }
            }
        }
    }
    path_count
}

#[advent_of_code(2021, 12, 2)]
pub fn solve_level2(input: &[&str]) -> i32 {
    // build adjacency list
    //   paths[src].contains(dst);
    let mut paths: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();
    for line in input {
        let (from, to) = scan_fmt!(line, "{}-{}", String, String).expect("Expected direction");
        paths.entry(from.clone()).or_default().insert(to.clone());
        paths.entry(to).or_default().insert(from);
    }

    let mut q: VecDeque<(String, bool, Vec<String>, FxHashSet<String>)> = VecDeque::new();
    let s: FxHashSet<String> = ["start".to_string()].iter().cloned().collect();
    q.push_back(("start".to_string(), false, vec![], s));
    let mut taken_paths = FxHashSet::default();
    while let Some((cave, vis_small, taken_path, seen)) = q.pop_front() {
        if cave == "end" {
            taken_paths.insert(taken_path);
        } else {
            for n in paths[&cave].iter() {
                let mut tp = taken_path.clone();
                tp.push(n.clone());
                if *n == n.to_uppercase() {
                    q.push_back((n.clone(), vis_small, tp, seen.clone()));
                } else if !seen.contains(n) {
                    if !vis_small { // we have not yet visited a small cave for free
                        q.push_back((n.clone(), true, tp.clone(), seen.clone())); // visit a small cave for free
                        let mut s = seen.clone(); // visit the small cave
                        s.insert(n.clone());
                        q.push_back((n.clone(), false, tp, s))
                    } else {
                        let mut s = seen.clone();
                        s.insert(n.clone());
                        q.push_back((n.clone(), vis_small, tp, s));
                    }
                }
            }
        }
    }
    taken_paths.len() as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/12-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/12-sample.out").unwrap());

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
