#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

#[advent_of_code(2024, 5, 1)]
pub fn solve_level1(input: &[&str]) -> u32 {
    let rules_input = input.iter().take_while(|&line| !line.is_empty());
    let updates_input = input.iter().skip_while(|&line| !line.is_empty()).skip(1);

    let rules: Vec<_> = rules_input.map(|&rule| scan_fmt!(rule, "{}|{}", u32, u32).unwrap()).collect();

    let updates: Vec<Vec<_>> = updates_input.map(|&update| update.split(",").map(|page| page.parse::<u32>().unwrap()).collect()).collect();

    let mut before_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut after_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (before, after) in rules {
        before_map.entry(after).or_insert(HashSet::new()).insert(before);
        after_map.entry(before).or_insert(HashSet::new()).insert(after);
    }


    let mut sum = 0;
    for update in updates {
        let mut correct = true;
        for (index, page) in update.iter().enumerate() {
            for (index2, page2) in update.iter().enumerate() {
                if index < index2 && before_map.contains_key(page) && before_map[page].contains(page2) {
                    correct = false;
                }
            }
        }

        if correct {
            sum += update[update.len() / 2];
        }
    }

    sum
}

#[advent_of_code(2024, 5, 2)]
pub fn solve_level2(input: &[&str]) -> u32 {
    let rules_input = input.iter().take_while(|&line| !line.is_empty());
    let updates_input = input.iter().skip_while(|&line| !line.is_empty()).skip(1);

    let rules: Vec<_> = rules_input.map(|&rule| scan_fmt!(rule, "{}|{}", u32, u32).unwrap()).collect();

    let updates: Vec<Vec<_>> = updates_input.map(|&update| update.split(",").map(|page| page.parse::<u32>().unwrap()).collect()).collect();

    let mut before_map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut after_map: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (before, after) in rules {
        before_map.entry(after).or_insert(HashSet::new()).insert(before);
        after_map.entry(before).or_insert(HashSet::new()).insert(after);
    }


    let mut sum = 0;
    for update in updates {
        let mut correct = true;
        for (index, page) in update.iter().enumerate() {
            for (index2, page2) in update.iter().enumerate() {
                if index < index2 && before_map.contains_key(page) && before_map[page].contains(page2) {
                    correct = false;
                }
            }
        }

        if correct { continue; }

        let pages_set: HashSet<u32> = update.iter().copied().collect();
        let mut edge_counts: HashMap<u32, u32> = HashMap::new();
        for page in &update {
            let num_pages_before = before_map.get(page).unwrap_or(&HashSet::new()).intersection(&pages_set).count();
            edge_counts.insert(*page, num_pages_before as u32);
        }

        let mut corrected: Vec<u32> = Vec::new();
        let mut q: VecDeque<u32> = VecDeque::new();
        for page in update {
            if edge_counts[&page] == 0 {
                q.push_back(page);
            }
        }

        while !q.is_empty() {
            let cur = q.pop_front().unwrap();
            corrected.push(cur);
            for next in after_map.get(&cur).unwrap_or(&HashSet::new()).iter() {
                if edge_counts.contains_key(next) {
                    edge_counts.entry(*next).and_modify(|x| *x -= 1);
                    if edge_counts[next] == 0 {
                        q.push_back(*next);
                    }
                }
            }
        }

        sum += corrected[corrected.len() / 2];
    }

    sum
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
