use itertools::Itertools;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 2, solve_all)
}

fn check_report_safety(report: &[i32], bad_level_tol: usize) -> bool {
    let is_safe = |values: &[i32]| {
        values
            .iter()
            .all(|&value| (1..4).contains(&value.abs()) && value.signum() == values[0].signum())
    };

    let mut combinations = report.iter().combinations(report.len() - bad_level_tol);
    combinations.any(|combination| {
        let diffs = combination
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect::<Vec<_>>();
        is_safe(&diffs)
    })
}

fn count_safe_reports(reports: &[Vec<i32>], bad_level_tol: usize) -> usize {
    reports
        .iter()
        .filter(|report| check_report_safety(report, bad_level_tol))
        .count()
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Vec<i32>>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|part| part.parse::<i32>())
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| e.into())
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let reports = parse_input(input)?;
    let part_one = count_safe_reports(&reports, 0).to_string();
    let part_two = count_safe_reports(&reports, 1).to_string();
    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    use super::count_safe_reports;
    use super::parse_input;
    use utils::load_test_input;

    #[test]
    fn part_one() {
        let input = load_test_input!();
        let reports = parse_input(&input).unwrap();
        let solution = count_safe_reports(&reports, 0);
        assert_eq!(solution, 2);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let reports = parse_input(&input).unwrap();
        let solution = count_safe_reports(&reports, 1);
        assert_eq!(solution, 4);
    }
}
