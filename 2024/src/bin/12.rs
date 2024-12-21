use anyhow::Result;
use std::collections::HashSet;
use std::str::FromStr;
use utils::grid::Grid;

type Position = (usize, usize);

fn analyze_region(
    grid: &Grid<char>,
    current: Position,
    visited: &mut Vec<Position>,
    neighbors: &mut HashSet<Position>,
    n_internal_edges: &mut usize,
) {
    let (r, c) = current;
    neighbors.insert(current);
    visited.push(current);
    let current_char = grid.get(r, c).unwrap();
    let cur_neighbors = grid.get_cardinal_neighbors(r, c);
    for (nr, nc) in cur_neighbors {
        if grid.get(nr, nc) == Some(current_char) {
            *n_internal_edges += 1;
            if !visited.contains(&(nr, nc)) {
                analyze_region(grid, (nr, nc), visited, neighbors, n_internal_edges);
            }
        }
    }
}

fn count_corners(region: &[Position], grid: &Grid<char>) -> usize {
    let min_r = region.iter().map(|(r, _)| r).min().unwrap();
    let max_r = region.iter().map(|(r, _)| r).max().unwrap();
    let min_c = region.iter().map(|(_, c)| c).min().unwrap();
    let max_c = region.iter().map(|(_, c)| c).max().unwrap();
    let mut n_corners = 0;
    let current_char = grid.get(region[0])
}

fn solve_part_one(grid: &Grid<char>) -> usize {
    let mut visited = Vec::new();
    let mut regions = Vec::new();
    for pos in grid.iter() {
        if visited.contains(&pos) {
            continue;
        }
        let mut neighbors = HashSet::new();
        let mut n_internal_edges = 0;
        analyze_region(
            grid,
            pos,
            &mut visited,
            &mut neighbors,
            &mut n_internal_edges,
        );
        regions.push((pos, (neighbors, n_internal_edges)));
    }
    let mut total = 0;
    for (_, (neighbors, n_internal_edges)) in regions.iter() {
        let a = neighbors.len();
        let b = a * 4 - n_internal_edges;
        total += a * b;
    }
    total
}

fn solve_part_two(grid: &Grid<char>) -> usize {
    let mut visited = Vec::new();
    let mut regions = Vec::new();
    for pos in grid.iter() {
        if visited.contains(&pos) {
            continue;
        }
        let mut neighbors = HashSet::new();
        let mut n_internal_edges = 0;
        analyze_region(
            grid,
            pos,
            &mut visited,
            &mut neighbors,
            &mut n_internal_edges,
        );
        regions.push((pos, (neighbors, n_internal_edges)));
    }



    let mut total = 0;
    for (_, (neighbors, n_internal_edges)) in regions.iter() {
        let a = neighbors.len();
        let b = a * 4 - n_internal_edges;
        total += a * b;
    }
    total
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/12.input")?;
    let input = Grid::from_str(&input)?;

    let part1 = solve_part_one(&input);
    let part2 = solve_part_two(&input);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day12 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/12.test").unwrap();
        let input = Grid::from_str(&input).unwrap();
        let part1 = solve_part_one(&input);
        assert_eq!(part1, 36);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/12.test").unwrap();
        let input = Grid::from_str(&input).unwrap();
        let part2 = solve_part_two(&input);
        assert_eq!(part2, 81);
    }
}
