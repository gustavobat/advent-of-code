use anyhow::Result;

fn solve(input: &[String], include_spelled: bool) -> Result<usize> {
    let mut patterns = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    if include_spelled {
        patterns.extend_from_slice(&[
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ]);
    }

    let nums = input
        .iter()
        .map(|s| {
            let indexes = patterns
                .iter()
                .enumerate()
                .map(|(i, substr)| {
                    let (mut first_match, mut last_match) = (None, None);
                    if let Some(index) = s.find(substr) {
                        first_match = Some((i % 10, index));
                    }
                    if let Some(index) = s.rfind(substr) {
                        last_match = Some((i % 10, index));
                    }
                    (first_match, last_match)
                })
                .collect::<Vec<_>>();
            let first = indexes
                .iter()
                .filter_map(|(first, _)| *first)
                .min_by_key(|(_, num)| *num)
                .unwrap()
                .0;
            let last = indexes
                .iter()
                .filter_map(|(_, last)| *last)
                .max_by_key(|(_, num)| *num)
                .unwrap()
                .0;
            10 * first + last
        })
        .sum();
    Ok(nums)
}

fn main() -> Result<()> {
    let input: Vec<String> = std::fs::read_to_string("./data/01.input")?
        .lines()
        .map(|s| s.to_string())
        .collect();

    println!("Part 1: {:?}", solve(&input, false)?);
    println!("Part 2: {:?}", solve(&input, true)?);

    Ok(())
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let input: Vec<String> = std::fs::read_to_string("./data/01.test1")?
            .lines()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(solve(&input, false)?, 142);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let input: Vec<String> = std::fs::read_to_string("./data/01.test2")?
            .lines()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(solve(&input, true)?, 281);

        Ok(())
    }
}
