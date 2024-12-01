#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use std::collections::HashMap;
use im::Vector;

#[advent_of_code(2021, 3, 1)]
pub fn solve_level1(input: &[&str]) -> u32 {
    let mut gamma_bits = String::new();
    let mut epsilon_bits = String::new();

    let diagnostics: Vec<_> = input.iter().map(|s| s.as_bytes()).collect();

    for bit_index in 0..input[0].len() {
        let ones = diagnostics.iter().filter(|line| line[bit_index] == b'1').count();
        let zeroes = input.len() - ones;
        gamma_bits.push(if ones > zeroes { '1' } else { '0' });
        epsilon_bits.push(if ones > zeroes { '0' } else { '1' });
    }

    let gamma = u32::from_str_radix(&gamma_bits, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon_bits, 2).unwrap();
    gamma * epsilon
}

#[advent_of_code(2021, 3, 2)]
pub fn solve_level2(input: &[&str]) -> u32 {
    let diagnostics: Vec<_> = input.iter().map(|s| s.as_bytes()).collect();

    let most_common = |zeroes, ones| if zeroes <= ones { b'1' } else { b'0' };
    let least_common = |zeroes, ones|
        if ones == 0 { b'0' }
        else if zeroes == 0 { b'1' }
        else if ones < zeroes { b'1' }
        else { b'0' };


    let oxy = find_rating(&diagnostics.clone(), most_common);
    let co2 = find_rating(&diagnostics.clone(), least_common);

    oxy * co2
}

fn find_rating<'a>(diagnostics: &[&[u8]], predicate: impl Fn(usize, usize) -> u8) -> u32 {
    let mut candidates = diagnostics.to_vec();
    for bit_index in 0..candidates[0].len() {
        if candidates.len() == 1 { break; }

        let ones = candidates.iter().filter(|line| line[bit_index] == b'1').count();
        let zeroes = candidates.len() - ones;

        let bit = predicate(zeroes, ones);
        candidates.retain(|line| line[bit_index] == bit);
    }
    use core::str;
    u32::from_str_radix(str::from_utf8(candidates[0]).unwrap(), 2).unwrap()
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
