#![allow(unused_imports)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Puter {
    a: i64,
    b: i64,
    c: i64,
    program: Vec<u8>,
    ip: usize,
}

impl Puter {
    fn has_next(&self) -> bool {
        self.ip < self.program.len() - 1
    }

    fn get_opcode(&self) -> u8 {
        self.program[self.ip]
    }

    fn get_literal_operand(&self) -> i64 {
        self.program[self.ip + 1] as i64
    }

    fn get_combo_operand(&self) -> i64 {
        let val = self.program[self.ip + 1];
        match val {
            0|1|2|3 => val as i64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Unexpected combo operand")
        }
    }
}

fn parse_input(input: &[&str]) -> Puter {
    let ints = scan_fmt!(input[4], "Program: {}", String).unwrap();

    Puter {
        a: scan_fmt!(input[0], "Register A: {}", i64).unwrap(),
        b: scan_fmt!(input[1], "Register B: {}", i64).unwrap(),
        c: scan_fmt!(input[2], "Register C: {}", i64).unwrap(),
        program: ints.split(",").map(|s| s.parse().unwrap()).collect(),
        ip: 0,
    }
}

const ADV: u8 = 0;
const BXL: u8 = 1;
const BST: u8 = 2;
const JNZ: u8 = 3;
const BXC: u8 = 4;
const OUT: u8 = 5;
const BDV: u8 = 6;
const CDV: u8 = 7;

fn run(mut program: Puter) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();

    while program.has_next() {
        // read opcode
        let opcode = program.get_opcode();

        // perform the opcode
        if opcode == ADV {
            let operand = program.get_combo_operand();
            program.a = program.a / 2i64.pow(operand as u32);
            program.ip += 2;
        }

        if opcode == BXL {
            let operand = program.get_literal_operand();
            program.b = program.b ^ operand;
            program.ip += 2;
        }

        if opcode == BST {
            let operand = program.get_combo_operand();
            program.b = operand % 8;
            program.ip += 2;
        }

        if opcode == JNZ {
            let operand = program.get_literal_operand();
            if program.a != 0 {
                program.ip = operand as usize;
            } else {
                program.ip += 2;
            }
        }

        if opcode == BXC {
            let _operand = program.get_literal_operand(); // ignored, therefore may be invalid combo
            program.b = program.b ^ program.c;
            program.ip += 2;
        }

        if opcode == OUT {
            let operand = program.get_combo_operand();
            output.push((operand % 8) as u8);
            program.ip += 2;
        }

        if opcode == BDV {
            let operand = program.get_combo_operand();
            program.b = program.a / 2i64.pow(operand as u32);
            program.ip += 2;
        }

        if opcode == CDV {
            let operand = program.get_combo_operand();
            program.c = program.a / 2i64.pow(operand as u32);
            program.ip += 2;
        }
    }

    output
}

#[advent_of_code(2024, 17, 1)]
pub fn solve_level1(input: &[&str]) -> String {
    let out = format!("{:?}", run(parse_input(input)));

    str::replace(
        &str::replace(
            &str::replace(&out, " ", ""),
            "[", ""),
        "]", "").to_string()
}

// Lucked out. This one works only on my own input
// (and perhaps some others, but not the ones I've tried)
#[advent_of_code(2024, 17, 2)]
pub fn solve_level2(input: &[&str]) -> i64 {
    let mut program = parse_input(input);
    let orig = program.program.clone();

    let mut a = 1;

    // find an answer of the same length
    loop {
        program.a = a;
        let result = run(program.clone());
        if result.len() == orig.len() {
            break;
        }
        a *= 2;
    }

    // println!("{}", a);

    // Find a significant subset
    let digits = 7;
    loop {
        program.a = a;
        let result = run(program.clone());

        assert_eq!(result.len(), orig.len());

        if result.iter().take(digits).collect::<Vec<_>>() == orig.iter().take(digits).collect::<Vec<_>>() {
            // println!("{}: {:?}",a, result);
            break;
        }

        a += 1
    }

    // Brute force
    loop {
        program.a = a;
        let result = run(program.clone());

        assert_eq!(result.len(), orig.len());

        if result == orig {
            return a;
        }

        a += 1024 * 1024 * 1024;
    }
}

fn find_quine(program: Puter, index: usize, partial: i64) -> i64 {
    if index == 0 { return partial; }
    for octal in 0..8 {
        let res = run(Puter { a: partial * 8 + octal,
                              b: program.b,
                              c: program.c,
                              program: program.program.clone(),
                              ip: 0 });
        if res == program.program[index - 1..] {
            let res = find_quine(program.clone(), index - 1, partial * 8 + octal);
            if res != -1 { return res; }
        }
    }
    -1
}

#[advent_of_code(2024, 17, 2)]
pub fn solve_level2_recurse(input: &[&str]) -> i64 {
    let p = parse_input(input);
    find_quine(p.clone(), p.program.len(), 0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/17-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/17-sample.out").unwrap());

    static SAMPLE2: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/17-sample2.in").unwrap());
    static SAMPLE2_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/17-sample2.out").unwrap());

    #[test]
    fn test_level1() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().next().unwrap();
        assert_eq!(format!("{}", solve_level1(&input)), expected);
    }

    #[test]
    fn test_level2() {
        let input: Vec<_> = (*SAMPLE2).lines().collect();
        let expected = (*SAMPLE2_OUT).lines().skip(3).next().unwrap();
        assert_eq!(format!("{}", solve_level2(&input)), expected);
    }
}
