use anyhow::anyhow;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 12, solve_all)
}

#[derive(Debug, Clone, PartialEq)]
struct Shape {
    pattern: Vec<Vec<bool>>,
}

impl Shape {
    fn area(&self) -> usize {
        self.pattern
            .iter()
            .map(|row| row.iter().filter(|&&cell| cell).count())
            .sum()
    }
}

#[derive(Debug, Clone)]
struct Region {
    width: usize,
    height: usize,
    required_shapes: Vec<usize>,
}

impl Region {
    fn area(&self) -> usize {
        self.width * self.height
    }
}

fn parse_input(input: &str) -> anyhow::Result<(Vec<Shape>, Vec<Region>)> {
    let mut shapes = Vec::new();
    let mut regions = Vec::new();
    let mut lines = input.lines().peekable();

    while lines.peek().is_some() {
        while lines.peek().is_some_and(|l| l.trim().is_empty()) {
            lines.next();
        }

        if lines.peek().is_none() {
            break;
        }

        let first_line = lines.next().unwrap();
        if first_line.contains('x') && first_line.contains(':') {
            let parts: Vec<&str> = first_line.split(':').collect();
            if parts.len() != 2 {
                return Err(anyhow!("Invalid region format"));
            }

            let dimensions = parts[0].trim();
            let dim_parts: Vec<&str> = dimensions.split('x').collect();
            if dim_parts.len() != 2 {
                return Err(anyhow!("Invalid dimensions format"));
            }

            let width: usize = dim_parts[0].parse()?;
            let height: usize = dim_parts[1].parse()?;

            let required_shapes: Vec<usize> = parts[1]
                .split_whitespace()
                .map(|n| n.parse().map_err(|e| anyhow!("Parse error: {}", e)))
                .collect::<Result<_, _>>()?;

            regions.push(Region {
                width,
                height,
                required_shapes,
            });
            continue;
        }

        if !first_line.ends_with(':') {
            return Err(anyhow!("Expected shape index ending with ':'"));
        }

        let mut pattern = Vec::new();
        for _ in 0..3 {
            if let Some(line) = lines.next() {
                let row: Vec<bool> = line.trim().chars().map(|c| c == '#').collect();
                pattern.push(row);
            } else {
                return Err(anyhow!("Incomplete shape grid"));
            }
        }

        shapes.push(Shape { pattern });
    }

    Ok((shapes, regions))
}

fn trivially_valid(region: &Region) -> bool {
    let n_required_shapes = region.required_shapes.iter().sum();
    region.width / 3 * region.height / 3 >= n_required_shapes
}

fn trivially_invalid(shapes: &[Shape], region: &Region) -> bool {
    let total_shape_area: usize = region
        .required_shapes
        .iter()
        .enumerate()
        .map(|(i, &count)| count * shapes[i].area())
        .sum();
    total_shape_area > region.area()
}

fn solve_part_one(shapes: &[Shape], regions: &[Region]) -> usize {
    regions.iter().fold(0, |valid_count, region| {
        if trivially_valid(region) {
            valid_count + 1
        } else {
            debug_assert!(trivially_invalid(shapes, region));
            valid_count
        }
    })
}

fn solve_part_two(shapes: &[Shape], regions: &[Region]) -> usize {
    shapes.len() + regions.len()
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let (shapes, regions) = parse_input(input)?;
    let part_one = solve_part_one(&shapes, &regions).to_string();
    let part_two = solve_part_two(&shapes, &regions).to_string();

    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_one() {
        // The solution is only viable for the actual input.
    }

    #[test]
    fn part_two() {
        // The solution is only viable for the actual input.
    }
}
