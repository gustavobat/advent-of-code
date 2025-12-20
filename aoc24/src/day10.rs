use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use utils::grid::Direction;
use utils::grid::Grid;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 10, solve_all)
}

type TrailHead = HashMap<(usize, usize), Vec<(usize, usize)>>;

fn breadth_first_search(grid: &Grid<u8>, start: (usize, usize)) -> TrailHead {
    let mut result: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    while let Some(pos) = queue.pop_front() {
        let cur_height = grid[pos];

        let directions = Direction::cardinals();
        let neighbors = grid.get_neighbors(pos, &directions).filter(|n| {
            let neighbor_height = grid[*n];
            neighbor_height == cur_height + 1
        });

        for neighbor in neighbors {
            let neighbor_height = grid[neighbor];
            if neighbor_height == 9 {
                result.entry(start).or_default().push(neighbor);
            }
            queue.push_back(neighbor);
        }
    }
    result
}

fn find_trailheads(grid: &Grid<u8>) -> Vec<TrailHead> {
    grid.iter_flat_indices()
        .filter(|(r, c)| grid.get((*r, *c)) == Some(&0))
        .map(|pos| breadth_first_search(grid, pos))
        .collect()
}

fn solve_part_one(trailheads: &[TrailHead]) -> usize {
    trailheads
        .iter()
        .flat_map(|trails| {
            trails.values().map(|trail| {
                let unique_ends = HashSet::<(usize, usize)>::from_iter(trail.iter().copied());
                unique_ends.len()
            })
        })
        .sum()
}

fn solve_part_two(trailheads: &[TrailHead]) -> usize {
    trailheads
        .iter()
        .flat_map(|trails| trails.values().map(|trail| trail.len()))
        .sum()
}

fn parse_input(input: &str) -> anyhow::Result<Grid<u8>> {
    Grid::from_char_grid_str(input)
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let input = parse_input(input)?;
    let trailheads = find_trailheads(&input);
    let part_one = solve_part_one(&trailheads).to_string();
    let part_two = solve_part_two(&trailheads).to_string();

    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    use super::find_trailheads;
    use super::parse_input;
    use super::solve_part_one;
    use super::solve_part_two;
    use utils::load_test_input;

    #[test]
    fn part_one() {
        let input = load_test_input!();
        let grid = parse_input(&input).unwrap();
        let trailheads = find_trailheads(&grid);
        let solution = solve_part_one(&trailheads);
        assert_eq!(solution, 36);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let grid = parse_input(&input).unwrap();
        let trailheads = find_trailheads(&grid);
        let solution = solve_part_two(&trailheads);
        assert_eq!(solution, 81);
    }
}
