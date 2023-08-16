use anyhow::Result;

#[derive(Debug)]
struct Assignment {
    start: u8,
    end: u8,
}

impl std::str::FromStr for Assignment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((start_str, end_str)) = s.split_once('-') {
            let start: u8 = start_str.parse::<u8>()?;
            let end: u8 = end_str.parse::<u8>()?;
            Ok(Assignment { start, end })
        } else {
            Err(anyhow::format_err!("Error spliting sections"))
        }
    }
}

impl Assignment {
    fn fully_contains(&self, other: &Assignment) -> bool {
        other.start >= self.start && other.end <= self.end
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        other.start >= self.start && other.start <= self.end
            || other.end >= self.start && other.end <= self.end
    }
}

#[derive(Debug)]
struct AssignmentPair {
    first: Assignment,
    second: Assignment,
}

impl std::str::FromStr for AssignmentPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((first_str, second_str)) = s.split_once(',') {
            let first = first_str.parse::<Assignment>()?;
            let second = second_str.parse::<Assignment>()?;
            Ok(AssignmentPair { first, second })
        } else {
            Err(anyhow::format_err!("Error spliting sections"))
        }
    }
}

impl AssignmentPair {
    pub fn fully_contains(&self) -> bool {
        self.first.fully_contains(&self.second) || self.second.fully_contains(&self.first)
    }

    pub fn overlaps(&self) -> bool {
        self.first.overlaps(&self.second) || self.second.overlaps(&self.first)
    }
}

fn count_fully_contains(assignment_pairs: &[AssignmentPair]) -> usize {
    assignment_pairs
        .iter()
        .filter(|pair| pair.fully_contains())
        .count()
}

fn count_overlaps(assignment_pairs: &[AssignmentPair]) -> usize {
    assignment_pairs
        .iter()
        .filter(|pair| pair.overlaps())
        .count()
}

fn main() -> Result<()> {
    let input: Vec<AssignmentPair> = utils::parse_each_line("data/04.input")?;

    println!("Part 1: {}", count_fully_contains(&input));
    println!("Part 2: {}", count_overlaps(&input));

    Ok(())
}

#[cfg(test)]
mod day04 {
    use super::*;
    #[test]
    fn part1() -> Result<()> {
        let input: Vec<AssignmentPair> = utils::parse_each_line("data/04.test")?;
        assert_eq!(count_fully_contains(&input), 2);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let input: Vec<AssignmentPair> = utils::parse_each_line("data/04.test")?;
        assert_eq!(count_overlaps(&input), 4);
        Ok(())
    }
}
