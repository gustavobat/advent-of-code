use anyhow::Result;
use anyhow::anyhow;
use std::collections::HashMap;
use std::collections::HashSet;
use utils::grid::Direction;
use utils::grid::Grid;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 6, solve_all)
}

#[derive(Debug, Clone)]
enum GuardRoute {
    InfiniteLoop,
    Route(HashSet<(usize, usize)>),
}

fn analyze_guard_route(grid: &Grid<char>) -> Result<GuardRoute> {
    let mut cur_dir = Direction::Up;
    let mut cur_pos = grid
        .find(|char| *char == '^')
        .ok_or(anyhow!("No starting point found"))?;

    let mut visited = HashSet::new();
    visited.insert(cur_pos);

    let mut turns = HashMap::new();
    let mut just_turned = false;

    let mut cur_path = grid.iter_from_start_and_direction(cur_pos, cur_dir).skip(1);
    while let Some(next) = cur_path.next() {
        if *next == '#' {
            if turns.get(&cur_pos).is_some_and(|&dir| dir == cur_dir) {
                return Ok(GuardRoute::InfiniteLoop);
            }
            if !just_turned {
                turns.insert(cur_pos, cur_dir);
            }
            just_turned = true;
            cur_dir = cur_dir.rotate_right();
            cur_path = grid.iter_from_start_and_direction(cur_pos, cur_dir).skip(1);
            continue;
        }
        if *next == '.' || *next == '^' {
            just_turned = false;
            let cur_pos_coord = cur_pos.into();
            cur_pos = cur_dir.apply_to_coord(cur_pos_coord).try_into()?;
            visited.insert(cur_pos);
        }
    }

    Ok(GuardRoute::Route(visited))
}
fn solve_part_one(grid: &Grid<char>) -> Result<usize> {
    let route = analyze_guard_route(grid)?;
    let GuardRoute::Route(visited) = route else {
        return Err(anyhow!("Unexpected infinite loop"));
    };
    Ok(visited.len())
}

fn solve_part_two(grid: &Grid<char>) -> Result<usize> {
    let route = analyze_guard_route(grid)?;
    let GuardRoute::Route(visited) = route else {
        return Err(anyhow!("Unexpected infinite loop"));
    };
    let infinite_loops = visited
        .iter()
        .map(|pos| {
            let mut new_grid = grid.clone();
            new_grid.set(*pos, '#')?;
            Ok(matches!(
                analyze_guard_route(&new_grid),
                Ok(GuardRoute::InfiniteLoop)
            ))
        })
        .collect::<Result<Vec<_>>>()?;

    let count = infinite_loops.iter().filter(|&&b| b).count();
    Ok(count)
}

fn parse_input(input: &str) -> Result<Grid<char>> {
    Grid::from_char_grid_str(input)
}

fn solve_all(input: &str) -> Result<Solution> {
    let input = parse_input(input)?;
    let part_one = solve_part_one(&input)?.to_string();
    let part_two = solve_part_two(&input)?.to_string();

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
        let grid = parse_input(&input).unwrap();
        let solution = solve_part_one(&grid).unwrap();
        assert_eq!(solution, 41);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let grid = parse_input(&input).unwrap();
        let solution = solve_part_two(&grid).unwrap();
        assert_eq!(solution, 6);
    }
}
