#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use std::collections::HashMap;

#[advent_of_code(2021, 3, 1)]
pub fn solve_level1(input: &[&str]) -> String {
    let mut gamma: Vec<char> = Vec::new();
    let mut epsilon: Vec<char> = Vec::new();

    let diagnostics: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

    for bit_index in 0..input[0].len() {
        let mut ones = 0;
        let mut zeroes = 0;
        for line in &diagnostics {
            match line[bit_index] {
                '0' => zeroes += 1,
                '1' => ones += 1,
                _ => panic!("Cannot happen!")
            }
        }
        gamma.push(if ones > zeroes { '1' } else { '0' });
        epsilon.push(if ones > zeroes { '0' } else { '1' });

    }


    let gs: String = gamma.into_iter().collect();
    let es: String = epsilon.into_iter().collect();
    let g = u32::from_str_radix(&gs, 2).unwrap();
    let e = u32::from_str_radix(&es, 2).unwrap();
    (g*e).to_string()
}

#[advent_of_code(2021, 3, 2)]
pub fn solve_level2(input: &[&str]) -> String {
    let diagnostics: Vec<Vec<char>> = input.iter().map(|s| s.chars().collect()).collect();

    let mut filtered = diagnostics.clone();
    for bit_index in 0..input[0].len() {
        filtered = filter_diagnostics(&filtered, bit_index);
    }

    let mut filtered2 = diagnostics.clone();
    for bit_index in 0..input[0].len() {
        filtered2 = filter_diagnostics2(&filtered2, bit_index);
    }

    if filtered.len() != 1 {
        panic!("Too many items in filtered: {}", filtered.len());
    }
    if filtered2.len() != 1 {
        panic!("Too many items in filtered2");
    }

    let s1: String = filtered[0].clone().into_iter().collect();
    let s2: String = filtered2[0].clone().into_iter().collect();
    let i1 = u32::from_str_radix(&s1, 2).unwrap();
    let i2 = u32::from_str_radix(&s2, 2).unwrap();

    (i1*i2).to_string()
}

fn filter_diagnostics(diagnostics: &Vec<Vec<char>>, bit_index: usize) -> Vec<Vec<char>> {
    let x = diagnostics.iter();
    let y = x.map(|line| line[bit_index]);
    let mut map: HashMap<char, u32> = HashMap::new();
    for c in y {
        *map.entry(c).or_default() += 1;
    }
    let mut most_common = map.iter().max_by_key(|(_k, v)| *v)
        .map(|(k, _v)| *k)
        .unwrap();

    if map.get(&'0').unwrap_or(&0) == map.get(&'1').unwrap_or(&0) { most_common = '1'; }

    let foo = diagnostics.iter()
        .filter(|line| line[bit_index] == most_common);

    let bar: Vec<Vec<char>> = foo.cloned().collect();

    bar
}

fn filter_diagnostics2(diagnostics: &Vec<Vec<char>>, bit_index: usize) -> Vec<Vec<char>> {
    let x = diagnostics.iter();
    let y = x.map(|line| line[bit_index]);
    let mut map: HashMap<char, u32> = HashMap::new();
    for c in y {
        *map.entry(c).or_default() += 1;
    }
    let mut least_common = map.iter().min_by_key(|(_k, v)| *v)
        .map(|(k, _v)| *k)
        .unwrap();
    if map.get(&'0').unwrap_or(&0) == map.get(&'1').unwrap_or(&0) { least_common = '0'; }

    let foo = diagnostics.iter()
        .filter(|line| line[bit_index] == least_common);

    let bar: Vec<Vec<char>> = foo.cloned().collect();

    bar
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/03-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/03-sample.out").unwrap());

    #[test]
    fn test_level1() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().next().unwrap();
        assert_eq!(solve_level1(&input), expected);
    }

    #[test]
    fn test_level2() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().skip(3).next().unwrap();
        assert_eq!(solve_level2(&input), expected);
    }
}
