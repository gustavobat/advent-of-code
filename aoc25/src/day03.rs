use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 3, solve_all)
}

fn digits_to_u64(v: Vec<u32>) -> u64 {
    v.into_iter().fold(0, |acc, x| acc * 10 + x as u64)
}

fn bank_joltage(bank: &[u32], n_required_digits: usize) -> u64 {
    let mut turned_on = Vec::with_capacity(n_required_digits);
    let bank_len = bank.len();
    for (i, battery_joltage) in bank.iter().enumerate() {
        let n_batteries_available = bank_len - i;
        while turned_on.last().is_some_and(|last| battery_joltage > last) {
            if n_required_digits >= turned_on.len() + n_batteries_available {
                // We can't pop, otherwise we won't have enough batteries
                break;
            }
            turned_on.pop();
        }
        if turned_on.len() < n_required_digits {
            turned_on.push(*battery_joltage);
        }
    }
    digits_to_u64(turned_on)
}

fn solve_part_one(battery_banks: &[Vec<u32>]) -> u64 {
    battery_banks.iter().map(|bank| bank_joltage(bank, 2)).sum()
}

fn solve_part_two(battery_banks: &[Vec<u32>]) -> u64 {
    battery_banks
        .iter()
        .map(|bank| bank_joltage(bank, 12))
        .sum()
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<u32>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| anyhow::anyhow!("Invalid digit"))
                })
                .collect::<Result<Vec<_>, _>>()
        })
        .collect()
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let batteries = parse_input(input)?;
    let part_one = solve_part_one(&batteries).to_string();
    let part_two = solve_part_two(&batteries).to_string();
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
        let battery_banks = parse_input(&input).unwrap();
        let solution = solve_part_one(&battery_banks);
        assert_eq!(solution, 357);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let battery_banks = parse_input(&input).unwrap();
        let solution = solve_part_two(&battery_banks);
        assert_eq!(solution, 3121910778619);
    }
}
