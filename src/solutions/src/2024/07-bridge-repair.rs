#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;

type Equation = (u64, Vec<u64>);

fn parse(line: &str) -> Equation {
    let mut splitted = line.split(":");
    let expected = splitted.next().unwrap();
    let numbers = splitted.next().unwrap();
    (expected.parse::<u64>().unwrap(),
    numbers.split_whitespace().map(|s| s.parse::<u64>().unwrap()).collect())
}

fn is_solvable((expected, numbers): &Equation, operators: &Vec<impl Fn(u64, u64) -> u64>) -> bool {

    fn recurse(numbers: &Vec<u64>, index: usize, result: u64, expected: u64, operators: &Vec<impl Fn(u64, u64) -> u64>) -> bool {
        if index == numbers.len() && result == expected { return true; }
        if index == numbers.len() && result != expected { return false; }
        if result >= expected { return false; }

        let next = numbers[index];
        operators
            .iter()
            .any(|o| recurse(numbers, index + 1, o(result, next), expected, operators))
    }

    recurse(numbers, 1, numbers[0], *expected, operators)
}

#[advent_of_code(2024, 7, 1)]
pub fn solve_level1(input: &[&str]) -> u64 {
    let equations = input.iter().copied().map(parse);
    let operators: Vec<_> = vec![|a, b| a + b, |a, b| a * b];
    equations.filter(|e| is_solvable(e, &operators)).map(|(e, _n)| e).sum()
}

fn concat_numbers(a: u64, b: u64) -> u64 {
    a * 10u64.pow(b.ilog10() + 1) + b
}

#[advent_of_code(2024, 7, 2)]
pub fn solve_level2(input: &[&str]) -> u64 {
    let equations = input.iter().copied().map(parse);
    let operators: Vec<_> = vec![|a, b| a + b, |a, b| a * b, concat_numbers];
    equations.filter(|e| is_solvable(e, &operators)).map(|(e, _n)| e).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/07-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/07-sample.out").unwrap());

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
