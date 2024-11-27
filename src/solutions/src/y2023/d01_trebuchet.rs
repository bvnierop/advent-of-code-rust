use aoc_macros::advent_of_code;
use inventory;

#[advent_of_code(2023, 1, 1)]
pub fn solve_level1(input: &[&str]) -> String {
    todo!("Implement solution for level 1")
}

#[advent_of_code(2023, 1, 2)]
pub fn solve_level2(input: &[&str]) -> String {
    todo!("Implement solution for level 2")
}

// #[advent_of_code(2023, 1, 2)]
// pub fn solve_level2(input: &[&str]) -> String {
//     todo!("Implement solution for level 2")
// }

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
