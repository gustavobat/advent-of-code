use utils::grid::Direction;
use utils::grid::Grid;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 4, solve_all)
}

fn is_accessible_roll(grid: &Grid<char>, coord: (usize, usize)) -> bool {
    grid.get(coord) == Some(&'@')
        && grid
            .get_neighbors(coord, &Direction::all())
            .filter(|neigh| grid.get(*neigh) == Some(&'@'))
            .count()
            < 4
}

fn solve_part_one(grid: &Grid<char>) -> usize {
    grid.iter_flat_indices()
        .filter(|coord| is_accessible_roll(grid, *coord))
        .count()
}

fn solve_part_two(grid: &mut Grid<char>) -> usize {
    let mut total = 0;
    loop {
        let accessible_rolls = grid
            .iter_flat_indices()
            .filter(|coord| is_accessible_roll(grid, *coord))
            .collect::<Vec<_>>();

        if accessible_rolls.is_empty() {
            break;
        }

        total += accessible_rolls.len();
        accessible_rolls.iter().for_each(|roll| {
            grid.set(*roll, '.')
                .expect("Roll coord should be part of the grid")
        });
    }
    total
}

fn parse_input(input: &str) -> anyhow::Result<Grid<char>> {
    Grid::from_char_grid_str(input)
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let mut input = parse_input(input)?;
    let part_one = solve_part_one(&input).to_string();
    let part_two = solve_part_two(&mut input).to_string();

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
        assert_eq!(solution, 13);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let mut input = parse_input(&input).unwrap();
        let solution = solve_part_two(&mut input);
        assert_eq!(solution, 43);
    }
}
