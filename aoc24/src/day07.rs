use anyhow::anyhow;
use itertools::Itertools;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 7, solve_all)
}

#[derive(Debug, Clone)]
struct Equation {
    result: u64,
    args: Vec<u32>,
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
            let mut result = self.args[0] as u64;
            for (op, &arg) in ops.iter().zip(self.args.iter().skip(1)) {
                match op {
                    Op::Add => result += arg as u64,
                    Op::Mul => result *= arg as u64,
                    Op::Concat => result = result * 10u64.pow(arg.ilog10() + 1) + arg as u64,
                }
            }
            result == self.result
        })
    }
}

fn solve_part_one(input: &[Equation]) -> u64 {
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

fn solve_part_two(input: &[Equation]) -> u64 {
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

fn parse_input(input: &str) -> anyhow::Result<Vec<Equation>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let result = parts
                .next()
                .ok_or_else(|| anyhow!("Missing result part"))?
                .trim()
                .parse::<u64>()
                .map_err(|e| anyhow!("Failed to parse result value: {e}"))?;
            let args = parts
                .next()
                .ok_or_else(|| anyhow!("Missing arguments part"))?
                .split_whitespace()
                .map(|s| {
                    s.parse::<u32>()
                        .map_err(|e| anyhow!("Failed to parse argument value: {e}"))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Equation { result, args })
        })
        .collect::<Result<Vec<_>, _>>()
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let input = parse_input(input)?;
    let part_one = solve_part_one(&input).to_string();
    let part_two = solve_part_two(&input).to_string();

    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::solve_part_one;
    use super::solve_part_two;
    use utils::load_test_input;

    #[test]
    fn part_one() {
        let input = load_test_input!();
        let equations = parse_input(&input).unwrap();
        let solution = solve_part_one(&equations);
        assert_eq!(solution, 3749);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let equations = parse_input(&input).unwrap();
        let solution = solve_part_two(&equations);
        assert_eq!(solution, 11387);
    }
}
