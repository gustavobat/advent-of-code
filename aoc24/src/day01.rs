use anyhow::anyhow;
use itertools::Itertools;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 1, solve_all)
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let list = parse_input(input)?;
    let part_one = solve_part_one(&list).to_string();
    let part_two = solve_part_two(&list).to_string();
    Ok(Solution { part_one, part_two })
}

fn solve_part_one(list: &[(usize, usize)]) -> u32 {
    let (first, second): (Vec<_>, Vec<_>) = list.iter().copied().unzip();
    let first = first.iter().enumerate().sorted_by_key(|(_, a)| *a);
    let second = second.iter().enumerate().sorted_by_key(|(_, b)| *b);
    first
        .zip(second)
        .map(|((_, lhs), (_, rhs))| (*lhs as i32 - *rhs as i32).unsigned_abs())
        .sum()
}

fn solve_part_two(list: &[(usize, usize)]) -> usize {
    let (first, second): (Vec<_>, Vec<_>) = list.iter().copied().unzip();
    let counts = second.iter().counts();
    first
        .iter()
        .map(|&a| a * counts.get(&a).unwrap_or(&0))
        .sum()
}

fn parse_input(input: &str) -> anyhow::Result<Vec<(usize, usize)>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let a = parts
                .next()
                .ok_or(anyhow!("Missing first number"))?
                .parse::<usize>()?;
            let b = parts
                .next()
                .ok_or(anyhow!("Missing second number"))?
                .parse::<usize>()?;
            Ok((a, b))
        })
        .collect::<anyhow::Result<Vec<_>>>()
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
        let list = parse_input(&input).unwrap();
        let solution = solve_part_one(&list);
        assert_eq!(solution, 11);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let list = parse_input(&input).unwrap();
        let solution = solve_part_two(&list);
        assert_eq!(solution, 31);
    }
}
