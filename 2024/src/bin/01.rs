use anyhow::anyhow;
use itertools::Itertools;

fn solve_part_one(list: &[(usize, usize)]) -> u32 {
    let first = list
        .iter()
        .map(|(a, _)| a)
        .enumerate()
        .sorted_by_key(|(_, a)| *a)
        .collect::<Vec<_>>();
    let second = list
        .iter()
        .map(|(_, b)| b)
        .enumerate()
        .sorted_by_key(|(_, b)| *b)
        .collect::<Vec<_>>();
    first
        .iter()
        .zip(second.iter())
        .map(|((_, &lhs), (_, &rhs))| (lhs as i32 - rhs as i32).unsigned_abs())
        .sum()
}

fn solve_part_two(list: &[(usize, usize)]) -> usize {
    let first = list.iter().map(|(a, _)| a).collect::<Vec<_>>();
    let second = list.iter().map(|(_, b)| b).collect::<Vec<_>>();
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

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("data/01.input")?;
    let list = parse_input(&input)?;
    let part1 = solve_part_one(&list);
    let part2 = solve_part_two(&list);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/01.test").unwrap();
        let list = parse_input(&input).unwrap();
        let part1 = solve_part_one(&list);
        assert_eq!(part1, 11);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/01.test").unwrap();
        let list = parse_input(&input).unwrap();
        let part1 = solve_part_two(&list);
        assert_eq!(part1, 31);
    }
}
