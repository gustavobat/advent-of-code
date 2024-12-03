use anyhow::anyhow;
use itertools::Itertools;

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
                .map_err(|e| anyhow!("Failed to parse number: {e}"))
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("data/02.input")?;
    let reports = parse_input(&input)?;
    let part1 = count_safe_reports(&reports, 0);
    let part2 = count_safe_reports(&reports, 1);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/02.test").unwrap();
        let list = parse_input(&input).unwrap();
        let part1 = count_safe_reports(&list, 0);
        assert_eq!(part1, 2);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/02.test").unwrap();
        let list = parse_input(&input).unwrap();
        let part2 = count_safe_reports(&list, 1);
        assert_eq!(part2, 4);
    }
}
