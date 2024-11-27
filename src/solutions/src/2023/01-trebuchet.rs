use aoc_macros::advent_of_code;
use inventory;

#[advent_of_code(2023, 1, 1)]
pub fn solve_level1(input: &[&str]) -> String {
    "expected".to_string()
}

#[advent_of_code(2023, 1, 2)]
pub fn solve_level2(_input: &[&str]) -> String {
    "expected".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
"#;

    #[test]
    fn test_level1() {
        let input: Vec<_> = EXAMPLE.lines().collect();
        assert_eq!(solve_level1(&input), "expected");
    }

    #[test]
    fn test_level2() {
        let input: Vec<_> = EXAMPLE.lines().collect();
        assert_eq!(solve_level2(&input), "expected");
    }
}
