#![allow(unused_imports)]

use std::cmp::{min, max};
use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use im::HashMap;

#[derive(Debug)]
struct Pipe {
    x1: i32, y1: i32,
    x2: i32, y2: i32,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Point { x: i32, y: i32 }

impl Pipe {
    fn new(line: &str) -> Self {
        let (x1, y1, x2, y2) = scan_fmt!(line, "{},{} -> {},{}", i32, i32, i32, i32).unwrap();
        Pipe { x1, y1, x2, y2 }
    }

    fn pts(&self) -> Vec<Point> {
        if self.x1 == self.x2 {
            (min(self.y1, self.y2)..=max(self.y1, self.y2))
                          .map(|y| Point { x: self.x1, y })
                        .collect()
        } else if self.y1 == self.y2 {
            (min(self.x1, self.x2)..=max(self.x1, self.x2))
                          .map(|x| Point { x, y: self.y1 })
                        .collect()
        } else {
            let xs: Vec<_> = if self.x1 < self.x2 { (self.x1..=self.x2).rev().collect() } else { (self.x2..=self.x1).collect() };
            let ys: Vec<_> = if self.y1 < self.y2 { (self.y1..=self.y2).rev().collect() } else { (self.y2..=self.y1).collect() };
            xs.into_iter().zip(ys.into_iter())
                          .map(|(x, y)| Point { x, y })
                          .collect()
        }
    }
}



#[advent_of_code(2021, 5, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let straight_pipes = input.into_iter()
                              .map(|line| Pipe::new(*line))
                              .filter(|pipe| pipe.x1 == pipe.x2 || pipe.y1 == pipe.y2);

    let counts = straight_pipes.flat_map(|pipe| pipe.pts().into_iter())
        .fold(HashMap::new(), |counts, point| counts.update_with(point, 1, |old, new| old + new));

    counts.iter()
        .filter(|(_k, v)| **v >= 2)
        .count()
}

#[advent_of_code(2021, 5, 2)]
pub fn solve_level2(input: &[&str]) -> usize {
    let pipes = input.into_iter()
                              .map(|line| Pipe::new(*line));

    let counts = pipes.flat_map(|pipe| pipe.pts().into_iter())
        .fold(HashMap::new(), |counts, point| counts.update_with(point, 1, |old, new| old + new));

    counts.iter()
        .filter(|(_k, v)| **v >= 2)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/05-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/05-sample.out").unwrap());

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
