use anyhow::Result;
use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet, HashSet};

type Graph = BTreeMap<String, Vec<String>>;

fn build_graph(pairs: &[(String, String)]) -> Graph {
    let mut graph = BTreeMap::new();
    for (from, to) in pairs {
        graph
            .entry(from.clone())
            .or_insert_with(Vec::new)
            .push(to.clone());
        graph
            .entry(to.clone())
            .or_insert_with(Vec::new)
            .push(from.clone());
    }
    graph
}

fn find_cycles(graph: &Graph, cycle_len: usize) -> BTreeSet<BTreeSet<String>> {
    let mut cycles = BTreeSet::new();
    let mut visited = HashSet::new();

    for node in graph.keys() {
        visited.insert(node.clone());
        let neighbors = &graph[node];
        let iter = std::iter::once(node).chain(neighbors);
        let combinations = iter.combinations(cycle_len);
        for combination in combinations {
            if combination.iter().all(|&node| visited.contains(node)) {
                continue;
            }
            if is_cycle(graph, &combination) {
                let mut new_set = BTreeSet::new();
                for node in combination {
                    new_set.insert(node.clone());
                }
                cycles.insert(new_set);
            }
        }
    }
    cycles
}

fn is_cycle(graph: &Graph, elements: &[&String]) -> bool {
    elements
        .iter()
        .combinations(2)
        .all(|v| graph[*v[0]].contains(*v[1]))
}

fn parse_input(input: &str) -> Result<Graph> {
    let pairs = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('-').collect();
            (parts[0].to_string(), parts[1].to_string())
        })
        .collect::<Vec<_>>();
    Ok(build_graph(&pairs))
}

fn solve_part_one(graph: &Graph) -> usize {
    let cycles = find_cycles(graph, 3);
    cycles
        .iter()
        .filter(|cycle| cycle.iter().any(|node| node.starts_with('t')))
        .count()
}

fn solve_part_two(graph: &Graph) -> String {
    let mut max = 3;
    let mut max_cycle = String::new();
    for (node, neighbors) in graph {
        let mut all = vec![node.clone()];
        all.extend_from_slice(neighbors.as_slice());
        let cur_range = max..=neighbors.len();
        for i in cur_range {
            let combinations = all.iter().combinations(i);
            for combination in combinations {
                if is_cycle(graph, &combination) && i > max {
                    max = i;
                    max_cycle = combination.iter().sorted().join(",");
                }
            }
        }
    }
    max_cycle
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/23.input")?;
    let pairs = parse_input(&input)?;

    let part1 = solve_part_one(&pairs);
    let part2 = solve_part_two(&pairs);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day23 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/23.test").unwrap();
        let pairs = parse_input(&input).unwrap();
        let part1 = solve_part_one(&pairs);
        assert_eq!(part1, 7);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/23.test").unwrap();
        let pairs = parse_input(&input).unwrap();
        let part2 = solve_part_two(&pairs);
        assert_eq!(part2, "co,de,ka,ta".to_string());
    }
}
