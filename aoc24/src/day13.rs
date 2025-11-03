use anyhow::Result;
use anyhow::anyhow;
use std::str::FromStr;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 13, solve_all)
}

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
    pub fn try_solve(&self) -> Option<i64> {
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
    games.iter().filter_map(|game| game.try_solve()).sum()
}

fn solve_part_two(games: &mut [Game]) -> i64 {
    games.iter_mut().for_each(|game| {
        let (x, y) = game.prize;
        game.prize = (x + 10000000000000, y + 10000000000000);
    });
    games.iter().filter_map(|game| game.try_solve()).sum()
}

fn parse_input(input: &str) -> Result<Vec<Game>> {
    input.split("\n\n").map(|s| s.parse()).collect()
}

fn solve_all(input: &str) -> Result<Solution> {
    let mut games = parse_input(input)?;
    let part_one = solve_part_one(&games).to_string();
    let part_two = solve_part_two(&mut games).to_string();

    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::solve_part_one;
    use utils::load_test_input;

    #[test]
    fn both_parts() {
        let input = load_test_input!();
        let games = parse_input(&input).unwrap();
        let solution = solve_part_one(&games);
        assert_eq!(solution, 480);
    }
}
