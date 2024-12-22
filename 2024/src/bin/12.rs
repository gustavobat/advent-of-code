use anyhow::Result;
use std::collections::HashSet;
use std::str::FromStr;
use utils::grid::Grid;

type Position = (usize, usize);

fn traverse_plot(
    grid: &Grid<char>,
    current: Position,
    visited: &mut Vec<Position>,
    neighbors: &mut HashSet<Position>,
    n_internal_edges: &mut usize,
) {
    neighbors.insert(current);
    visited.push(current);
    let current_char = grid.get(current).unwrap();
    let cur_neighbors = grid.get_cardinal_neighbors(current);
    for (_, neigh) in cur_neighbors {
        if grid.get(neigh) == Some(current_char) {
            *n_internal_edges += 1;
            if !visited.contains(&neigh) {
                traverse_plot(grid, neigh, visited, neighbors, n_internal_edges);
            }
        }
    }
}

fn find_garden_plots(grid: &Grid<char>) -> Vec<(HashSet<Position>, usize)> {
    let mut visited = Vec::new();
    let mut plots = Vec::new();
    for pos in grid.iter() {
        if visited.contains(&pos) {
            continue;
        }
        let mut neighbors = HashSet::new();
        let mut n_internal_edges = 0;
        traverse_plot(
            grid,
            pos,
            &mut visited,
            &mut neighbors,
            &mut n_internal_edges,
        );
        plots.push((neighbors, n_internal_edges));
    }
    plots
}

fn count_corners(plot: &HashSet<Position>, grid: &Grid<char>) -> usize {
    if plot.len() == 1 || plot.len() == 2 {
        return 4;
    }
    let mut plot_nodes = HashSet::new();
    for (r, c) in plot.iter() {
        for dr in 0..=1 {
            for dc in 0..=1 {
                plot_nodes.insert((*r as i32 + dr, *c as i32 + dc));
            }
        }
    }
    let current_char = grid.get(*plot.iter().next().unwrap()).unwrap();
    let mut n_corners = 0;

    for (r, c) in plot_nodes.iter().copied() {
        let mut neighbors = grid.get_node_neighbors((r as usize, c as usize));
        let n_neighbors = neighbors.len();

        neighbors.retain(|pos| plot.contains(pos) && grid.get(*pos).unwrap() == current_char);
        let n_same = neighbors.len();

        // If the node is a corner, it will have 1 or 3 neighbors
        if n_same == 1 || n_same == 3 {
            n_corners += 1;
            continue;
        }

        // Check if the node has a diagonal of the same chars, e.g, the node (2, 1) in:
        //  AAA
        //  ABA
        //  CAA
        if n_neighbors == 4 && n_same == 2 {
            let (r0, c0) = neighbors[0];
            let (r1, c1) = neighbors[1];
            if r0 != r1 && c0 != c1 {
                n_corners += 2;
            }
        }
    }

    n_corners
}

fn solve_part_one(grid: &Grid<char>) -> usize {
    let plots = find_garden_plots(grid);
    plots.iter().fold(0, |acc, (plants, n_internal_edges)| {
        let n_plants = plants.len();
        let fences = n_plants * 4 - n_internal_edges;
        acc + n_plants * fences
    })
}

fn solve_part_two(grid: &Grid<char>) -> usize {
    let plots = find_garden_plots(grid);
    plots.iter().fold(0, |acc, (plants, _)| {
        let n_corners = count_corners(plants, grid);
        acc + plants.len() * n_corners
    })
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
        assert_eq!(part1, 1930);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/12.test").unwrap();
        let input = Grid::from_str(&input).unwrap();
        let part2 = solve_part_two(&input);
        assert_eq!(part2, 1206);
    }
}
