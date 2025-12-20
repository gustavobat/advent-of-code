use anyhow::Result;
use std::collections::HashSet;
use std::collections::VecDeque;
use utils::grid::Direction;
use utils::grid::Grid;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 12, solve_all)
}

#[derive(Debug, Clone)]
struct Region {
    plots: HashSet<(usize, usize)>,
    internal_edges: usize,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        self.area() * 4 - self.internal_edges
    }
}

fn find_regions(grid: &Grid<char>) -> Vec<Region> {
    let mut visited = HashSet::new();
    let mut regions = Vec::new();
    for start in grid.iter_flat_indices() {
        if visited.contains(&start) {
            continue;
        }
        let region = traverse_region(grid, start);
        visited.extend(region.plots.iter().copied());
        regions.push(region);
    }
    regions
}

fn traverse_region(grid: &Grid<char>, start: (usize, usize)) -> Region {
    let mut plots = HashSet::from([start]);
    let mut queue = VecDeque::new();

    let mut internal_edges = 0;
    let cur_plant = grid[start];

    queue.push_back(start);

    while let Some(pos) = queue.pop_front() {
        let directions = Direction::cardinals();
        let neighbors = grid.get_neighbors(pos, &directions).filter(|n| {
            let neigh_plant = grid[*n];
            neigh_plant == cur_plant
        });

        for neighbor in neighbors {
            internal_edges += 1;
            if plots.contains(&neighbor) {
                continue;
            }
            plots.insert(neighbor);
            queue.push_back(neighbor);
        }
    }

    Region {
        plots,
        internal_edges,
    }
}

// We count the sides by counting the corners of the plots,
// since a polygon has as many sides as its corners.
fn count_sides(plots: &HashSet<(usize, usize)>, grid: &Grid<char>) -> usize {
    if plots.len() == 1 || plots.len() == 2 {
        return 4;
    }

    // Insert all corners of each plot into the set.
    let mut plot_corners = HashSet::new();
    for (r, c) in plots.iter() {
        for dr in 0..=1 {
            for dc in 0..=1 {
                plot_corners.insert((*r + dr, *c + dc));
            }
        }
    }

    let mut n_sides = 0;
    for corner in plot_corners.iter().copied() {
        let mut corner_neighbors = grid.get_corner_neighbors(corner).collect::<Vec<_>>();
        let n_neighbors = corner_neighbors.len();

        // Remove any plots that are not in the region.
        corner_neighbors.retain(|pos| plots.contains(pos));
        let valid_connections = corner_neighbors.len();

        // If the node is a corner, it will have 1 or 3 plots connected to it.
        if valid_connections == 1 || valid_connections == 3 {
            n_sides += 1;
            continue;
        }

        // Check if the region has diagonally connected plots, e.g.,
        // the pair (0,0) && (1, 1) below:
        //  AAA
        //  A.A
        //  .AA
        if n_neighbors == 4 && valid_connections == 2 {
            let (r0, c0) = corner_neighbors[0];
            let (r1, c1) = corner_neighbors[1];
            if r0 != r1 && c0 != c1 {
                n_sides += 2;
            }
        }
    }

    n_sides
}

fn solve_part_one(plots: &[Region]) -> usize {
    plots
        .iter()
        .map(|region| region.area() * region.perimeter())
        .sum()
}

fn solve_part_two(grid: &Grid<char>, plots: &[Region]) -> usize {
    plots
        .iter()
        .map(|region| region.area() * count_sides(&region.plots, grid))
        .sum()
}

fn parse_input(input: &str) -> Result<Grid<char>> {
    Grid::from_char_grid_str(input)
}

fn solve_all(input: &str) -> Result<Solution> {
    let grid = parse_input(input)?;
    let regions = find_regions(&grid);
    let part_one = solve_part_one(&regions).to_string();
    let part_two = solve_part_two(&grid, &regions).to_string();

    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    use super::find_regions;
    use super::parse_input;
    use super::solve_part_one;
    use super::solve_part_two;
    use utils::load_test_input;

    #[test]
    fn part_one() {
        let input = load_test_input!();
        let grid = parse_input(&input).unwrap();
        let regions = find_regions(&grid);
        let solution = solve_part_one(&regions);
        assert_eq!(solution, 1930);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let grid = parse_input(&input).unwrap();
        let regions = find_regions(&grid);
        let solution = solve_part_two(&grid, &regions);
        assert_eq!(solution, 1206);
    }
}
