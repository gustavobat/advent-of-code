use anyhow::Result;
use itertools::Either;

fn is_visible(i: usize, j: usize, grid: &[Vec<u32>]) -> bool {
    let rows = grid.len();
    let cols = grid[0].len();

    // Handle edge trees
    if i == 0 || j == 0 || i == rows - 1 || j == cols - 1 {
        return true;
    }

    let cur_height = grid[i][j];
    let ranges = [
        Either::Left((0..j).rev()),
        Either::Right(j + 1..cols),
        Either::Left((0..i).rev()),
        Either::Right(i + 1..rows),
    ];
    ranges.iter().enumerate().any(|(idx, range)| {
        range
            .clone()
            .map(|k| match idx {
                0 | 1 => grid[i][k],
                2 | 3 => grid[k][j],
                _ => unreachable!(),
            })
            .all(|height| height < cur_height)
    })
}

fn scenic_score(i: usize, j: usize, grid: &[Vec<u32>]) -> u32 {
    let rows = grid.len();
    let cols = grid[0].len();

    // Handle edge trees
    if i == 0 || j == 0 || i == rows - 1 || j == cols - 1 {
        return 0;
    }

    // Current height
    let cur_height = grid[i][j];
    let ranges = [
        Either::Left((0..j).rev()),
        Either::Right(j + 1..cols),
        Either::Left((0..i).rev()),
        Either::Right(i + 1..rows),
    ];

    ranges.iter().enumerate().fold(1, |acc, (idx, range)| {
        let mut visible_trees = 0;
        for k in range.clone() {
            visible_trees += 1;
            let height = match idx {
                0 | 1 => grid[i][k],
                2 | 3 => grid[k][j],
                _ => unreachable!(),
            };
            if height >= cur_height {
                break;
            }
        }
        acc * visible_trees
    })
}

fn solve_part1(grid: &[Vec<u32>]) -> u32 {
    grid.iter().enumerate().fold(0, |acc, (i, row)| {
        acc + row
            .iter()
            .enumerate()
            .filter(|(j, _)| is_visible(i, *j, grid))
            .count() as u32
    })
}

fn solve_part2(grid: &[Vec<u32>]) -> u32 {
    grid.iter().enumerate().fold(0, |max_val, (i, row)| {
        max_val.max(
            row.iter()
                .enumerate()
                .map(|(j, _)| scenic_score(i, j, grid))
                .max()
                .unwrap_or(0),
        )
    })
}

fn get_input_grid<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Vec<u32>>> {
    let grid = std::fs::read_to_string(path)?
        .trim_end()
        .split('\n')
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Could not parse digit to u32"))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    Ok(grid)
}

fn main() -> Result<()> {
    let grid = get_input_grid("./data/08.input")?;

    let n_visible = solve_part1(&grid);
    println!("Part 1: {n_visible}");

    let highest_score = solve_part2(&grid);
    println!("Part 2: {highest_score}");

    Ok(())
}

#[cfg(test)]
mod day08 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let grid = get_input_grid("./data/08.test")?;
        assert_eq!(solve_part1(&grid), 21);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let grid = get_input_grid("./data/08.test")?;
        assert_eq!(solve_part2(&grid), 8);
        Ok(())
    }
}
