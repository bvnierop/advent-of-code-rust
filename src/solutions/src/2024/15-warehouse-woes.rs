#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;

pub fn mv(x: usize, y: usize, dx: i32, dy: i32, map: &mut Vec<Vec<char>>) -> bool {
    // try to move the next box in the direction. If that succeeded, move the box we're looking at
    let nx = (x as i32 + dx) as usize;
    let ny = (y as i32 + dy) as usize;
    if map[ny][nx] == '#' { return false; }
    if map[ny][nx] != 'O' { // not wall (see above), not box
        map[ny][nx] = 'O';
        map[y][x] = '.';
        return true;
    } else {
        // try to move the next box in the direction. If that succeeded, move the box we're looking at
        if mv((x as i32 + dx) as usize, (y as i32 + dy) as usize, dx, dy, map) {
            map[ny][nx] = 'O';
            map[y][x] = '.';
            return true;
        }
    }

    false
}

#[advent_of_code(2024, 15, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let mut map: Vec<Vec<_>> = input.iter().take_while(|&l| !l.is_empty())
                                           .map(|&line| line.chars().collect()).collect();
    let instructions: Vec<_> = input.iter().skip_while(|&l| !l.is_empty()).skip(1).join("").chars().collect();

    let width = map[0].len();
    let height = map.len();

    let mut rx = 0; let mut ry = 0;
    for x in 0..width {
        for y in 0..height {
            if map[y][x] == '@' {
                rx = x;
                ry = y;
            }
        }
    }

    for instruction in instructions {
        // println!("{}", instruction);
        if instruction == '^' {
            if map[ry-1][rx] == '#' { continue; }
            if map[ry-1][rx] == 'O' { mv(rx, ry - 1, 0, -1, &mut map); }
            if map[ry-1][rx] != 'O' { ry -= 1; }
        }
        if instruction == '>' {
            if map[ry][rx+1] == '#' { continue; }
            if map[ry][rx+1] == 'O' { mv(rx + 1, ry, 1, 0, &mut map); }
            if map[ry][rx+1] != 'O' { rx += 1; }
        }
        if instruction == 'v' {
            if map[ry+1][rx] == '#' { continue; }
            if map[ry+1][rx] == 'O' { mv(rx, ry + 1, 0, 1, &mut map); }
            if map[ry+1][rx] != 'O' { ry += 1; }
        }
        if instruction == '<' {
            if map[ry][rx-1] == '#' { continue; }
            if map[ry][rx-1] == 'O' { mv(rx - 1, ry, -1, 0, &mut map); }
            if map[ry][rx-1] != 'O' { rx -= 1; }
        }

        // for y in 0..height {
        //     for x in 0..width {
        //         if y == ry && x == rx {
        //             print!("@");
        //             assert!(map[y][x] != '#');
        //             assert!(map[y][x] != 'O');
        //         } else {
        //             print!("{}", map[y][x]);
        //         }
        //     }
        //     println!("");
        // }
    }

    let mut score = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == 'O' {
                score += (y*100) + x;
            }
        }
    }

    score
}

pub fn mv2(x: usize, y: usize, dx: i32, dy: i32, map: &mut Vec<Vec<char>>) -> bool {
    let nx = (x as i32 + dx*1) as usize;
    let nx2 = (x as i32 + dx*2) as usize;
    let ny = (y as i32 + dy) as usize;
    if map[ny][nx2] == '#' { return false; }
    if map[ny][nx2] != '[' && map[ny][nx2] != ']' { // not wall (see above), not box
        map[ny][nx.min(nx2)] = '[';
        map[ny][nx2.max(nx)] = ']';
        map[y][x] = '.';
        return true;
    } else {
        // try to move the next box in the direction. If that succeeded, move the box we're looking at
        if mv2(nx2, ny, dx, dy, map) {
            map[ny][nx.min(nx2)] = '[';
            map[ny][nx2.max(nx)] = ']';
            map[y][x] = '.';
            return true;
        }
    }

    false
}

pub fn check(x: usize, y: usize, dx: i32, dy: i32, map: &mut Vec<Vec<char>>) -> bool {
    // we only call this when we're moving a box UP.
    assert_eq!(dx, 0);

    assert_eq!(map[y][x], '[');
    assert_eq!(map[y][x + 1], ']');

    let ny = (y as i32 + dy) as usize;

    // find the left and right of the box
    let mut left = x; let mut right = x + 1;
    if map[y][x] == ']' {
        left = x - 1; right = x;
    }

    // we can move a box if
    //    1) There is no obstacle in front at all
    if map[ny][left] == '.' && map[ny][right] == '.' { return true; }

    //    2) There is a box in front and it can be moved
    if map[ny][left] == '[' {
        assert_eq!(map[ny][right], ']');
        return check(left, ny, dx, dy, map);
    }

    //   3) There's a box on the left overlap that can be moved
    if map[ny][left] == ']' && map[ny][right] == '.' {
        return check(left - 1, ny, dx, dy, map);
    }

    //   4) There's a box on the right overlap that can be moved
    if map[ny][left] == '.' && map[ny][right] == '[' {
        return check(right, ny, dx, dy, map);
    }

    //   4) There are two overlapping boxes and both can be moved
    if map[ny][left] == ']' && map[ny][right] == '[' {
        return check(left - 1, ny, dx, dy, map) && check(right, ny, dx, dy, map);
    }

    // In any other case, we cannot move
    false
}

pub fn mv3(x: usize, y: usize, dx: i32, dy: i32, map: &mut Vec<Vec<char>>) -> bool {
    // we only call this when we're moving a box UP.
    assert_eq!(dx, 0);
    let ny = (y as i32 + dy) as usize;

    // find the left and right of the box
    let mut left = x; let mut right = x + 1;
    if map[y][x] == ']' {
        left = x - 1; right = x;
    }

    if !check(left, y, dx, dy, map) { return false; }

    if map[ny][left] == '[' { // box directly in front
        assert!(mv3(left, ny, dx, dy, map));
    }

    if map[ny][left] == ']' { // box in front on the left
        assert!(mv3(left - 1, ny, dx, dy, map));
    }

    if map[ny][right] == '[' { // box in front on the right
        assert!(mv3(right, ny, dx, dy, map));
    }

    assert!(map[ny][left] == '.');
    assert!(map[ny][right] == '.');

    map[ny][left] = '[';
    map[ny][right] = ']';
    map[y][left] = '.';
    map[y][right] = '.';
    true
}

#[advent_of_code(2024, 15, 2)]
pub fn solve_level2(input: &[&str]) -> usize {
    let mut map: Vec<Vec<_>> = input.iter().take_while(|&l| !l.is_empty())
                                           .map(|&line| {
                                               line.chars().map(|c| {
                                                   match c {
                                                       '#' => "##",
                                                       'O' => "[]",
                                                       '.' => "..",
                                                       '@' => "@.",
                                                       _ => panic!("Expected one of #,O,.,@"),
                                                   }.chars()
                                               }).flatten().collect()
                                           }).collect();
    let instructions: Vec<_> = input.iter().skip_while(|&l| !l.is_empty()).skip(1).join("").chars().collect();

    let width = map[0].len();
    let height = map.len();

    let mut rx = 0; let mut ry = 0;
    for x in 0..width {
        for y in 0..height {
            if map[y][x] == '@' {
                rx = x;
                ry = y;
                map[y][x] = '.';
            }
        }
    }


    for instruction in instructions {
        // println!("{}", instruction);
        if instruction == '^' {
            if map[ry-1][rx] == '#' { continue; }
            if map[ry-1][rx] == '[' || map[ry-1][rx] == ']' { mv3(rx, ry - 1, 0, -1, &mut map); }
            if map[ry-1][rx] != '#' && map[ry-1][rx] != '[' && map[ry-1][rx] != ']' {
                ry -= 1;
            }
        }
        if instruction == '>' {
            if map[ry][rx+1] == '#' { continue; }
            if map[ry][rx+1] == '[' { mv2(rx + 1, ry, 1, 0, &mut map); }
            if map[ry][rx+1] != '[' { rx += 1; }
        }
        if instruction == 'v' {
            if map[ry+1][rx] == '#' { continue; }
            if map[ry+1][rx] == '[' || map[ry+1][rx] == ']' { mv3(rx, ry + 1, 0, 1, &mut map); }
            if map[ry+1][rx] != '#' && map[ry+1][rx] != '[' && map[ry+1][rx] != ']' {
                ry += 1;
            }
        }
        if instruction == '<' {
            if map[ry][rx-1] == '#' { continue; }
            if map[ry][rx-1] == ']' { mv2(rx - 1, ry, -1, 0, &mut map); }
            if map[ry][rx-1] != ']' { rx -= 1; }
        }

        // for y in 0..height {
        //     for x in 0..width {
        //         if y == ry && x == rx {
        //             print!("@");
        //             assert!(map[y][x] != '#');
        //             assert!(map[y][x] != '[');
        //             assert!(map[y][x] != ']');
        //         } else if map[y][x] == '@' {
        //             print!(".");
        //         } else {
        //             print!("{}", map[y][x]);
        //         }
        //     }
        //     println!("");
        // }
    }
        // for y in 0..height {
        //     for x in 0..width {
        //         if y == ry && x == rx {
        //             print!("@");
        //             assert!(map[y][x] != '#');
        //             assert!(map[y][x] != 'O');
        //         } else {
        //             print!("{}", map[y][x]);
        //         }
        //     }
        //     println!("");
        // }

    let mut score = 0;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == '[' {
                score += (y*100) + x;
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/15-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/15-sample.out").unwrap());

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
