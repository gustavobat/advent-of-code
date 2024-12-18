use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use utils::grid::Grid;
use utils::grid::Position;

fn dfs(
    grid: &Grid<u8>,
    visited: &mut Vec<Position>,
    results: &mut HashMap<Position, Vec<Position>>,
) -> bool {
    if let Some(&pos) = visited.last() {
        if let Some(&current_val) = grid.get(pos) {
            if current_val == 9 {
                results.entry(visited[0]).or_default().push(pos);
                visited.pop();
                return true;
            }
            for (_, neighbor) in
                grid.get_cardinal_neighbors(pos)
                    .into_iter()
                    .filter(|(_, neigh)| {
                        grid.get(*neigh)
                            .map_or(false, |&val| val == current_val + 1)
                    })
            {
                visited.push(neighbor);
                if !dfs(grid, visited, results) {
                    visited.pop();
                }
            }
        }
    }
    false
}

fn process_grid<F>(grid: &Grid<u8>, result_mapper: F) -> usize
where
    F: Fn(Vec<Position>) -> usize,
{
    let mut results = HashMap::new();
    let possible_starts = grid.iter().filter(|pos| grid.get(*pos) == Some(&0));
    possible_starts.for_each(|(r, c)| {
        let mut visited = vec![(r, c)];
        dfs(grid, &mut visited, &mut results);
    });
    results.into_values().map(result_mapper).sum()
}

fn solve_part_one(grid: &Grid<u8>) -> usize {
    process_grid(grid, |v| HashSet::<Position>::from_iter(v).len())
}

fn solve_part_two(grid: &Grid<u8>) -> usize {
    process_grid(grid, |v| v.len())
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/10.input")?;
    let input = Grid::from_str(&input)?;

    let part1 = solve_part_one(&input);
    let part2 = solve_part_two(&input);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day10 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/10.test").unwrap();
        let input = Grid::from_str(&input).unwrap();
        let part1 = solve_part_one(&input);
        assert_eq!(part1, 36);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/10.test").unwrap();
        let input = Grid::from_str(&input).unwrap();
        let part2 = solve_part_two(&input);
        assert_eq!(part2, 81);
    }
}
