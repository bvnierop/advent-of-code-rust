#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use ndarray::*;
use std::collections::VecDeque;

fn step(grid: &mut Array2<u32>) -> usize {
    // increment all
    grid.mapv_inplace(|o| o + 1);

    // flash: starting at the top-left, flash each octopus.
    //          increment surrounding
    //          if > 10: flash recursively
    //            (maintain 'seen' or 'flashed' list)
    let mut q: VecDeque<_> = grid.indexed_iter()
        .filter_map(|((y, x), &n)| if 9 < n { Some((y, x)) } else { None }).collect();
    let mut flashed = grid.mapv(|o| 9 < o);

    let dx = [0, 1, 0, -1, 1, 1, -1, -1];
    let dy = [-1, 0, 1, 0, 1, -1, 1, -1];

    while let Some((y, x)) = q.pop_front() {
        for d in 0..8 {
            let nx = (x as i32 + dx[d]) as usize;
            let ny = (y as i32 + dy[d]) as usize;
            if let Some(f) = flashed.get((ny, nx)) {
                grid[(ny, nx)] += 1;
                if !f && 9 < grid[(ny, nx)] {
                    flashed[(ny, nx)] = true;
                    q.push_back((ny, nx));
                }
            }
        }
    }

    // update flashed octopi
    grid.mapv_inplace(|o| if 9 < o { 0 } else { o });
    flashed.iter().filter(|&&f| f).count()
}

#[advent_of_code(2021, 11, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let v: Vec<_> = input.iter().flat_map(|&line| line.chars().map(|c| c.to_digit(10).unwrap())).collect();

    let width = input[0].len();
    let height = input.len();
    let mut grid = Array2::from_shape_vec((height, width), v).unwrap();

    (0..100).map(|_s| step(&mut grid)).sum()
}

#[advent_of_code(2021, 11, 2)]
pub fn solve_level2(input: &[&str]) -> usize {
    let v: Vec<_> = input.iter().flat_map(|&line| line.chars().map(|c| c.to_digit(10).unwrap())).collect();

    let width = input[0].len();
    let height = input.len();
    let mut grid = Array2::from_shape_vec((height, width), v).unwrap();

    let count = width * height;
    (1..1000000).find(|_step| step(&mut grid) == count).expect("Not all octopi flashed at the same step!")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/11-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/11-sample.out").unwrap());

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
