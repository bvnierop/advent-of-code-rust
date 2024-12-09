#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::collections::HashMap;

#[advent_of_code(2021, 10, 1)]
pub fn solve_level1(input: &[&str]) -> u64 {
    let mut sum = 0;
    let lookup = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    for line in input {
        let mut stack: Vec<char> = vec![];
        for chr in line.chars() {
            match chr {
                '(' | '[' | '{' | '<' => { stack.push(chr); },

                ')' | ']' | '}' | '>' => {
                    if let Some(c) = stack.pop() {
                        if c == '(' && chr != ')' { sum += lookup[&chr]; break; }
                        if c == '[' && chr != ']' { sum += lookup[&chr]; break; }
                        if c == '{' && chr != '}' { sum += lookup[&chr]; break; }
                        if c == '<' && chr != '>' { sum += lookup[&chr]; break; }
                    }
                },
                _ => panic!("Expected a brace")
            }
        }
    }

    sum
}

#[advent_of_code(2021, 10, 2)]
pub fn solve_level2(input: &[&str]) -> u64 {
    let lookup = HashMap::from([('(', 1), ('[', 2), ('{', 3), ('<', 4)]);
    let mut scores: Vec<u64> = vec![];
    for line in input {
        let mut stack: Vec<char> = vec![];
        let mut correct = true;
        for chr in line.chars() {
            match chr {
                '(' | '[' | '{' | '<' => { stack.push(chr); },
                ')' | ']' | '}' | '>' => {
                    if let Some(c) = stack.pop() {
                        if c == '(' && chr != ')' { correct = false; break; }
                        if c == '[' && chr != ']' { correct = false; break; }
                        if c == '{' && chr != '}' { correct = false; break; }
                        if c == '<' && chr != '>' { correct = false; break; }
                    }
                },
                _ => panic!("Expected a brace")
            }
        }

        if !correct { continue };
        let mut sum = 0;
        while let Some(c) = stack.pop() {
            sum *= 5;
            sum += lookup[&c];
        }
        scores.push(sum);
    }

    scores.sort();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/10-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/10-sample.out").unwrap());

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
