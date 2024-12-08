use std::str::FromStr;
use utils::grid::Direction;
use utils::grid::Grid;
fn solve_part_one(grid: &Grid<char>) -> usize {
    let directions = [
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
        Direction::UpLeft,
        Direction::UpRight,
        Direction::DownLeft,
        Direction::DownRight,
    ];

    let is_xmas = |mut iter: Box<dyn Iterator<Item = &char>>| -> bool {
        let target = ['X', 'M', 'A', 'S'];
        for &ch in &target {
            if iter.next() != Some(&ch) {
                return false;
            }
        }
        true
    };

    let mut count = 0;
    for (row, col) in grid.iter() {
        if grid.get(row, col) == Some(&'X') {
            count += directions
                .iter()
                .map(|dir| is_xmas(Box::new(grid.iter_direction(row, col, *dir))))
                .filter(|&b| b)
                .count();
        }
    }
    count
}

fn solve_part_two(grid: &Grid<char>) -> usize {
    let mut count = 0;
    for (row, col) in grid.iter() {
        if row < 1 || col < 1 {
            continue;
        }
        if let (Some(&'A'), Some(&c11), Some(&c12), Some(&c21), Some(&c22)) = (
            grid.get(row, col),
            grid.get(row - 1, col - 1),
            grid.get(row + 1, col + 1),
            grid.get(row - 1, col + 1),
            grid.get(row + 1, col - 1),
        ) {
            let is_mas =
                |c1: char, c2: char| -> bool { (c1, c2) == ('M', 'S') || (c1, c2) == ('S', 'M') };

            if is_mas(c11, c12) && is_mas(c21, c22) {
                count += 1;
            }
        }
    }
    count
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("data/04.input")?;
    let input = Grid::from_str(&input)?;
    let part1 = solve_part_one(&input);
    let part2 = solve_part_two(&input);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day04 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/04.test").unwrap();
        let input = Grid::from_str(&input).unwrap();
        let part1 = solve_part_one(&input);
        assert_eq!(part1, 18);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/04.test").unwrap();
        let input = Grid::from_str(&input).unwrap();
        let part2 = solve_part_two(&input);
        assert_eq!(part2, 9);
    }
}
