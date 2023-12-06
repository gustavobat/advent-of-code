use anyhow::{Context, Result};
use std::str::FromStr;
use utils::parse_each_line;

#[derive(Debug)]
struct Round {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    rounds: Vec<Round>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self> {
        let parts: Vec<_> = input.split(':').collect();
        if parts.len() != 2 {
            anyhow::bail!("Invalid game format. Expected 'Game ID: data'");
        }

        let id = parts[0]
            .split_whitespace()
            .last()
            .and_then(|id_str| id_str.parse::<usize>().ok())
            .context("Invalid game ID")?;

        let rounds = parts[1]
            .split(';')
            .map(|round_str| {
                let mut green = 0;
                let mut red = 0;
                let mut blue = 0;

                round_str
                    .trim()
                    .split(',')
                    .map(|ball_count_str| {
                        let parts: Vec<_> = ball_count_str.split_whitespace().collect();
                        if parts.len() != 2 {
                            anyhow::bail!("Invalid ball count format");
                        }
                        let count = parts[0].parse::<usize>().context("Invalid count")?;
                        match parts[1] {
                            "red" => red = count,
                            "green" => green = count,
                            "blue" => blue = count,
                            _ => anyhow::bail!("Invalid color"),
                        }
                        Ok(())
                    })
                    .collect::<Result<Vec<_>>>()?;

                Ok(Round { red, green, blue })
            })
            .collect::<Result<Vec<_>>>()?;

        Ok(Game { id, rounds })
    }
}

fn solve_part1(input: &[Game]) -> usize {
    let reference_round = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .iter()
        .filter_map(|game| {
            for round in game.rounds.iter() {
                if round.red > reference_round.red
                    || round.green > reference_round.green
                    || round.blue > reference_round.blue
                {
                    return None;
                }
            }
            Some(game.id)
        })
        .sum()
}

fn solve_part2(input: &[Game]) -> usize {
    input
        .iter()
        .map(|game| {
            let mut max_red = 0;
            let mut max_green = 0;
            let mut max_blue = 0;
            for round in game.rounds.iter() {
                max_red = max_red.max(round.red);
                max_green = max_green.max(round.green);
                max_blue = max_blue.max(round.blue);
            }
            max_red * max_green * max_blue
        })
        .sum()
}

fn main() -> Result<()> {
    let input: Vec<Game> = parse_each_line("./data/02.input")?;

    let part1 = solve_part1(&input);
    println!("Part 1: {}", part1);

    let part2 = solve_part2(&input);
    println!("Part 2: {}", part2);
    Ok(())
}

#[cfg(test)]
mod day02 {
    use super::*;
    #[test]
    fn part1() -> Result<()> {
        let round_vec: Vec<Game> = parse_each_line("data/02.test")?;
        assert_eq!(solve_part1(&round_vec), 8);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let round_vec: Vec<Game> = parse_each_line("data/02.test")?;
        assert_eq!(solve_part2(&round_vec), 2286);
        Ok(())
    }
}
