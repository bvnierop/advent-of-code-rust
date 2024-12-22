#![allow(unused_imports)]
#![allow(unused_variables)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use im::{Vector, vector};
use memoize::memoize;
use rustc_hash::FxHashMap;

const MOD: i128 = 16777216;

fn next(cur: i128) -> i128 {
    let p1 = (cur ^ (cur * 64)) % MOD;
    let p2 = (p1 ^ (p1 / 32)) % MOD;
    let p3 = (p2 ^ (p2 * 2048)) % MOD;
    p3
}

#[advent_of_code(2024, 22, 1)]
pub fn solve_level1(input: &[&str]) -> i128 {
    let mut ans = 0;
    for nums in input {
        let num = nums.parse::<i128>().expect("number");
        let mut n = num;
        for i in 0..2000 {
            n = next(n);
        }
        ans += n;
    }
    ans
}

fn create_map_for_buyer(start: i128) -> FxHashMap<u32, i8> {
    let mut map = FxHashMap::default();

    let mut secret = start;
    let mut encoded_steps = 0 as u32;
    for i in 0..2000 {
        let p1 = (secret % 10) as i8;
        secret = next(secret);
        let p2 = (secret % 10) as i8;
        let diff = p2 - p1;

        // to encode:
        //    shift 5, OR enc, AND 0b11111111111111111111
        let enc = (diff + 10) as u32;
        encoded_steps <<= 5;
        encoded_steps |= enc;
        encoded_steps &= 2u32.pow(20) - 1;

        if !map.contains_key(&encoded_steps) && i > 3 { // monkey only buys first time
            map.insert(encoded_steps, p2);
        }
    }

    map
}

#[advent_of_code(2024, 22, 2)]
pub fn solve_level2(input: &[&str]) -> u32 {
    let buyers: Vector<i128> = input.into_iter().map(|l| l.parse::<i128>().unwrap()).collect();

    let maps = buyers.into_iter().map(create_map_for_buyer);
    // merge maps
    let mut grand_map: FxHashMap<u32, u32> = FxHashMap::default();

    for map in maps {
        for (k, v) in map.into_iter() {
            *grand_map.entry(k).or_default() += v as u32;
        }
    }

    let mut ans = 0;
    for i in 0..=2u32.pow(20) {
        if grand_map.contains_key(&i) {
            ans = ans.max(grand_map[&i]);
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/22-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/22-sample.out").unwrap());

    static SAMPLE2: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/22-sample2.in").unwrap());
    static SAMPLE2_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/22-sample2.out").unwrap());
    #[test]
    fn test_level1() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().next().unwrap();
        assert_eq!(format!("{}", solve_level1(&input)), expected);
    }

    #[test]
    fn test_level2() {
        let input: Vec<_> = (*SAMPLE2).lines().collect();
        let expected = (*SAMPLE2_OUT).lines().skip(3).next().unwrap();
        assert_eq!(format!("{}", solve_level2(&input)), expected);
    }
}
