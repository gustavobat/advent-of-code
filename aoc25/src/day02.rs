use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 2, solve_all)
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let list = parse_input(input)?;
    let part_one = solve_part_one(&list).to_string();
    let part_two = solve_part_two(&list).to_string();
    Ok(Solution { part_one, part_two })
}

fn test_repeating_pattern(id: u64, pattern_len: u64) -> bool {
    let id_len = id.ilog10() + 1;
    let mut rest_len = id_len - pattern_len as u32;
    let reference_seq = id / 10_u64.pow(rest_len);
    let mut rest = id % 10_u64.pow(rest_len);
    while rest_len > 0 {
        rest_len -= pattern_len as u32;
        let new_seq = rest / 10_u64.pow(rest_len);
        if new_seq != reference_seq {
            return false;
        }
        rest %= 10_u64.pow(rest_len);
    }
    true
}

fn is_invalid_id(id: u64, max_repeats: Option<u64>) -> bool {
    let id_len = (id.ilog10() + 1) as u64;
    let max_repeats = max_repeats.unwrap_or(id_len);
    for n_repeats in 2..=max_repeats {
        if !id_len.is_multiple_of(n_repeats) {
            continue;
        }
        let pattern_len = id_len / n_repeats;
        if test_repeating_pattern(id, pattern_len) {
            return true;
        }
    }
    false
}

fn solve_part_one(id_ranges: &[(u64, u64)]) -> u64 {
    id_ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&id| is_invalid_id(id, Some(2)))
        .sum()
}

fn solve_part_two(id_ranges: &[(u64, u64)]) -> u64 {
    id_ranges
        .iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&id| is_invalid_id(id, None))
        .sum()
}

fn parse_input(input: &str) -> anyhow::Result<Vec<(u64, u64)>> {
    let ranges = input
        .split(',')
        .filter_map(|part| {
            let part = part.trim();
            if part.is_empty() {
                return None;
            }
            part.split_once('-').and_then(|(a, b)| {
                let start = a.parse::<u64>().ok()?;
                let end = b.parse::<u64>().ok()?;
                Some((start, end))
            })
        })
        .collect();
    Ok(ranges)
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
        let id_ranges = parse_input(&input).unwrap();
        let solution = solve_part_one(&id_ranges);
        assert_eq!(solution, 1227775554);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let id_ranges = parse_input(&input).unwrap();
        let solution = solve_part_two(&id_ranges);
        assert_eq!(solution, 4174379265);
    }
}
