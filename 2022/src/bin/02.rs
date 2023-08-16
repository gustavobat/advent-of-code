use anyhow::Result;
use std::str::FromStr;
use utils::parse_each_line;

#[derive(Debug, Copy, Clone)]
enum OpponentPlay {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for OpponentPlay {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "A" => Ok(OpponentPlay::Rock),
            "B" => Ok(OpponentPlay::Paper),
            "C" => Ok(OpponentPlay::Scissors),
            _ => Err(anyhow::format_err!("Could not parse OpponentPlay")),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum SecondColumn {
    X = 0,
    Y = 1,
    Z = 2,
}

impl FromStr for SecondColumn {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "X" => Ok(SecondColumn::X),
            "Y" => Ok(SecondColumn::Y),
            "Z" => Ok(SecondColumn::Z),
            _ => Err(anyhow::format_err!("Could not parse SecondColumn")),
        }
    }
}

impl SecondColumn {
    fn to_shape_points(self) -> u32 {
        self as u32 + 1
    }

    fn to_outcome_points(self) -> u32 {
        self as u32 * 3
    }
}

#[derive(Debug)]
struct Round {
    opponent_play: OpponentPlay,
    second_column: SecondColumn,
}

impl FromStr for Round {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        if let Some((opponent_play_str, second_column_str)) = s.split_once(' ') {
            Ok(Round::new(
                OpponentPlay::from_str(opponent_play_str).unwrap(),
                SecondColumn::from_str(second_column_str).unwrap(),
            ))
        } else {
            Err(anyhow::format_err!("Could not parse plays"))
        }
    }
}

impl Round {
    fn new(opponent_play: OpponentPlay, second_column: SecondColumn) -> Self {
        Self {
            opponent_play,
            second_column,
        }
    }

    fn outcome_score(&self) -> u32 {
        (self.second_column as u32 + 5 - self.opponent_play as u32) % 3 * 3
    }

    fn shape_score(&self) -> u32 {
        (self.opponent_play as u32 + self.second_column as u32 + 1) % 3 + 1
    }

    pub fn total_round_score_1(&self) -> u32 {
        self.second_column.to_shape_points() + self.outcome_score()
    }

    pub fn total_round_score_2(&self) -> u32 {
        self.second_column.to_outcome_points() + self.shape_score()
    }
}

fn main() -> Result<()> {
    let round_vec: Vec<Round> = parse_each_line("data/02.input")?;

    let score1 = round_vec
        .iter()
        .fold(0, |acc, round| acc + round.total_round_score_1());

    let score2 = round_vec
        .iter()
        .fold(0, |acc, round| acc + round.total_round_score_2());

    println!("Part 1: {}", score1);
    println!("Part 2: {}", score2);

    Ok(())
}

#[cfg(test)]
mod day02 {
    use super::*;
    #[test]
    fn part1() -> Result<()> {
        let round_vec: Vec<Round> = parse_each_line("data/02.test")?;
        let score = round_vec
            .iter()
            .fold(0, |acc, round| acc + round.total_round_score_1());
        assert_eq!(score, 15);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let round_vec: Vec<Round> = parse_each_line("data/02.test")?;
        let score = round_vec
            .iter()
            .fold(0, |acc, round| acc + round.total_round_score_2());
        assert_eq!(score, 12);
        Ok(())
    }
}
