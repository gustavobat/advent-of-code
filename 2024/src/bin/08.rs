use anyhow::Result;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use utils::grid::Grid;

fn find_antennas(grid: &Grid<char>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut antennas = HashMap::new();
    for (row, col) in grid.iter() {
        if let Some(&c) = grid.get(row, col) {
            if c.is_ascii_alphanumeric() {
                antennas.entry(c).or_insert_with(Vec::new).push((row, col));
            }
        }
    }
    antennas
}

fn calculate_antinodes<F>(
    positions: &[(usize, usize)],
    grid: &Grid<char>,
    mut add_antinodes: F,
) -> HashSet<(i32, i32)>
where
    F: FnMut((i32, i32), (i32, i32), &Grid<char>, &mut HashSet<(i32, i32)>),
{
    let mut antinodes = HashSet::new();
    let pairs = positions.iter().combinations(2);
    for pair in pairs {
        let (a_r, a_c) = pair[0];
        let (a_r, a_c) = (*a_r as i32, *a_c as i32);
        let (b_r, b_c) = pair[1];
        let (b_r, b_c) = (*b_r as i32, *b_c as i32);
        let (delta_r, delta_c) = (b_r - a_r, b_c - a_c);

        add_antinodes((a_r, a_c), (delta_r, delta_c), grid, &mut antinodes);
        add_antinodes((b_r, b_c), (-delta_r, -delta_c), grid, &mut antinodes);
    }
    antinodes
}

fn solve_part_one(grid: &Grid<char>) -> usize {
    let antennas = find_antennas(grid);
    let mut antinodes = HashSet::new();
    for positions in antennas.values() {
        antinodes.extend(calculate_antinodes(
            positions,
            grid,
            |(a_r, a_c), (delta_r, delta_c), grid, antinodes| {
                let (test_r, test_c) = (a_r - delta_r, a_c - delta_c);
                if !grid.is_out_of_bounds(test_r, test_c) {
                    antinodes.insert((test_r, test_c));
                }
            },
        ));
    }
    antinodes.len()
}

fn solve_part_two(grid: &Grid<char>) -> usize {
    let antennas = find_antennas(grid);
    let mut antinodes = HashSet::new();
    for positions in antennas.values() {
        antinodes.extend(calculate_antinodes(
            positions,
            grid,
            |(a_r, a_c), (delta_r, delta_c), grid, antinodes| {
                let (mut test_r, mut test_c) = (a_r + delta_r, a_c + delta_c);
                while !grid.is_out_of_bounds(test_r, test_c) {
                    antinodes.insert((test_r, test_c));
                    test_r += delta_r;
                    test_c += delta_c;
                }
            },
        ));
    }
    antinodes.len()
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/08.input")?;
    let input = Grid::from_str(&input)?;

    let part1 = solve_part_one(&input);
    let part2 = solve_part_two(&input);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day08 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/08.test").unwrap();
        let input = Grid::from_str(&input).unwrap();
        let part1 = solve_part_one(&input);
        assert_eq!(part1, 14);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/08.test").unwrap();
        let input = Grid::from_str(&input).unwrap();
        let part2 = solve_part_two(&input);
        assert_eq!(part2, 34);
    }
}
