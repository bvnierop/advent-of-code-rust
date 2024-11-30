use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;

#[advent_of_code(2021, 2, 1)]
pub fn solve_level1(input: &[&str]) -> String {
    input.iter()
         .map(|s| scan_fmt!(s, "{} {}", String, u32).unwrap())
         .fold([0, 0], |[pos, depth], (instruction, amount)| {
             match instruction.as_str() {
                 "forward" => [pos + amount, depth],
                 "down" => [pos, depth + amount],
                 "up" => [pos, depth - amount],
                 _ => todo!("This can never happen")
             }
         })
         .iter().product::<u32>().to_string()
}

#[advent_of_code(2021, 2, 2)]
pub fn solve_level2(input: &[&str]) -> String {
    input.iter()
         .map(|s| scan_fmt!(s, "{} {}", String, u32).unwrap())
         .fold([0, 0, 0], |[pos, depth, aim], (instruction, amount)| {
             match instruction.as_str() {
                 "forward" => [pos + amount, depth + (aim * amount), aim],
                 "down" => [pos, depth, aim + amount],
                 "up" => [pos, depth, aim - amount],
                 _ => todo!("This can never happen")
             }
         })
         .iter().take(2).product::<u32>().to_string()
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
