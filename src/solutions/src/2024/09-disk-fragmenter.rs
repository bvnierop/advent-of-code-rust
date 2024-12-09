#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[advent_of_code(2024, 9, 1)]
pub fn solve_level1(input: &[&str]) -> u64 {
    let digits: Vec<_> = input[0].chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
    let total = digits.iter().sum::<u64>();

    let mut mem: Vec<i32> = vec![-1; total as usize];

    // fill memory
    let mut cur = 0;
    let mut is_empty = false;
    let mut index = 0;
    for digit in digits {
        if !is_empty {
            for i in index..index+digit {
                mem[i as usize] = cur;
            }
            cur += 1;
        }
        index += digit;
        is_empty = !is_empty;
    }

    // look at right / left and move each one.
    let mut left = 0; let mut right = mem.len() - 1;
    while left < right {
        // look for the next block to move
        while mem[right] == -1 {
            right -= 1;
        }
        // look for the next free space
        while mem[left] != -1 {
            left += 1;
        }
        if left < right {
            mem[left] = mem[right];
            mem[right] = -1;
        }
    }

    mem.iter().copied().filter(|&x| x != -1)
        .enumerate()
        .map(|(i, m)| i as u64 * m as u64)
        .sum()
}

#[advent_of_code(2024, 9, 2)]
pub fn solve_level2(input: &[&str]) -> u64 {
    let digits: Vec<_> = input[0].chars().map(|c| c.to_digit(10).unwrap() as u64).collect();
    let total = digits.iter().sum::<u64>();

    let mut mem: Vec<i32> = vec![-1; total as usize];

    // fill memory
    let mut file_id = 0;
    let mut is_empty = false;
    let mut index = 0;
    for digit in digits {
        if !is_empty {
            for i in index..index+digit {
                mem[i as usize] = file_id;
            }
            file_id += 1;
        }
        index += digit;
        is_empty = !is_empty;
    }
    file_id -= 1;

    // look at right / left and move each one.
    let mut right: i32 = (mem.len() - 1) as i32;
    while file_id >= 0 {
        // search back for file
        if mem[right as usize] != file_id {
            while mem[right as usize] != file_id {
                right -= 1;
            }
        }
        let mut fs = 0;
        while right >= 0 && mem[right as usize] == file_id {
            right -= 1;
            fs += 1;
        }
        right += 1;

        // look for a block to move it in
        let mut left = 0;
        loop {
            while mem[left] != -1 {
                left += 1;
            }
            let start = left;
            let mut es = 0;
            while left < mem.len() && mem[left] == -1 {
                left += 1;
                es += 1;
            }

            left = start;
            if es >= fs || left > right as usize {
                break;
            }
            left += 1;


        // at this point, `left` is the start of the empty block,
        // and `right` is the start of the file.
        if left < right as usize {
            for i in 0..fs {
                mem[left + i] = file_id;
                mem[right as usize + i] = -1;
            }
        }

        file_id -= 1;
    }

    mem.iter().copied()
        .enumerate()
        .filter(|(_i, x)| *x != -1)
        .map(|(i, m)| i as u64 * m as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/09-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/09-sample.out").unwrap());

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
