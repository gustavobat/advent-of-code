use anyhow::anyhow;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 1, solve_all)
}

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Left(i32),
    Right(i32),
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let list = parse_input(input)?;
    let part_one = solve_part_one(&list).to_string();
    let part_two = solve_part_two(&list).to_string();
    Ok(Solution { part_one, part_two })
}

fn solve_part_one(rotation: &[Rotation]) -> usize {
    let mut dial: i32 = 50;
    let mut zero_count: usize = 0;

    for rot in rotation {
        let delta = match rot {
            Rotation::Left(d) => -d,
            Rotation::Right(d) => *d,
        };
        dial = (dial + delta).rem_euclid(100);
        if dial == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn solve_part_two(rotation: &[Rotation]) -> usize {
    let mut dial: i32 = 50;
    let mut zero_count: usize = 0;

    for rot in rotation {
        let (delta, total) = match rot {
            Rotation::Left(total) => (-1, *total),
            Rotation::Right(total) => (1, *total),
        };
        for _ in 0..total {
            dial = (dial + delta).rem_euclid(100);
            if dial == 0 {
                zero_count += 1;
            }
        }
    }

    zero_count
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Rotation>> {
    input
        .lines()
        .map(|line| {
            let (direction, value) = line.split_at(1);
            let direction = direction.parse::<char>()?;
            let value = value.parse::<i32>()?;
            match direction {
                'L' => Ok(Rotation::Left(value)),
                'R' => Ok(Rotation::Right(value)),
                _ => Err(anyhow!("Invalid direction: {}", direction)),
            }
        })
        .collect::<anyhow::Result<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::solve_part_one;
    use super::solve_part_two;
    use utils::load_test_input;

    #[test]
    fn part_one() {
        let input = load_test_input!();
        let rotations = parse_input(&input).unwrap();
        let solution = solve_part_one(&rotations);
        assert_eq!(solution, 3);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let rotations = parse_input(&input).unwrap();
        let solution = solve_part_two(&rotations);
        assert_eq!(solution, 6);
    }
}
