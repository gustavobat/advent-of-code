use anyhow::{anyhow, Result};
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use utils::grid::Direction;
use utils::grid::Grid;

#[derive(Debug, Clone)]
enum GuardRoute {
    InfiniteLoop,
    Route(HashSet<(usize, usize)>),
}

fn analyze_guard_route(grid: &Grid<char>) -> Result<GuardRoute> {
    let mut start = grid
        .iter()
        .find(|pos| grid.get(*pos) == Some(&'^'))
        .ok_or(anyhow!("No starting point found"))?;

    let mut direction = Direction::Up;

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut turns = HashMap::new();
    let mut just_turned = false;

    let mut iter = grid.iter_direction(start, direction).skip(1);
    while let Some((_, next)) = iter.next() {
        if *next == '#' {
            if turns.get(&start).is_some_and(|&dir| dir == direction) {
                return Ok(GuardRoute::InfiniteLoop);
            }
            if !just_turned {
                // Avoid updating the direction if we just turned
                turns.insert(start, direction);
            }
            just_turned = true;
            direction.rotate_right();
            iter = grid.iter_direction(start, direction).skip(1);
            continue;
        }
        if *next == '.' || *next == '^' {
            just_turned = false;
            start = direction.move_position_unchecked(start);
            visited.insert(start);
        }
    }

    Ok(GuardRoute::Route(visited))
}

fn solve_part_one(grid: &Grid<char>) -> Result<usize> {
    let Ok(GuardRoute::Route(visited)) = analyze_guard_route(grid) else {
        return Err(anyhow!("Unexpected infinite loop"));
    };
    Ok(visited.len())
}

fn solve_part_two(grid: &Grid<char>) -> Result<usize> {
    let Ok(GuardRoute::Route(visited)) = analyze_guard_route(grid) else {
        return Err(anyhow!("Unexpected infinite loop"));
    };
    let infinite_loop_count = visited
        .iter()
        .filter(|&pos| {
            let mut new_grid = grid.clone();
            new_grid.set(*pos, '#');
            matches!(analyze_guard_route(&new_grid), Ok(GuardRoute::InfiniteLoop))
        })
        .count();
    Ok(infinite_loop_count)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/06.input")?;
    let input = Grid::from_str(&input)?;

    let part1 = solve_part_one(&input)?;
    let part2 = solve_part_two(&input)?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day06 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/06.test").unwrap();
        let input = Grid::from_str(&input).unwrap();
        let part1 = solve_part_one(&input).unwrap();
        assert_eq!(part1, 41);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/06.test").unwrap();
        let input = Grid::from_str(&input).unwrap();
        let part2 = solve_part_two(&input).unwrap();
        assert_eq!(part2, 6);
    }
}
