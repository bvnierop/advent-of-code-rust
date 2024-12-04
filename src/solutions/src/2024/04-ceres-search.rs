#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use regex::Regex;

fn count_xmas(str: &String) -> usize {
    let re = Regex::new(r"(XMAS)").unwrap();
    let reversed = str.chars().rev().collect::<String>();
    re.captures_iter(&str).count() +
        re.captures_iter(&reversed).count()
}

fn form_diagonal<I, J>(lookup: &Vec<Vec<char>>, xs: I, ys: J) -> String
    where I: Iterator<Item = usize>, J: Iterator<Item = usize> {
    xs.zip(ys).map(|(x, y)| lookup[y][x]).collect()
}

#[advent_of_code(2024, 4, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let lookup: Vec<Vec<_>> = input.iter().map(|&s| s.chars().collect()).collect();

    let cols = lookup[0].len();
    let rows = lookup.len();

    let mut total = 0;

    // horizontal
    for line in input {
        total += count_xmas(&line.to_string());
    }

    // vertical
    for x in 0..cols {
        let str = lookup.iter().map(|line| line[x]).collect::<String>();
        total += count_xmas(&str);
    }

    // top-left -> bottom-right, starting at the top row
    for x in 0..cols {
        total += count_xmas(&form_diagonal(&lookup, x..cols, 0..rows));
    }

    // top-left -> bottom-right, starting at the left column. Skip (0,0).
    for y in 1..rows {
        total += count_xmas(&form_diagonal(&lookup, 0..cols, y..rows));
    }

    // top-right -> bottom-left, starting at the top row
    for x in 0..cols {
        total += count_xmas(&form_diagonal(&lookup, (0..x+1).rev(), 0..rows));
    }

    // top-right -> bottom-left, starting at the right column. Skip (cols, 0);
    for y in 1..rows {
        total += count_xmas(&form_diagonal(&lookup, (0..cols).rev(), y..rows));
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
        let input = vec!["AMXMA"];
        assert_eq!(solve_level1(&input), 0);

        let input = vec!["XMASXMAS"];
        assert_eq!(solve_level1(&input), 2);
        let input = vec!["XMASSAMX"];
        assert_eq!(solve_level1(&input), 2);

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
            ".....X",
            "....MS",
            "...AA.",
            "..SM..",
            "..X..."
        ];
        assert_eq!(solve_level1(&input), 2);
    }
}
