use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};

pub type OrderingRules = HashMap<usize, HashSet<usize>>;

#[derive(Debug, Clone)]
pub struct Update(pub Vec<usize>);

impl Update {
    pub fn is_sorted(&self, ordering: &OrderingRules) -> bool {
        let Update(ref vec) = self;
        vec.windows(2).all(|pair| {
            let less = pair[0];
            let greater = pair[1];
            ordering
                .get(&less)
                .map_or(false, |set| set.contains(&greater))
        })
    }

    pub fn sort(&mut self, ordering_rules: &OrderingRules) {
        let Update(ref mut vec) = self;
        vec.sort_by(|a, b| {
            if ordering_rules.contains_key(a) {
                if ordering_rules[a].contains(b) {
                    std::cmp::Ordering::Less
                } else {
                    std::cmp::Ordering::Greater
                }
            } else {
                std::cmp::Ordering::Equal
            }
        });
    }

    pub fn middle_element(&self) -> usize {
        let Update(ref vec) = self;
        vec[vec.len() / 2]
    }
}

fn parse_input(input: &str) -> Result<(OrderingRules, Vec<Update>)> {
    let mut parts = input.split("\n\n");
    let orders_str = parts.next().ok_or(anyhow!("No order part"))?;
    let updates_str = parts.next().ok_or(anyhow!("No update part"))?;

    let rules = orders_str
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            let less = parts.next().ok_or(anyhow!("No less part"))?.parse()?;
            let greater = parts.next().ok_or(anyhow!("No greater part"))?.parse()?;
            Ok((less, greater))
        })
        .collect::<Result<Vec<_>>>()?;

    let mut page_ordering = OrderingRules::default();
    for (less, greater) in rules {
        page_ordering.entry(less).or_default().insert(greater);
    }

    let updates = updates_str
        .lines()
        .map(|line| {
            let update: Vec<usize> = line
                .split(',')
                .map(|part| part.parse().map_err(|e| anyhow!("Parse error: {}", e)))
                .collect::<Result<Vec<_>>>()?;
            Ok(Update(update))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok((page_ordering, updates))
}

fn solve_part_one(ordering: &OrderingRules, updates: &[Update]) -> usize {
    updates
        .iter()
        .filter_map(|update| {
            if update.is_sorted(ordering) {
                Some(update.middle_element())
            } else {
                None
            }
        })
        .sum()
}

fn solve_part_two(ordering: &OrderingRules, updates: &mut [Update]) -> usize {
    updates
        .iter_mut()
        .filter_map(|update| {
            if !update.is_sorted(ordering) {
                update.sort(ordering);
                Some(update.middle_element())
            } else {
                None
            }
        })
        .sum()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/05.input")?;
    let (ordering, mut updates) = parse_input(&input)?;
    let part1 = solve_part_one(&ordering, &updates);
    let part2 = solve_part_two(&ordering, &mut updates);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/05.test").unwrap();
        let (ordering, updates) = parse_input(&input).unwrap();
        let part1 = solve_part_one(&ordering, &updates);
        assert_eq!(part1, 143);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/05.test").unwrap();
        let (ordering, mut updates) = parse_input(&input).unwrap();
        let part2 = solve_part_two(&ordering, &mut updates);
        assert_eq!(part2, 123);
    }
}
