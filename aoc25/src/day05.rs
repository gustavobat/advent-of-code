use anyhow::anyhow;
use std::collections::BTreeSet;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 5, solve_all)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FreshRanges {
    pub start: usize,
    pub end: usize,
}

impl FreshRanges {
    pub fn new(start: usize, end: usize) -> anyhow::Result<Self> {
        if start <= end {
            Ok(Self { start, end })
        } else {
            Err(anyhow!("Invalid range: start ({}) > end ({})", start, end))
        }
    }

    pub fn contains(&self, ingredient_id: usize) -> bool {
        self.start <= ingredient_id && ingredient_id <= self.end
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && other.start <= self.end
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start + 1
    }
}

impl Ord for FreshRanges {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start).then(self.end.cmp(&other.end))
    }
}

impl PartialOrd for FreshRanges {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn merge_overlapping_ranges<'a>(
    original_ranges: impl Iterator<Item = &'a FreshRanges>,
) -> Vec<FreshRanges> {
    let mut new_ranges = Vec::new();
    for range in original_ranges {
        if new_ranges.is_empty() {
            new_ranges.push(*range);
            continue;
        }

        let last_index = new_ranges.len() - 1;
        let last = new_ranges[last_index];
        if range.overlaps(&last) {
            let merged = range.merge(&last);
            new_ranges[last_index] = merged;
        } else {
            new_ranges.push(*range);
        }
    }
    new_ranges
}

fn solve_part_one(fresh_ranges: &[FreshRanges], ids: &[u64]) -> u64 {
    // This is quick enough, so a binary search isn't necessary
    let mut count = 0;
    for id in ids {
        for range in fresh_ranges {
            if range.contains(*id as usize) {
                count += 1;
                break;
            }
        }
    }
    count
}

fn solve_part_two(ranges: &[FreshRanges]) -> u64 {
    ranges.iter().map(FreshRanges::len).sum::<usize>() as u64
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<FreshRanges>, Vec<u64>)> {
    let (fresh_ranges, ingredient_ids) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("Invalid input"))?;

    let fresh_ranges = fresh_ranges
        .lines()
        .map(|line| {
            let (start_str, end_str) = line
                .split_once('-')
                .ok_or_else(|| anyhow!("Invalid range line: {}", line))?;
            let start: usize = start_str.parse()?;
            let end: usize = end_str.parse()?;
            FreshRanges::new(start, end)
        })
        .collect::<Result<BTreeSet<_>, _>>()?;
    let fresh_ranges = merge_overlapping_ranges(fresh_ranges.iter());

    let ingredient_ids = ingredient_ids
        .lines()
        .map(|line| line.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    Ok((fresh_ranges, ingredient_ids))
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let (ranges, ingredients) = parse_input(input).unwrap();

    let part_one = solve_part_one(&ranges, &ingredients).to_string();
    let part_two = solve_part_two(&ranges).to_string();
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
        let (ranges, ingredients) = parse_input(&input).unwrap();
        let solution = solve_part_one(&ranges, &ingredients);
        assert_eq!(solution, 3);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let (ranges, _) = parse_input(&input).unwrap();
        let solution = solve_part_two(&ranges);
        assert_eq!(solution, 14);
    }
}
