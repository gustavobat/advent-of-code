use regex::Regex;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 3, solve_all)
}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Do,
    Dont,
    Mul(usize, usize),
}

fn solve_part_one(input: &[Operation]) -> usize {
    input
        .iter()
        .filter_map(|op| match op {
            Operation::Mul(lhs, rhs) => Some(lhs * rhs),
            _ => None,
        })
        .sum()
}

fn solve_part_two(input: &[Operation]) -> usize {
    let mut should_count = true;
    let mut total = 0;
    for op in input {
        match op {
            Operation::Do => should_count = true,
            Operation::Dont => should_count = false,
            Operation::Mul(lhs, rhs) => {
                if should_count {
                    total += lhs * rhs;
                }
            }
        }
    }
    total
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Operation>> {
    let regex = Regex::new(r"mul\((-?\d+),(-?\d+)\)|do\(\)|don't\(\)")?;

    let operations = regex
        .captures_iter(input)
        .map(|cap| {
            let op = cap.get(0).ok_or(anyhow::anyhow!("No match found"))?;
            if op.as_str().strip_prefix("mul").is_some() {
                let x: usize = cap[1].parse()?;
                let y: usize = cap[2].parse()?;
                Ok(Operation::Mul(x, y))
            } else if op.as_str() == "do()" {
                Ok(Operation::Do)
            } else if op.as_str() == "don't()" {
                Ok(Operation::Dont)
            } else {
                Err(anyhow::anyhow!("Unknown operation"))
            }
        })
        .collect::<anyhow::Result<Vec<Operation>>>()?;

    Ok(operations)
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
        let input = load_test_input!(1);
        let operations = parse_input(&input).unwrap();
        let solution = solve_part_one(&operations);
        assert_eq!(solution, 161);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!(2);
        let operations = parse_input(&input).unwrap();
        let solution = solve_part_two(&operations);
        assert_eq!(solution, 48);
    }
}
