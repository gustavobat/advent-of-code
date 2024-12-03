use regex::Regex;

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
    let mut operations = Vec::new();

    for cap in regex.captures_iter(input) {
        if let Some(m) = cap.get(0) {
            if m.as_str().starts_with("mul") {
                let x: usize = cap[1].parse()?;
                let y: usize = cap[2].parse()?;
                operations.push(Operation::Mul(x, y));
            } else if m.as_str() == "do()" {
                operations.push(Operation::Do);
            } else if m.as_str() == "don't()" {
                operations.push(Operation::Dont);
            }
        }
    }

    Ok(operations)
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("data/03.input")?;
    let input = parse_input(&input)?;
    let part1 = solve_part_one(&input);
    let part2 = solve_part_two(&input);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/03.test1").unwrap();
        let input = parse_input(&input).unwrap();
        let part1 = solve_part_one(&input);
        assert_eq!(part1, 161);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/03.test2").unwrap();
        let input = parse_input(&input).unwrap();
        let part2 = solve_part_two(&input);
        assert_eq!(part2, 48);
    }
}
