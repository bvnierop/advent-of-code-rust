#![allow(unused_imports)]
#![allow(unused_variables)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;

pub fn to_heights(lk: &Vec<Vec<char>>, chr: char) -> Vec<usize> {
    let mut v: Vec<usize> = vec![];
    let w = lk[0].len();
    let h = lk.len();

    for x in 0..w {
        let mut count = 0;
        for y in 0..h {
            if lk[y][x] == chr {
                count += 1
            }
        }
        v.push(count);
    }
    v
}

#[advent_of_code(2024, 25, 1)]
pub fn solve_level1(input: &[&str]) -> u32 {
    let mut splitted = input.split(|l| l.is_empty());
    let mut keys = vec![];
    let mut locks = vec![];
    while let Some(lk) = splitted.next() {
        let x = lk.iter().map(|l| l.chars().collect_vec()).collect_vec();
        if x[0][0] == '.' {
            keys.push(to_heights(&x, '#'));
        } else {
            locks.push(to_heights(&x, '.'));
        }
    }

    let mut ans = 0;
    for k in keys.clone() {
        for l in locks.clone() {
            let mut good = true;
            for x in 0..k.len() {
                if k[x] > l[x] {
                    good = false;
                }
            }
            if good { ans += 1; }
        }
    }

    ans
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/25-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/25-sample.out").unwrap());

    #[test]
    fn test_level1() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().next().unwrap();
        assert_eq!(format!("{}", solve_level1(&input)), expected);
    }
}
