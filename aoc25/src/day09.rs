use anyhow::anyhow;
use rayon::prelude::*;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 9, solve_all)
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Rectangle {
    min_x: usize,
    min_y: usize,
    max_x: usize,
    max_y: usize,
}

impl Rectangle {
    fn new(a: (usize, usize), b: (usize, usize)) -> Self {
        let (ax, ay) = a;
        let (bx, by) = b;
        Rectangle {
            min_x: ax.min(bx),
            min_y: ay.min(by),
            max_x: ax.max(bx),
            max_y: ay.max(by),
        }
    }

    fn area(&self) -> usize {
        let width = self.max_y.abs_diff(self.min_y) + 1;
        let height = self.max_x.abs_diff(self.min_x) + 1;
        width * height
    }

    fn intersects(&self, other: &Rectangle) -> bool {
        self.min_x < other.max_x
            && self.max_x > other.min_x
            && self.min_y < other.max_y
            && self.max_y > other.min_y
    }
}

fn calc_all_rectangles(points: &[(usize, usize)]) -> Vec<Rectangle> {
    let n_points = points.len();
    let mut rectangles = Vec::with_capacity(n_points * (n_points - 1) / 2);
    for i in 0..points.len() {
        for j in i + 1..points.len() {
            rectangles.push(Rectangle::new(points[i], points[j]));
        }
    }
    rectangles
}

fn solve_part_one(rectangles: &[Rectangle]) -> anyhow::Result<usize> {
    let max_area = rectangles
        .par_iter()
        .map(|r| r.area())
        .max()
        .ok_or(anyhow!("No rectangles found"))?;
    Ok(max_area)
}

fn solve_part_two(points: &[(usize, usize)], rectangles: &[Rectangle]) -> anyhow::Result<usize> {
    let n_points = points.len();
    let edges = (0..n_points)
        .map(|i| Rectangle::new(points[i], points[(i + 1) % n_points]))
        .collect::<Vec<_>>();

    let max = rectangles
        .par_iter()
        .filter_map(|r| {
            if edges.iter().any(|e| r.intersects(e)) {
                None
            } else {
                Some(r.area())
            }
        })
        .max()
        .ok_or(anyhow!("No valid rectangles found"))?;
    Ok(max)
}

fn parse_input(input: &str) -> anyhow::Result<Vec<(usize, usize)>> {
    let mut points = Vec::new();
    for line in input.lines() {
        let mut parts = line.split(',');
        let x_str = parts.next().ok_or(anyhow!("Missing x coordinate"))?;
        let y_str = parts.next().ok_or(anyhow!("Missing y coordinate"))?;
        let x = x_str.parse::<usize>()?;
        let y = y_str.parse::<usize>()?;
        points.push((x, y));
    }
    Ok(points)
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let points = parse_input(input)?;
    let rectangles = calc_all_rectangles(&points);
    let part_one = solve_part_one(&rectangles)?.to_string();
    let part_two = solve_part_two(&points, &rectangles)?.to_string();

    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    use super::calc_all_rectangles;
    use super::parse_input;
    use super::solve_part_one;
    use super::solve_part_two;
    use utils::load_test_input;

    #[test]
    fn part_one() {
        let input = load_test_input!();
        let points = parse_input(&input).unwrap();
        let rectangles = calc_all_rectangles(&points);
        let solution = solve_part_one(&rectangles).unwrap();
        assert_eq!(solution, 50);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let points = parse_input(&input).unwrap();
        let rectangles = calc_all_rectangles(&points);
        let solution = solve_part_two(&points, &rectangles).unwrap();
        assert_eq!(solution, 24);
    }
}
