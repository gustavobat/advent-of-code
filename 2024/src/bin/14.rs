use anyhow::{anyhow, Result};
use std::cmp::Ordering;
use std::str::FromStr;

type Position = (i64, i64);
type Velocity = (i64, i64);
type GridSize = (usize, usize);

#[derive(Debug, Copy, Clone)]
struct Robot {
    pub pos: Position,
    pub velocity: Velocity,
}

impl FromStr for Robot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = regex::Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").map_err(|e| anyhow!(e))?;
        let caps = re.captures(s).ok_or(anyhow!("Invalid input format"))?;

        let pos = (
            caps.get(1)
                .ok_or(anyhow!("Could not find position x"))?
                .as_str()
                .parse::<i64>()?,
            caps.get(2)
                .ok_or(anyhow!("Could not find position y"))?
                .as_str()
                .parse::<i64>()?,
        );

        let velocity = (
            caps.get(3)
                .ok_or(anyhow!("Could not find velocity x"))?
                .as_str()
                .parse::<i64>()?,
            caps.get(4)
                .ok_or(anyhow!("Could not find velocity y"))?
                .as_str()
                .parse::<i64>()?,
        );
        Ok(Robot { pos, velocity })
    }
}

impl Robot {
    pub fn navigate(&self, grid_size: GridSize, seconds: i64) -> Position {
        let (x, y) = self.pos;
        let (v_x, v_y) = self.velocity;
        let (pos_x, pos_y) = (x + v_x * seconds, y + v_y * seconds);
        let (width, height) = grid_size;
        (
            pos_x.rem_euclid(width as i64),
            pos_y.rem_euclid(height as i64),
        )
    }
}

fn calc_safety_factor(positions: impl Iterator<Item = Position>, grid_size: GridSize) -> usize {
    let mut quadrant_counts = [0; 4];
    let (width, height) = grid_size;
    for (x, y) in positions {
        match (x.cmp(&(width as i64 / 2)), y.cmp(&(height as i64 / 2))) {
            (Ordering::Less, Ordering::Less) => {
                quadrant_counts[0] += 1;
            }
            (Ordering::Greater, Ordering::Less) => {
                quadrant_counts[1] += 1;
            }
            (Ordering::Less, Ordering::Greater) => {
                quadrant_counts[2] += 1;
            }
            (Ordering::Greater, Ordering::Greater) => {
                quadrant_counts[3] += 1;
            }
            _ => {}
        };
    }
    quadrant_counts.iter().product()
}

fn calc_xy_variance(positions: impl Iterator<Item = Position> + Clone) -> f64 {
    let (mut x_sum, mut y_sum) = (0, 0);
    let mut count = 0;
    for (x, y) in positions.clone() {
        x_sum += x;
        y_sum += y;
        count += 1;
    }
    let x_avg = x_sum as f64 / count as f64;
    let y_avg = y_sum as f64 / count as f64;

    positions.fold(0.0, |acc, (x, y)| {
        acc + (x as f64 - x_avg).powi(2) + (y as f64 - y_avg).powi(2)
    })
}

fn solve_part_one(robots: &[Robot]) -> usize {
    let grid_size = (101, 103);
    let iter = robots.iter().map(|robot| robot.navigate(grid_size, 100));
    calc_safety_factor(iter, grid_size)
}

fn solve_part_two(robots: &[Robot]) -> usize {
    let grid_size = (101, 103);
    let mut min_variance = f64::MAX;
    let mut min_i = 0;
    for i in 1..=10000 {
        let iter = robots.iter().map(|robot| robot.navigate(grid_size, i));
        let variance = calc_xy_variance(iter);
        if variance < min_variance {
            min_variance = variance;
            min_i = i;
        }
    }
    min_i as usize
}

fn parse_input(input: &str) -> Result<Vec<Robot>> {
    input.lines().map(|line| line.parse()).collect()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/14.input")?;
    let input = parse_input(&input)?;

    let part1 = solve_part_one(&input);
    let part2 = solve_part_two(&input);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day14 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/14.test").unwrap();
        let input = parse_input(&input).unwrap();
        let grid_size = (11, 7);
        let iter = input.iter().map(|robot| robot.navigate(grid_size, 100));
        let part1 = calc_safety_factor(iter, grid_size);
        assert_eq!(part1, 12);
    }
}
