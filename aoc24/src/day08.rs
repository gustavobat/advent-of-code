use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use utils::grid::Grid;
use utils::grid::GridVector;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 8, solve_all)
}

fn find_antennas(grid: &Grid<char>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas = HashMap::new();
    for coord in grid.iter_flat_indices() {
        if let Some(&c) = grid.get(coord)
            && c.is_ascii_alphanumeric()
        {
            antennas.entry(c).or_insert_with(Vec::new).push(coord);
        }
    }
    antennas
}

fn iter_antinodes<'g>(
    start: GridVector,
    delta: GridVector,
    city: &'g Grid<char>,
) -> impl Iterator<Item = GridVector> + 'g {
    let mut next = start;
    std::iter::from_fn(move || {
        if next.try_into().is_ok_and(|idx| city.contains(idx)) {
            let res = next;
            next += delta;
            Some(res)
        } else {
            None
        }
    })
}

fn calculate_antinodes(
    coords: &[(usize, usize)],
    city: &Grid<char>,
    resonate: bool,
) -> HashSet<GridVector> {
    let mut antinodes = HashSet::new();
    let pairs = coords.iter().combinations(2);
    for pair in pairs {
        let a = (*pair[0]).into();
        let b = (*pair[1]).into();
        let mut a_iter = iter_antinodes(a, a - b, city);
        let mut b_iter = iter_antinodes(b, b - a, city);
        if resonate {
            antinodes.extend(a_iter.chain(b_iter));
        } else {
            antinodes.extend(a_iter.nth(1));
            antinodes.extend(b_iter.nth(1));
        }
    }
    antinodes
}

fn solve_part_one(city: &Grid<char>) -> usize {
    find_antennas(city)
        .values()
        .flat_map(|coords| calculate_antinodes(coords, city, false))
        .collect::<HashSet<_>>()
        .len()
}

fn solve_part_two(city: &Grid<char>) -> usize {
    find_antennas(city)
        .values()
        .flat_map(|coords| calculate_antinodes(coords, city, true))
        .collect::<HashSet<_>>()
        .len()
}

fn parse_input(input: &str) -> Result<Grid<char>> {
    Grid::from_char_grid_str(input)
}

fn solve_all(input: &str) -> Result<Solution> {
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
        let city = parse_input(&input).unwrap();
        let solution = solve_part_one(&city);
        assert_eq!(solution, 14);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let city = parse_input(&input).unwrap();
        let solution = solve_part_two(&city);
        assert_eq!(solution, 34)
    }
}
