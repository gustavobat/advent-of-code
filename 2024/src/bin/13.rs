use anyhow::{anyhow, Result};
use std::str::FromStr;

type Point = (i64, i64);

#[derive(Debug, Copy, Clone)]
struct Game {
    pub button_a: Point,
    pub button_b: Point,
    pub prize: Point,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = regex::Regex::new(r"X[+=](\d+), Y[+=](\d+)").map_err(|e| anyhow!(e))?;
        let points = re
            .captures_iter(s)
            .map(|cap| {
                let x = cap
                    .get(1)
                    .ok_or(anyhow!("Could not find X value"))?
                    .as_str()
                    .parse::<i64>()?;
                let y = cap
                    .get(2)
                    .ok_or(anyhow!("Could not find Y value"))?
                    .as_str()
                    .parse::<i64>()?;
                Ok((x, y))
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Game {
            button_a: points[0],
            button_b: points[1],
            prize: points[2],
        })
    }
}

impl Game {
    pub fn solve(&self) -> Option<i64> {
        let (x1, y1) = self.button_a;
        let (x2, y2) = self.button_b;
        let (x3, y3) = self.prize;
        let a = (y3 * x2 - y2 * x3) / (y1 * x2 - y2 * x1);
        let b = (y3 - a * y1) / y2;
        if a * x1 + b * x2 == x3 && a * y1 + b * y2 == y3 {
            Some(3 * a + b)
        } else {
            None
        }
    }
}

fn solve_part_one(games: &[Game]) -> i64 {
    games.iter().filter_map(|game| game.solve()).sum()
}

fn solve_part_two(games: &mut [Game]) -> i64 {
    games.iter_mut().for_each(|game| {
        let (x, y) = game.prize;
        game.prize = (x + 10000000000000, y + 10000000000000);
    });
    games.iter().filter_map(|game| game.solve()).sum()
}

fn parse_input(input: &str) -> Result<Vec<Game>> {
    input.split("\n\n").map(|s| s.parse()).collect()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/13.input")?;
    let mut input = parse_input(&input)?;

    let part1 = solve_part_one(&input);
    let part2 = solve_part_two(&mut input);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day13 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/13.test").unwrap();
        let input = parse_input(&input).unwrap();
        let part1 = solve_part_one(&input);
        assert_eq!(part1, 480);
    }
}
