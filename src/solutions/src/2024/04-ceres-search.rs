#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use regex::Regex;

#[advent_of_code(2024, 4, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let lookup: Vec<Vec<_>> = input.iter().map(|&s| s.chars().collect()).collect();

    let mut total = 0;

    // horizontal
    for row in 0..lookup.len() {
        for col in 0..(lookup[row].len() - 3) {
            if lookup[row][col] == 'X' {
                if lookup[row][col + 1] == 'M' && lookup[row][col + 2] == 'A' && lookup[row][col + 3] == 'S' {
                    total += 1;
                }
            }
        }
    }
    for row in 0..lookup.len() {
        for col in 3..lookup[row].len()  {
            if lookup[row][col] == 'X' {
                if lookup[row][col - 1] == 'M' && lookup[row][col - 2] == 'A' && lookup[row][col - 3] == 'S' {
                    total += 1;
                }
            }
        }
    }

    // verticcal
    for row in 0..(lookup.len() - 3) {
        for col in 0..lookup[row].len()  {
            if lookup[row][col] == 'X' {
                if lookup[row + 1][col] == 'M' && lookup[row + 2][col] == 'A' && lookup[row + 3][col] == 'S' {
                    total += 1;
                }
            }
        }
    }
    for row in 3..lookup.len() {
        for col in 0..lookup[row].len()  {
            if lookup[row][col] == 'X' {
                if lookup[row - 1][col] == 'M' && lookup[row - 2][col] == 'A' && lookup[row - 3][col] == 'S' {
                    total += 1;
                }
            }
        }
    }

    // diagonal tl-br
    for row in 0..lookup.len() - 3 {
        for col in 0..lookup[row].len() - 3  {
            if lookup[row][col] == 'X' {
                if lookup[row + 1][col + 1] == 'M' && lookup[row + 2][col + 2] == 'A' && lookup[row + 3][col + 3] == 'S' {
                    total += 1;
                }
            }
        }
    }
    for row in 3..lookup.len() {
        for col in 3..lookup[row].len()  {
            if lookup[row][col] == 'X' {
                if lookup[row - 1][col - 1] == 'M' && lookup[row - 2][col - 2] == 'A' && lookup[row - 3][col - 3] == 'S' {
                    total += 1;
                }
            }
        }
    }

    // diagonal tr-bl
    for row in 0..lookup.len() - 3 {
        for col in 3..lookup[row].len() {
            if lookup[row][col] == 'X' {
                if lookup[row + 1][col - 1] == 'M' && lookup[row + 2][col - 2] == 'A' && lookup[row + 3][col - 3] == 'S' {
                    total += 1;
                }
            }
        }
    }
    for row in 3..lookup.len() {
        for col in 0..lookup[row].len() - 3  {
            if lookup[row][col] == 'X' {
                if lookup[row - 1][col + 1] == 'M' && lookup[row - 2][col + 2] == 'A' && lookup[row - 3][col + 3] == 'S' {
                    total += 1;
                }
            }
        }
    }
    total
}

#[advent_of_code(2024, 4, 2)]
pub fn solve_level2(input: &[&str]) -> i32 {
    let lookup: Vec<Vec<_>> = input.iter().map(|&s| s.chars().collect()).collect();

    let mut total = 0;
    for row in 1..(lookup.len() - 1) {
        for col in 1..(lookup[row].len() - 1) {
            if lookup[row][col] == 'A' {
                match (lookup[row-1][col-1], lookup[row+1][col+1], lookup[row-1][col+1], lookup[row+1][col-1]) {
                    ('M', 'S', 'M', 'S') |
                    ('S', 'M', 'M', 'S') |
                    ('M', 'S', 'S', 'M') |
                    ('S', 'M', 'S', 'M') => total += 1,
                    _ => total = total
                }
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/04-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/04-sample.out").unwrap());

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
    fn test_level1_manual() {
        // let input = vec!["AMXMA"];
        // assert_eq!(solve_level1(&input), 0);

        // let input = vec!["XMASXMAS"];
        // assert_eq!(solve_level1(&input), 2);
        // let input = vec!["XMASSAMX"];
        // assert_eq!(solve_level1(&input), 2);

        let input = vec![
            "X.....",
            ".M....",
            "..A...",
            "...S.."
        ];
        assert_eq!(solve_level1(&input), 1);
        let input = vec![
            "S.....",
            ".A....",
            "..M...",
            "...X.."
        ];
        assert_eq!(solve_level1(&input), 1);

        let input = vec![
            "X...X.",
            ".M.M..",
            "..A...",
            ".S.S.."
        ];
        assert_eq!(solve_level1(&input), 2);

        let input = vec![
            "......",
            ".....X",
            "....M.",
            "...A..",
            "..S..."
        ];
        assert_eq!(solve_level1(&input), 1);

        let input = vec![
            "......",
            ".....S",
            "....A.",
            "...M..",
            "..X..."
        ];
        assert_eq!(solve_level1(&input), 1);
    }
}
