#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[advent_of_code(2024, 8, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let width = input[0].len() as i32;
    let height = input.len() as i32;
    let map: Vec<Vec<_>> = input.iter().map(|&line| line.chars().collect()).collect();

    let mut frequency_map:HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut frequencies: HashSet<char> = HashSet::new();
    for y in 0..height as usize {
        for x in 0..width as usize {
            if map[y][x] != '.' {
                frequency_map.entry(map[y][x]).or_default().push((x as i32, y as i32));
                frequencies.insert(map[y][x]);
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for freq in &frequencies {
        for ((x1, y1), (x2, y2)) in frequency_map[freq].iter().tuple_combinations() {
            let dx = (x1 - x2).abs();
            let dy = (y1 - y2).abs();

            let anx1 = if x1 < x2 { x1 - dx } else { x1 + dx };
            let any1 = if y1 < y2 { y1 - dy } else { y1 + dy };

            let anx2 = if x2 < x1 { x2 - dx } else { x2 + dx };
            let any2 = if y2 < y1 { y2 - dy } else { y2 + dy };

            antinodes.insert((anx1, any1));
            antinodes.insert((anx2, any2));
        }
    }

    antinodes.iter()
        .copied()
        .filter(|(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
        .count()
}

#[advent_of_code(2024, 8, 2)]
pub fn solve_level2(input: &[&str]) -> usize {
    let width = input[0].len() as i32;
    let height = input.len() as i32;
    let map: Vec<Vec<_>> = input.iter().map(|&line| line.chars().collect()).collect();

    let mut frequency_map:HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut frequencies: HashSet<char> = HashSet::new();
    for y in 0..height as usize {
        for x in 0..width as usize {
            if map[y][x] != '.' {
                frequency_map.entry(map[y][x]).or_default().push((x as i32, y as i32));
                frequencies.insert(map[y][x]);
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    for freq in &frequencies {
        for ((x1, y1), (x2, y2)) in frequency_map[freq].iter().tuple_combinations() {
            let dx = (x1 - x2).abs();
            let dy = (y1 - y2).abs();

            // side one
            let dx1 = if x1 < x2 { -dx } else { dx };
            let dy1 = if y1 < y2 { -dy } else { dy };

            let mut xx: i32 = *x1; let mut yy: i32 = *y1;
            xx += dx1; yy += dy1;
            while xx >= 0 && xx < width && yy >= 0 && yy < height {
                antinodes.insert((xx, yy));
                xx += dx1; yy += dy1;
            }

            let dx2 = if x1 < x2 { dx } else { -dx };
            let dy2 = if y1 < y2 { dy } else { -dy };
            let mut xx: i32 = *x1; let mut yy: i32 = *y1;
            xx += dx2; yy += dy2;
            while xx >= 0 && xx < width && yy >= 0 && yy < height {
                antinodes.insert((xx, yy));
                xx += dx2; yy += dy2;
            }

            antinodes.insert((*x1, *y1));
            antinodes.insert((*x2, *y2));
        }
    }

    antinodes.iter()
             .copied()
             .filter(|(x, y)| *x >= 0 && *x < width && *y >= 0 && *y < height)
             .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/08-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/08-sample.out").unwrap());

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
