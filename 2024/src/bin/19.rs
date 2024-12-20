use anyhow::Result;
use std::collections::HashMap;

fn count_possible_ways(
    design: &str,
    patterns: &[String],
    memo: &mut HashMap<String, usize>,
) -> usize {
    if design.is_empty() {
        return 1;
    }
    if let Some(&count) = memo.get(design) {
        return count;
    }
    let count = patterns
        .iter()
        .filter_map(|p| design.strip_prefix(p))
        .map(|remaining| count_possible_ways(remaining, patterns, memo))
        .sum();
    memo.insert(design.to_string(), count);
    count
}

fn parse_input(input: &str) -> Result<(Vec<String>, Vec<String>)> {
    let (patterns, designs) = input
        .split_once("\n\n")
        .ok_or(anyhow::anyhow!("Invalid input format"))?;
    let patterns = patterns.split(", ").map(str::to_string).collect();
    let designs = designs.lines().map(str::to_string).collect();
    Ok((patterns, designs))
}

fn solve_part_one(patterns: &[String], designs: &[String]) -> usize {
    let mut memo = HashMap::new();
    designs
        .iter()
        .filter(|design| count_possible_ways(design, patterns, &mut memo) > 0)
        .count()
}

fn solve_part_two(patterns: &[String], designs: &[String]) -> usize {
    let mut memo = HashMap::new();
    designs
        .iter()
        .map(|design| count_possible_ways(design, patterns, &mut memo))
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/19.input")?;
    let (patterns, designs) = parse_input(&input)?;

    let part1 = solve_part_one(&patterns, &designs);
    let part2 = solve_part_two(&patterns, &designs);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day19 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/19.test").unwrap();
        let (patterns, designs) = parse_input(&input).unwrap();
        let part1 = solve_part_one(&patterns, &designs);
        assert_eq!(part1, 6);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/19.test").unwrap();
        let (patterns, designs) = parse_input(&input).unwrap();
        let part2 = solve_part_two(&patterns, &designs);
        assert_eq!(part2, 16);
    }
}
