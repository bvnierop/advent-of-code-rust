#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use std::collections::HashSet;

const ROWS: usize = 5;
const COLS: usize = 5;

#[derive(Debug)]
struct BingoCard {
    numbers: [[u16; ROWS]; COLS],
}

impl BingoCard {
    fn new(lines: &[&str]) -> BingoCard {
        let mut card = BingoCard { numbers: [[0; ROWS]; COLS] };
        for (row, line) in lines.iter().enumerate() {
            let numbers = line.split_whitespace();
            for (col, num) in numbers.enumerate() {
                card.numbers[row][col] = num.parse::<u16>().unwrap();
            }
        }
        card
    }

    fn score(&self, draws: &[u16]) -> i32 {
        let draw_set: HashSet<u16> = draws.iter().copied().collect();
        let mut score: i32 = 0;
        for r in 0..ROWS {
            for c in 0..COLS {
                let num = self.numbers[r][c];
                if !draw_set.contains(&num) {
                    score += i32::from(num)
                }
            }
        }
        let last_draw_as_i32: i32 = i32::from(*draws.last().unwrap());
        score * last_draw_as_i32
    }

    fn winner(&self, draws: &[u16]) -> bool {
        let draw_set: HashSet<u16> = draws.iter().copied().collect();
        for i in 0..ROWS {
            let row = self.numbers[i];
            let col = self.numbers.iter().map(|r| r[i]).collect::<Vec<_>>();

            let winner = row.iter().all(|n| draw_set.contains(n)) || col.iter().all(|n| draw_set.contains(n));
            if winner { return true; }
        }
        false
    }
}

#[advent_of_code(2021, 4, 1)]
pub fn solve_level1(input: &[&str]) -> i32 {
    let draws: Vec<u16> = input[0].split(',').map(|s| s.parse::<u16>().unwrap()).collect();
    let cards: Vec<BingoCard> =
        input.iter()
             .skip(2)
             .filter(|l| !l.is_empty())
             .cloned()
             .collect::<Vec<&str>>()
             .chunks(ROWS)
             .map(BingoCard::new)
             .collect();

    for i in 1..=draws.len() {
        let subset = &draws[0..i];
        for card in &cards {
            if card.winner(subset) {
                return card.score(subset);
            }
        }

    }
    -1
}

#[advent_of_code(2021, 4, 2)]
pub fn solve_level2(input: &[&str]) -> i32 {
    let draws: Vec<u16> = input[0].split(',').map(|s| s.parse::<u16>().unwrap()).collect();
    let mut cards: Vec<BingoCard> =
        input.iter()
             .skip(2)
             .filter(|l| !l.is_empty())
             .cloned()
             .collect::<Vec<&str>>()
             .chunks(ROWS)
             .map(BingoCard::new)
             .collect();

    for i in 1..=draws.len() {
        let subset = &draws[0..i];
        if cards.len() == 1 && cards[0].winner(subset) {
            return cards[0].score(subset);
        }
        cards.retain(|card| !card.winner(subset));
    }
    -1
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/04-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2021/04-sample.out").unwrap());

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
