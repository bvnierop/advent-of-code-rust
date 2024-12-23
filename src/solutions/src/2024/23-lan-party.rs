#![allow(unused_imports)]
#![allow(unused_variables)]

use aoc_macros::advent_of_code;
use inventory;
use scan_fmt::scan_fmt;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use im::{Vector,vector};
use im::HashSet as ImHashSet;
use std::collections::VecDeque;

type Graph = FxHashMap<String, FxHashSet<String>>;
fn connects(graph: &Graph, current: &ImHashSet<String>, node: &String) -> bool {
    current.iter().all(|c| {
        graph[c].contains(node)
    })
}

pub fn find_cliques(graph: &Graph) -> Vector<ImHashSet<String>> {
    let nodes: Vec<String> = graph.keys().cloned().collect();
    dfs(graph, &nodes, 0, ImHashSet::new(), vector![])
}

fn dfs(graph: &Graph, nodes: &Vec<String>, idx: usize, cur: ImHashSet<String>, acc: Vector<ImHashSet<String>>) -> Vector<ImHashSet<String>> {
    if cur.len() == 3 {
        let mut res = acc.clone();
        res.push_back(cur);
        return res;
    }

    if idx == nodes.len() {
        return acc;
    }

    // exclude cur
    let mut res = dfs(graph, nodes, idx + 1, cur.clone(), acc.clone());

    // include cur
    let node = nodes[idx].clone();
    if connects(graph, &cur, &node) { // only if it connects
        let ncur = cur.update(node.clone());
        res = dfs(graph, nodes, idx + 1, ncur, res.clone());
    }
    res
}

#[advent_of_code(2024, 23, 1)]
pub fn solve_level1(input: &[&str]) -> usize {
    let mut adj: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();
    for line in input {
        let (a, b) = scan_fmt!(line, "{}-{}", String, String).unwrap();
        adj.entry(a.clone()).or_default().insert(b.clone());
        adj.entry(b).or_default().insert(a);
    }


    let c = find_cliques(&adj);

    c.iter().filter(|cl| cl.iter().any(|s| s.starts_with("t")))
    .count()
}

pub fn find_max_clique(graph: &Graph) -> ImHashSet<String> {
    let nodes: Vec<String> = graph.keys().cloned().collect();
    dfs2(graph, &nodes, 0, ImHashSet::new(), ImHashSet::new())
}

fn dfs2(graph: &Graph, nodes: &Vec<String>, idx: usize, cur: ImHashSet<String>, best: ImHashSet<String>) -> ImHashSet<String> {
    if idx == nodes.len() {
        return if cur.len() > best.len() { cur } else { best };
    }

    // if we can no longer improve, stop
    let nodes_left = nodes.len() - idx;
    if cur.len() + nodes_left <= best.len() {
        return best;
    }

    // exclude cur
    let mut res = dfs2(graph, nodes, idx + 1, cur.clone(), best.clone());

    // include cur
    let node = nodes[idx].clone();
    if connects(graph, &cur, &node) { // only if it connects
        let ncur = cur.update(node.clone());
        res = dfs2(graph, nodes, idx + 1, ncur, res.clone());
    }
    res
}

#[advent_of_code(2024, 23, 2)]
pub fn solve_level2(input: &[&str]) -> String {
    let mut adj: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();
    for line in input {
        let (a, b) = scan_fmt!(line, "{}-{}", String, String).unwrap();
        adj.entry(a.clone()).or_default().insert(b.clone());
        adj.entry(b).or_default().insert(a);
    }

    let c = find_max_clique(&adj);
    c.into_iter().sorted().join(",").to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::LazyLock;

    static SAMPLE: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/23-sample.in").unwrap());
    static SAMPLE_OUT: LazyLock<String> = LazyLock::new(|| fs::read_to_string("../../input/2024/23-sample.out").unwrap());

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
