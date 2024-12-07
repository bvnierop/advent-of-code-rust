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

fn is_solvable((expected, numbers): &Equation) -> bool {

    fn recurse(numbers: &Vec<u64>, index: usize, result: u64, expected: u64) -> bool {
        if index == numbers.len() && result == expected { return true; }
        if index == numbers.len() && result != expected { return false; }

        let next = numbers[index];
        if recurse(numbers, index + 1, result + next, expected) == true { return true; }
        if recurse(numbers, index + 1, result * next, expected) == true { return true; }

        false
    }

    recurse(numbers, 1, numbers[0], *expected)
}


fn is_solvable2((expected, numbers): &Equation) -> bool {

    fn recurse(numbers: &Vec<u64>, index: usize, result: u64, expected: u64) -> bool {
        if index == numbers.len() && result == expected { return true; }
        if index == numbers.len() && result != expected { return false; }

        let next = numbers[index];
        if recurse(numbers, index + 1, result + next, expected) == true { return true; }
        if recurse(numbers, index + 1, result * next, expected) == true { return true; }
        let c = format!("{}{}", result, next).parse::<u64>().unwrap();
        if recurse(numbers, index + 1, c, expected) == true { return true; }


        false
    }

    recurse(numbers, 1, numbers[0], *expected)
}

#[advent_of_code(2024, 7, 1)]
pub fn solve_level1(input: &[&str]) -> u64 {
    let equations = input.iter().copied().map(parse);
    equations.filter(is_solvable).map(|(e, _n)| e).sum()
}

#[advent_of_code(2024, 7, 2)]
pub fn solve_level2(input: &[&str]) -> u64 {
    let equations = input.iter().copied().map(parse);
    equations.filter(is_solvable2).map(|(e, _n)| e).sum()
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
