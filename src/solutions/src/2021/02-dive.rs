use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;

#[advent_of_code(2021, 2, 1)]
pub fn solve_level1(input: &[&str]) -> String {
    let mut depth = 0;
    let mut pos = 0;
    let instructions = input.iter()
                            .map(|s| scan_fmt!(s, "{} {}", String, u32).unwrap());

    for (instr, amount) in instructions {
        match instr.as_str() {
            "forward" => pos += amount,
            "down" => depth += amount,
            "up" => depth -= amount,
            _ => todo!("This can never happen")
        }
    }

    (pos * depth).to_string()
}

#[advent_of_code(2021, 2, 2)]
pub fn solve_level2(input: &[&str]) -> String {
    let mut depth = 0;
    let mut pos = 0;
    let mut aim = 0;
    let instructions = input.iter()
                            .map(|s| scan_fmt!(s, "{} {}", String, u32).unwrap());

    for (instr, amount) in instructions {
        match instr.as_str() {
            "forward" => {pos += amount; depth += aim * amount;},
            "down" => aim += amount,
            "up" => aim -= amount,
            _ => todo!("This can never happen")
        }
    }

    (pos * depth).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/02-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/02-sample.out").unwrap());

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
