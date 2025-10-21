use itertools::Itertools;
use std::str::FromStr;
use utils::grid::Direction;
use utils::grid::Grid;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 4, solve_all)
}

fn solve_part_one(grid: &Grid<char>) -> usize {
    grid.iter_coords()
        .filter(|coord| grid.get(*coord) == Some(&'X'))
        .map(|coord| {
            Direction::all()
                .iter()
                .filter(|&&dir| {
                    let yielded = grid
                        .iter_from_start_and_direction(coord, dir)
                        .copied()
                        .take(4)
                        .collect_array::<4>();
                    yielded.is_some_and(|arr| arr == ['X', 'M', 'A', 'S'])
                })
                .count()
        })
        .sum::<usize>()
}

fn solve_part_two(grid: &Grid<char>) -> usize {
    let is_mas = |c1, c2| (c1, c2) == ('M', 'S') || (c1, c2) == ('S', 'M');
    grid.iter_coords()
        .filter(|(row, col)| {
            if *row < 1 || *col < 1 {
                return false;
            }
            if let [Some(&'A'), Some(&c11), Some(&c12), Some(&c21), Some(&c22)] = [
                grid.get((*row, *col)),
                grid.get((row - 1, col - 1)),
                grid.get((row + 1, col + 1)),
                grid.get((row - 1, col + 1)),
                grid.get((row + 1, col - 1)),
            ] {
                return is_mas(c11, c12) && is_mas(c21, c22);
            }
            false
        })
        .count()
}

fn parse_input(input: &str) -> anyhow::Result<Grid<char>> {
    Grid::from_str(input)
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let input = parse_input(input)?;
    let part_one = solve_part_one(&input).to_string();
    let part_two = solve_part_two(&input).to_string();

    Ok(Solution { part_one, part_two })
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
        let input = parse_input(&input).unwrap();
        let solution = solve_part_one(&input);
        assert_eq!(solution, 18);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let input = parse_input(&input).unwrap();
        let solution = solve_part_two(&input);
        assert_eq!(solution, 9);
    }
}
