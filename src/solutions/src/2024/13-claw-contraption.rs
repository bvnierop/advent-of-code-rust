#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64
}


#[derive(Debug)]
struct Machine {
    a: Point,
    b: Point,
    prize: Point
}

fn parse(lines: &[&str], add: i64) -> Machine {
    let a = scan_fmt!(lines[0], "Button A: X+{}, Y+{}", i64, i64).unwrap();
    let b = scan_fmt!(lines[1], "Button B: X+{}, Y+{}", i64, i64).unwrap();
    let prize = scan_fmt!(lines[2], "Prize: X={}, Y={}", i64, i64).unwrap();
    Machine {
        a: Point { x: a.0, y: a.1 },
        b: Point { x: b.0, y: b.1 },
        prize: Point { x: prize.0 + add, y: prize.1 + add }
    }
}

fn solve(machine: &Machine) -> i64 {
    let mut best = i64::max_value();
    let mut ans = false;
    for a in 0..=100 {
        for b in 0..=100 {
            let x = machine.a.x * a + machine.b.x * b;
            let y = machine.a.y * a + machine.b.y * b;
            // println!("a: {}, b: {}, x: {}, y: {}, machine: {:?}", a, b, x, y, machine);
            if x == machine.prize.x && y == machine.prize.y {
                best = best.min(a*3 + b);
                ans = true;
            }
        }
    }

    if ans { best } else { 0 }
}

fn solve_eq(machine: &Machine) -> i64 {
    let b = (machine.prize.x * machine.a.y - machine.prize.y * machine.a.x)
        / (machine.a.y * machine.b.x - machine.b.y * machine.a.x);
    let a = (machine.prize.x * machine.b.y - machine.prize.y * machine.b.x)
        / (machine.b.y * machine.a.x - machine.a.y * machine.b.x);

    if machine.a.x * a + machine.b.x * b == machine.prize.x && machine.a.y * a + machine.b.y * b == machine.prize.y {
        3*a + b
    } else {
        0
    }
}

#[advent_of_code(2024, 13, 1)]
pub fn solve_level1(input: &[&str]) -> i64 {
    let machines: Vec<_> = input.chunks(4).map(|chunk| parse(chunk, 0)).collect();
    machines.iter()
        .map(solve)
        .sum()
}

#[advent_of_code(2024, 13, 2)]
pub fn solve_level2(input: &[&str]) -> i64 {
    let machines: Vec<_> = input.chunks(4).map(|chunk| parse(chunk, 10000000000000)).collect();

    machines.iter()
        .map(solve_eq)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/13-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/13-sample.out").unwrap());

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
