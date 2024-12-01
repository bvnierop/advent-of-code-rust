#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use im::Vector;

#[advent_of_code(2021, 3, 1)]
pub fn solve_level1(input: &[&str]) -> u32 {
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

    let g = u32::from_str_radix(
        &gamma.into_iter().collect::<String>(),
        2).unwrap();
    let e = u32::from_str_radix(
        &epsilon.into_iter().collect::<String>(),
        2).unwrap();
    g*e
}

type Chars = Vec<char>;

#[advent_of_code(2021, 3, 2)]
pub fn solve_level2(input: &[&str]) -> u32 {
    let diagnostics: Vec<Chars> = input.iter().map(|s| s.chars().collect()).collect();
    let reflist = diagnostics.iter().collect::<Vec<&Chars>>();

    let most_common = |zeroes, ones| if zeroes <= ones { '1' } else { '0' };
    let least_common = |zeroes, ones|
        if ones == 0 { '0' }
        else if zeroes == 0 { '1' }
        else if ones < zeroes { '1' }
        else { '0' };


    let oxy = search(&reflist.clone(), most_common);
    let co2 = search(&reflist.clone(), least_common);

    let o = u32::from_str_radix(&oxy, 2).unwrap();
    let c = u32::from_str_radix(&co2, 2).unwrap();
    o*c
}

fn search<'a>(diagnostics: &Vec<&'a Chars>, predicate: impl Fn(usize, usize) -> char) -> String {
    fn search_impl<'a>(diagnostics: &Vec<&'a Chars>, bit_index: usize, predicate: impl Fn(usize, usize) -> char) -> Vec<&'a Chars> {
        if diagnostics.len() == 1 {
            diagnostics.clone()
        } else {
            let (zeroes, ones) =
                diagnostics.iter()
                           .fold((0, 0), |(zeroes, ones), line|
                                 if line[bit_index] == '0' {
                                     (zeroes + 1, ones) }
                                 else {
                                     (zeroes, ones + 1)
                                 });

            let bit = predicate(zeroes, ones);

            search_impl(
                &diagnostics.iter()
                            .filter(|line| line[bit_index] == bit)
                            .cloned()
                            .collect(), bit_index + 1, predicate)
        }
    }
    search_impl(diagnostics, 0, predicate)[0].into_iter().collect::<String>()
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
            assert_eq!(format!("{}", solve_level1(&input)), expected);
        }

        #[test]
        fn test_level2() {
            let input: Vec<_> = (*SAMPLE).lines().collect();
            let expected = (*SAMPLE_OUT).lines().skip(3).next().unwrap();
            assert_eq!(format!("{}", solve_level2(&input)), expected);
        }
    }
