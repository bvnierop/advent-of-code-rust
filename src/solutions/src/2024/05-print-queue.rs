#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::{cmp::Ordering, collections::{HashMap, HashSet, VecDeque}};

#[advent_of_code(2024, 5, 1)]
pub fn solve_level1(input: &[&str]) -> u32 {
    let rules_input = input.iter().take_while(|&line| !line.is_empty());
    let updates_input = input.iter().skip_while(|&line| !line.is_empty()).skip(1);

    let rules: Vec<_> = rules_input.map(|&rule| scan_fmt!(rule, "{}|{}", u32, u32).unwrap()).collect();

    let updates: Vec<Vec<_>> = updates_input.map(|&update| update.split(",").map(|page| page.parse::<u32>().unwrap()).collect()).collect();

    let mut before_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (before, after) in rules {
        before_map.entry(after).or_default().insert(before);
    }

    updates.iter()
           .filter(|update| update.iter().copied().is_sorted_by(|a, b| {
               !before_map.entry(*a).or_default().contains(b)
           }))
           .map(|update| update[update.len() / 2])
           .sum()
}

#[advent_of_code(2024, 5, 2)]
pub fn solve_level2(input: &[&str]) -> u32 {
    let rules_input = input.iter().take_while(|&line| !line.is_empty());
    let updates_input = input.iter().skip_while(|&line| !line.is_empty()).skip(1);

    let rules: Vec<_> = rules_input.map(|&rule| scan_fmt!(rule, "{}|{}", u32, u32).unwrap()).collect();

    let updates: Vec<Vec<_>> = updates_input.map(|&update| update.split(",").map(|page| page.parse::<u32>().unwrap()).collect()).collect();

    let mut before_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (before, after) in rules {
        before_map.entry(after).or_default().insert(before);
    }

    updates.iter()
           .filter(|update| !update.iter().copied().is_sorted_by(|a, b| {
               !before_map.get(a).map_or(false, |s| s.contains(b))
           }))
        .map(|update| update.iter().sorted_by(|a, b| {
            if a == b { Ordering::Equal }
            else if before_map.get(a).map_or(false, |s|s.contains(b)) { Ordering::Greater }
            else { Ordering::Less }
        }).collect::<Vec<_>>())
        .map(|update| update[update.len() / 2])
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/05-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/05-sample.out").unwrap());

    #[test]
    fn test_level1() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().next().unwrap();
        assert_eq!(format!("{:?}", solve_level1(&input)), expected);
    }

    #[test]
    fn test_level2() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().skip(3).next().unwrap();
        assert_eq!(format!("{}", solve_level2(&input)), expected);
    }
}
