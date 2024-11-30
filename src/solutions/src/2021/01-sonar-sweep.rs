use aoc_macros::advent_of_code;
use inventory;

#[advent_of_code(2021, 1, 1)]
pub fn solve_level1(input: &[&str]) -> String {
    input.iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|window| window[1] > window[0])
        .count()
        .to_string()
}

#[advent_of_code(2021, 1, 2)]
pub fn solve_level2(input: &[&str]) -> String {
    input.iter()
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
        .windows(4)
        .filter(|window| window[3] > window[0])
        .count()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/01-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/01-sample.out").unwrap());

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
