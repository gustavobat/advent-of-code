use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Equation {
    result: usize,
    args: Vec<usize>,
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Add,
    Mul,
    Concat,
}

impl Equation {
    fn is_possible(&self, allowed_ops: &[Op]) -> bool {
        let n_args = self.args.len();
        let n_ops = n_args - 1;
        let mut combinations = (0..n_ops)
            .map(|_| allowed_ops.iter())
            .multi_cartesian_product();
        combinations.any(|ops| {
            let mut result = self.args[0];
            for (op, arg) in ops.iter().zip(self.args.iter().skip(1)) {
                match op {
                    Op::Add => result += arg,
                    Op::Mul => result *= arg,
                    Op::Concat => result = format!("{}{}", result, arg).parse().unwrap(),
                }
            }
            result == self.result
        })
    }
}

fn solve_part_one(input: &[Equation]) -> usize {
    input
        .iter()
        .filter_map(|eq| {
            if eq.is_possible(&[Op::Add, Op::Mul]) {
                Some(eq.result)
            } else {
                None
            }
        })
        .sum()
}

fn solve_part_two(input: &[Equation]) -> usize {
    input
        .iter()
        .filter_map(|eq| {
            if eq.is_possible(&[Op::Add, Op::Mul, Op::Concat]) {
                Some(eq.result)
            } else {
                None
            }
        })
        .sum()
}

fn parse_input(input: &str) -> Result<Vec<Equation>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let result = parts
                .next()
                .ok_or_else(|| anyhow!("Missing result part"))?
                .trim()
                .parse::<usize>()
                .map_err(|e| anyhow!("Failed to parse result value: {e}"))?;
            let args = parts
                .next()
                .ok_or_else(|| anyhow!("Missing arguments part"))?
                .split_whitespace()
                .map(|s| {
                    s.parse::<usize>()
                        .map_err(|e| anyhow!("Failed to parse argument value: {e}"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Equation { result, args })
        })
        .collect::<Result<Vec<_>, _>>()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/07.input")?;
    let input = parse_input(&input)?;

    let part1 = solve_part_one(&input);
    let part2 = solve_part_two(&input);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day07 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/07.test").unwrap();
        let input = parse_input(&input).unwrap();
        let part1 = solve_part_one(&input);
        assert_eq!(part1, 3749);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/07.test").unwrap();
        let input = parse_input(&input).unwrap();
        let part2 = solve_part_two(&input);
        assert_eq!(part2, 11387);
    }
}
