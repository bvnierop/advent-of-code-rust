#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::collections::HashMap;

#[advent_of_code(2024, 11, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let mut stones: Vec<_> = input.join("").split_whitespace().map(|num| num.parse::<u128>().unwrap()).collect();

    for _blink in 0..25 {
        let mut new_stones: Vec<u128> = Vec::with_capacity(stones.len());

        for stone in stones.iter().copied() {
            if stone == 0 {
                new_stones.push(1)
            } else {
                let len = stone.ilog10() + 1;
                if len % 2 == 0 {
                    let div = 10u128.pow(len / 2);
                    new_stones.push(stone / div);
                    new_stones.push(stone % div);
                    // println!("Split {} into {} and {}", stone, stone / div, stone % div);
            } else {
                new_stones.push(stone * 2024);
            }}
        }

        stones = new_stones;
    }

    stones.len()
}

fn solve_single_stone(stone: u128, step: u8, max_steps: u8, cache: &mut HashMap<(u128, u8), u128>) -> u128 {
    if step == max_steps {
        return 1;
    }

    if cache.contains_key(&(stone, step)) { return cache[&(stone, step)]; }

    let res =
        if stone == 0 {
            solve_single_stone(1, step + 1, max_steps, cache)
        } else {
            let len = stone.ilog10() + 1;
            if len % 2 == 0 {
                let div = 10u128.pow(len / 2);
                solve_single_stone(stone / div, step + 1, max_steps, cache) +
                    solve_single_stone(stone % div, step + 1, max_steps, cache)

            } else {
                solve_single_stone(stone * 2024, step + 1, max_steps, cache)
            }
        };
    cache.insert((stone, step), res);
    res
}

#[advent_of_code(2024, 11, 2)]
pub fn solve_level2(input: &[&str]) -> u128 {
    let stones: Vec<_> = input.join("").split_whitespace().map(|num| num.parse::<u128>().unwrap()).collect();

    let mut sum = 0;
    let mut cache: HashMap<(u128, u8), u128> = HashMap::new();
    for stone in stones {
        sum += solve_single_stone(stone, 0, 75, &mut cache);
    }

    sum
}

#[advent_of_code(2024, 11, 2)]
pub fn solve_with_counts(input: &[&str]) -> u128 {
    let stones: Vec<_> = input.join("").split_whitespace().map(|num| num.parse::<u128>().unwrap()).collect();
    let mut counts: HashMap<u128, u128> = HashMap::new();

    for stone in stones {
        *counts.entry(stone).or_default() += 1;
    }

    for _blink in 0..75 {
        let mut new_counts: HashMap<u128, u128> = HashMap::new();
        for stone in counts.keys().copied() {
            if stone == 0 {
                *new_counts.entry(1u128).or_default() += counts[&stone];
            } else {
                let len = stone.ilog10() + 1;
                if len % 2 == 0 {
                    let div = 10u128.pow(len / 2);
                    let p1 = stone / div;
                    let p2 = stone % div;

                    *new_counts.entry(p1).or_default() += counts[&stone];
                    *new_counts.entry(p2).or_default() += counts[&stone];
            } else {
                *new_counts.entry(stone * 2024).or_default() += counts[&stone];
            }}
        }
        counts = new_counts;
    }

    counts.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/11-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/11-sample.out").unwrap());

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
    fn test_solve_with_counts() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().skip(3).next().unwrap();
        assert_eq!(format!("{}", solve_with_counts(&input)), expected);
    }
}
