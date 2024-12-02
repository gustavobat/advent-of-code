use anyhow::{anyhow, Result};
use std::cmp::min;
use std::collections::HashSet;
use std::str::FromStr;
use utils::parse_each_line;

struct Card {
    winning_numbers: HashSet<u32>,
    elf_numbers: HashSet<u32>,
}

impl Card {
    fn count_winning_numbers(&self) -> usize {
        self.winning_numbers.intersection(&self.elf_numbers).count()
    }

    fn calc_score(&self) -> u32 {
        (0..self.count_winning_numbers()).fold(1, |score, _| score * 2) >> 1
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('|').collect();
        if parts.len() != 2 {
            return Err(anyhow!("Invalid input format"));
        }

        let winning_numbers_str = parts[0]
            .split(':')
            .nth(1)
            .ok_or_else(|| anyhow!("Missing winning numbers"))?;

        let winning_numbers = winning_numbers_str
            .split_whitespace()
            .map(|num| num.parse::<u32>().map_err(|e| anyhow!(e)))
            .collect::<Result<HashSet<_>>>()?;

        let elf_numbers = parts[1]
            .split_whitespace()
            .map(|num| num.parse::<u32>().map_err(|e| anyhow!(e)))
            .collect::<Result<HashSet<_>>>()?;

        Ok(Card {
            winning_numbers,
            elf_numbers,
        })
    }
}

fn total_scratch_cards(cards: &[Card]) -> usize {
    let mut copies = vec![1; cards.len()];
    cards.iter().enumerate().for_each(|(i, card)| {
        let winning_nums = card.count_winning_numbers();
        let lower_bound = min(cards.len(), i + 1);
        let upper_bound = min(cards.len(), i + 1 + winning_nums);
        for j in lower_bound..upper_bound {
            copies[j] += copies[i];
        }
    });
    copies.iter().sum()
}

fn main() -> Result<()> {
    let input: Vec<Card> = parse_each_line("./data/04.input")?;
    let part1: u32 = input.iter().map(|card| card.calc_score()).sum();
    println!("Part 1: {}", part1);

    let part2 = total_scratch_cards(&input);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod day04 {
    use super::*;
    #[test]
    fn part1() -> Result<()> {
        let input: Vec<Card> = parse_each_line("./data/04.test")?;
        let part1: u32 = input.iter().map(|card| card.calc_score()).sum();
        assert_eq!(part1, 13);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let input: Vec<Card> = parse_each_line("./data/04.test")?;
        assert_eq!(total_scratch_cards(&input), 30);
        Ok(())
    }
}
