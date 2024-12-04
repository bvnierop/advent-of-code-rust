#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;

const ONE: usize = 2;
const FOUR: usize = 4;
const SEVEN: usize = 3;
const EIGHT: usize = 7;

fn count_in_line(line: &str) -> usize {
    let output = line.split("|").skip(1).next().unwrap();
    let encoded_numbers: Vec<_> = output.split_whitespace().map(|s| s.len()).collect();
    encoded_numbers.iter().filter(|l| [ONE, FOUR, SEVEN, EIGHT].contains(l)).count()
}

#[advent_of_code(2021, 8, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    input.iter().copied().map(count_in_line).sum()
}

fn parse(line: &str) -> (Vec<HashSet<char>>, Vec<HashSet<char>>) {
    let mut splitted = line.split("|");
    let definition = splitted.next().expect("definition should be the first token");
    let encoded_numbers = splitted.next().expect("output numbers should be the second token");

    let definitions =
        definition.split_whitespace()
                  .map(|s| {
                      s.chars()
                       .sorted()
                       .collect::<HashSet<_>>()
                  }).collect();

    let output_numbers =
        encoded_numbers
        .split_whitespace()
        .map(|s| {
            s.chars()
             .sorted()
             .collect::<HashSet<_>>()
        }).collect();

    (definitions, output_numbers)
}

fn identify(definitions: &Vec<HashSet<char>>, how: impl Fn(&HashSet<char>) -> bool) -> &HashSet<char> {
    definitions.iter().find(|n| how(n)).expect("Expected to find exactly one match")
}

fn completely_covers(thing: &HashSet<char>, covers: &HashSet<char>) -> bool {
    thing.intersection(covers).count() == covers.len()
}

#[advent_of_code(2021, 8, 2)]
pub fn solve_level2(input: &[&str]) -> usize {

    // let map: HashMap<Vec<char>, i32> = HashMap::new();

    let mut sum = 0;
    for line in input {
        let (definitions, digits) = parse(line);

        let one = identify(&definitions, |n| n.len() == 2);
        let four = identify(&definitions, |n| n.len() == 4);
        let seven = identify(&definitions, |n| n.len() == 3);
        let eight = identify(&definitions, |n| n.len() == 7);
        let three = identify(&definitions, |n| n.len() == 5 && completely_covers(n, seven));
        let zero = identify(&definitions, |n| n.len() == 6 && completely_covers(n, seven) && !completely_covers(n, three));
        let six = identify(&definitions, |n| n.len() == 6 && !completely_covers(n, seven));
        let nine = identify(&definitions, |n| n.len() == 6 && completely_covers(n, three));
        let two = identify(&definitions, |n| n.len() == 5 && !completely_covers(nine, n) && !completely_covers(n, seven));
        let five = identify(&definitions, |n| n.len() == 5 && completely_covers(nine, n) && !completely_covers(n, seven));

        let mapping = [zero, one, two, three, four, five, six, seven, eight, nine];

        sum +=
            digits.iter().fold(0, |display, digit| {
                let (next_digit, _) = mapping.iter().enumerate().find(|(_n, d)| (**d).eq(digit)).expect(&format!("Expected to find exactly one match for\n{:?}\nin\n{:?}\n", digit, mapping));
                display * 10 + next_digit
            });
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/08-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/08-sample.out").unwrap());

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
