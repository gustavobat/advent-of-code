use utils::solution::Solution;
use utils::solution::Solver;

use anyhow::Result;
use anyhow::anyhow;
use hashbrown::HashSet;
use std::cmp::Ordering;
use std::str::FromStr;

inventory::submit! {
    Solver::new(2024, 14, solve_all)
}

type Position = (i64, i64);
type Velocity = (i64, i64);

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
        let (_, [x, y, v_x, v_y]) = caps.extract();
        let pos = (x.parse::<i64>()?, y.parse::<i64>()?);
        let velocity = (v_x.parse::<i64>()?, v_y.parse::<i64>()?);
        Ok(Robot { pos, velocity })
    }
}

impl Robot {
    pub fn navigate(&self, grid_size: (usize, usize), seconds: i64) -> Position {
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

fn calc_safety_factor(
    positions: impl Iterator<Item = Position>,
    grid_size: (usize, usize),
) -> usize {
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

    positions
        .map(|(x, y)| (x as f64 - x_avg).powi(2) + (y as f64 - y_avg).powi(2))
        .sum()
}

fn solve_part_one(robots: &[Robot], grid_size: (usize, usize)) -> usize {
    let iter = robots.iter().map(|robot| robot.navigate(grid_size, 100));
    calc_safety_factor(iter, grid_size)
}

fn solve_part_two(robots: &[Robot], grid_size: (usize, usize)) -> (usize, String) {
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

    let tree_positions = robots
        .iter()
        .map(|robot| robot.navigate(grid_size, min_i))
        .collect::<HashSet<_>>();

    let (x, y) = grid_size;
    let mut figure = String::with_capacity(x * y);
    for y in 0..y {
        for x in 0..x {
            if tree_positions.contains(&(x as i64, y as i64)) {
                figure.push('#');
            } else {
                figure.push('.');
            }
        }
        figure.push('\n');
    }
    (min_i as usize, figure)
}

fn parse_input(input: &str) -> Result<Vec<Robot>> {
    input.lines().map(|line| line.parse()).collect()
}

fn solve_all(input: &str) -> Result<Solution> {
    let robots = parse_input(input)?;
    let grid_size = (101, 103);
    let part_one = solve_part_one(&robots, grid_size).to_string();

    let (min_i, figure) = solve_part_two(&robots, grid_size);
    let part_two = format!("{}\n{}", min_i, figure);

    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::solve_part_one;
    use utils::load_test_input;

    #[test]
    fn part_one() {
        let input = load_test_input!();
        let grid_size = (11, 7);
        let robots = parse_input(&input).unwrap();
        let solution = solve_part_one(&robots, grid_size);
        assert_eq!(solution, 12);
    }
}
