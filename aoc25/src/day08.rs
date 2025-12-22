use anyhow::anyhow;
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 8, solve_all)
}

#[derive(Debug, Clone, Copy)]
struct Point3D {
    x: u64,
    y: u64,
    z: u64,
}

impl Point3D {
    fn euclid_dist(&self, other: &Self) -> u64 {
        let dx = self.x.abs_diff(other.x);
        let dy = self.y.abs_diff(other.y);
        let dz = self.z.abs_diff(other.z);
        dx * dx + dy * dy + dz * dz
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct Pair {
    index_a: usize,
    index_b: usize,
    dist: u64,
}

impl Ord for Pair {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}

impl PartialOrd for Pair {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone, Copy)]
struct Node {
    parent: usize,
    size: usize,
}

#[derive(Debug, Clone)]
struct DisjointSet {
    nodes: Vec<Node>,
}

impl DisjointSet {
    fn new(n: usize) -> Self {
        let nodes = (0..n)
            .map(|value| Node {
                parent: value,
                size: 1,
            })
            .collect_vec();
        Self { nodes }
    }

    fn find(&mut self, mut value: usize) -> usize {
        while self.nodes[value].parent != value {
            let parent = self.nodes[value].parent;
            self.nodes[value].parent = self.nodes[parent].parent;
            value = parent;
        }
        value
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut a = self.find(a);
        let mut b = self.find(b);
        if a != b {
            if self.nodes[a].size < self.nodes[b].size {
                (a, b) = (b, a);
            }
            self.nodes[b].parent = a;
            self.nodes[a].size += self.nodes[b].size;
        }
    }

    fn is_fully_connected(&self) -> bool {
        self.nodes.iter().any(|node| node.size == self.nodes.len())
    }
}

fn build_connection_order(points: &[Point3D]) -> BinaryHeap<Reverse<Pair>> {
    (0..points.len())
        .array_combinations::<2>()
        .map(|pair| {
            Reverse(Pair {
                index_a: pair[0],
                index_b: pair[1],
                dist: points[pair[0]].euclid_dist(&points[pair[1]]),
            })
        })
        .collect::<BinaryHeap<_>>()
}

fn solve_part_one(
    points: &[Point3D],
    pairs: &mut BinaryHeap<Reverse<Pair>>,
    max_connections: usize,
) -> anyhow::Result<u64> {
    if points.len() < 2 {
        return Err(anyhow!("Not enough points to form pairs"));
    }

    let mut circuits = DisjointSet::new(points.len());

    for _ in 0..max_connections {
        let Reverse(pair) = pairs
            .pop()
            .ok_or_else(|| anyhow!("Ran out of pairs while connecting components"))?;
        circuits.union(pair.index_a, pair.index_b);
    }

    let mut counts = circuits
        .nodes
        .iter()
        .map(|node| node.size as u64)
        .collect_vec();
    counts.sort();
    Ok(counts.iter().rev().take(3).product())
}

fn solve_part_two(points: &[Point3D], mut pairs: BinaryHeap<Reverse<Pair>>) -> anyhow::Result<u64> {
    if points.len() < 2 {
        return Err(anyhow!("Not enough points to form pairs"));
    }

    let mut circuits = DisjointSet::new(points.len());

    let Reverse(mut pair) = pairs.pop().ok_or_else(|| anyhow!("No pairs available"))?;

    loop {
        circuits.union(pair.index_a, pair.index_b);
        if circuits.is_fully_connected() {
            break;
        }
        let Reverse(next_pair) = pairs
            .pop()
            .ok_or_else(|| anyhow!("Ran out of pairs before full connectivity"))?;
        pair = next_pair;
    }

    let x_a = points[pair.index_a].x;
    let x_b = points[pair.index_b].x;
    Ok(x_a * x_b)
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Point3D>> {
    input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(',').map(str::trim).collect();
            if parts.len() != 3 {
                return Err(anyhow!("Fail parsing line '{}'.", line));
            }
            let x = parts[0]
                .parse::<u64>()
                .map_err(|e| anyhow!("Failed parsing x '{}': {}", parts[0], e))?;
            let y = parts[1]
                .parse::<u64>()
                .map_err(|e| anyhow!("Failed parsing y '{}': {}", parts[1], e))?;
            let z = parts[2]
                .parse::<u64>()
                .map_err(|e| anyhow!("Failed parsing z '{}': {}", parts[2], e))?;
            Ok(Point3D { x, y, z })
        })
        .collect()
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let points = parse_input(input)?;
    let mut pairs = build_connection_order(&points);

    let part_one = solve_part_one(&points, &mut pairs, 1000)?.to_string();
    let part_two = solve_part_two(&points, pairs)?.to_string();

    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    use super::build_connection_order;
    use super::parse_input;
    use super::solve_part_one;
    use super::solve_part_two;
    use utils::load_test_input;

    #[test]
    fn part_one() {
        let input = load_test_input!();
        let points = parse_input(&input).unwrap();
        let mut pairs = build_connection_order(&points);
        let solution = solve_part_one(&points, &mut pairs, 10).unwrap();
        assert_eq!(solution, 40);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let points = parse_input(&input).unwrap();
        let pairs = build_connection_order(&points);
        let solution = solve_part_two(&points, pairs).unwrap();
        assert_eq!(solution, 25272);
    }
}
