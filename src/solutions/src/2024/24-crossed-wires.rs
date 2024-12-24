#![allow(unused_imports)]
#![allow(unused_variables)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use im::HashMap as ImHashMap;
use im::HashSet as ImHashSet;
use std::mem::swap;

pub fn solve(wire: &String, wires: ImHashMap<String, u8>, gates: &FxHashMap<String, (String, String, String)>) -> (u8, ImHashMap<String, u8>) {
    if wires.contains_key(wire) {
        return (wires[wire], wires);
    }

    let (inp1, inp2, op) = &gates[wire];

    let (ans1, w) = solve(inp1, wires, gates);
    let (ans2, w) = solve(inp2, w, gates);

    let ans = if op == "AND" { ans1 & ans2 }
    else if op == "OR" { ans1 | ans2 }
    else if op == "XOR" { ans1 ^ ans2 }
    else { panic!("Invalid op"); };

    let updated_wires = w.update(wire.clone(), ans);

    return (ans, updated_wires);
}

#[advent_of_code(2024, 24, 1)]
pub fn solve_level1(input: &[&str]) -> u64 {
    let mut inp = input.split(|l| l.is_empty());

    let init = inp.next().unwrap().into_iter().map(|l| scan_fmt!(l, "{}: {}", String, u8).unwrap()).collect_vec();
    let gates_i = inp.next().unwrap().into_iter().map(|l| scan_fmt!(l, "{} {} {} -> {}", String, String, String, String).unwrap()).collect_vec();

    let mut wires: ImHashMap<String, u8> = ImHashMap::new();
    for (w, v) in init {
        wires.insert(w, v);
    }

    // store gates as output = in1, in2, op
    let mut gates: FxHashMap<String, (String, String, String)> = FxHashMap::default();

    let mut targets: Vec<String> = vec![];
    for (inp1, op, inp2, out) in gates_i {
        gates.insert(out.clone(), (inp1, inp2, op));
        if out.starts_with("z") {
            targets.push(out.clone());
        }
    }

    // The input only contains unique outputs, so each output must be
    // calculated, but only has to be calculated once
    let mut w = wires.clone();
    let mut res = 0u64;
    for t in targets.iter().sorted().rev() {
        let (a, nw) = solve(t, w.clone(), &gates);
        w = nw;
        res <<= 1;
        res |= a as u64;
    }


    res
}


#[advent_of_code(2024, 24, 2)]
pub fn solve_level2(input: &[&str]) -> String {
    let mut inp = input.split(|l| l.is_empty());

    let init = inp.next().unwrap().into_iter().map(|l| scan_fmt!(l, "{}: {}", String, u8).unwrap()).collect_vec();
    let gates = inp.next().unwrap().into_iter().copied().collect_vec();

    let swapped = swap_all(45, &gates);


    // by hand
    let ans = ["gpr", "z10", "nks", "z21", "ghp", "z33", "krs", "cpm"];
    let ans = ans.iter().sorted().join(",");

    let res = swapped.iter().flat_map(|(a, b)| [*a, *b])
        .sorted()
        .join(",");

    assert!(ans == res);
    res
}


fn find<'a>(a: Option<&str>, operator: &str, b: Option<&str>, gates: &[&'a str]) -> Option<&'a str> {
    match (a, b) {
        (Some(aa), Some(bb)) =>
            gates
            .iter()
            .find(|gate| {
                gate.starts_with(&format!("{} {} {}", aa, operator, bb))
                    || gate.starts_with(&format!("{} {} {}", bb, operator, aa))
            })
            .and_then(|gate| {
                // Split on " -> " and take the last piece
                gate.split(" -> ").last().map(|s| s)
            }),
        _ => None
    }
}

fn swap_all<'a>(bits: i32, gates: &[&'a str]) -> Vec<(&'a str, &'a str)> {
    let mut swapped = Vec::new();

    // carry
    let mut ci: Option<&str> = None;

    // verify and fix bitwise logic adder
    //   SumBit = x XOR y XOR CarryIn
    //   x XOR y  -> n
    //   n XOR ci -> z
    //
    //   CarryBit = (x AND y) OR (CarryIn AND (x XOR y))
    //   x AND y  -> m
    //   n AND ci -> i
    //   m OR  i  -> co
    //   co -> ci

    for i in 0..45 {
        let num = format!("{:02}", i);
        let mut n_out = find(Some(&format!("x{}", num)), "XOR", Some(&format!("y{}", num)), gates);
        let mut m_out = find(Some(&format!("x{}", num)), "AND", Some(&format!("y{}", num)), gates);

        // half-adder logic with carry
        if ci.is_some() {

            // check that M and N are not swapped
            // M/N are swapped if `ci AND n` does not exist.
            let mut i_out = find(n_out, "AND", ci, gates);
            if i_out.is_none() {
                // swap refs for checks ahead
                swap(&mut n_out, &mut m_out);
                swapped.push((n_out.unwrap(), m_out.unwrap()));

                // we need `i` for later checks
                i_out = find(n_out, "AND", ci, gates);
            }

            // at this point M and N are correct relative to each other.
            // they may still be swapped with z.
            let mut z_out = find(n_out, "XOR", ci, gates);

            // Z swapped with one of N, M or X.
            if !z_out.unwrap().starts_with('z') {
                if m_out.unwrap().starts_with('z') {
                    swap(&mut m_out, &mut z_out);
                    swapped.push((m_out.unwrap(), z_out.unwrap()));
                } else if n_out.unwrap().starts_with('z') {
                    swap(&mut n_out, &mut z_out);
                    swapped.push((n_out.unwrap(), z_out.unwrap()));
                } else if i_out.is_some() && i_out.unwrap().starts_with('z') {
                    swap(&mut i_out, &mut z_out);
                    swapped.push((i_out.unwrap(), z_out.unwrap()));
                }
            }


            let mut co_out = match (i_out, m_out) {
                (Some(rr), Some(mm)) => find(m_out, "OR", i_out, gates),
                _ => None
            };

            // CarryOut can start with a 'z' only if it's the last bit,
            // otherwise it's swapped with Z
            if co_out.is_some() && co_out.unwrap().starts_with('z') && co_out.unwrap() != "z45" {
                swap(&mut co_out, &mut z_out);
                swapped.push((co_out.unwrap(), z_out.unwrap()));
            }

            ci = co_out;
        } else {
            ci = m_out;
        }
    }

    swapped
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/24-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/24-sample.out").unwrap());

    #[test]
    fn test_level1() {
        let input: Vec<_> = (*SAMPLE).lines().collect();
        let expected = (*SAMPLE_OUT).lines().next().unwrap();
        assert_eq!(format!("{}", solve_level1(&input)), expected);
    }
}
