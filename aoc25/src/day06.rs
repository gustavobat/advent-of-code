use std::ops::RangeInclusive;
use utils::grid::Grid;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 6, solve_all)
}

#[derive(Debug)]
struct Block {
    row_span: RangeInclusive<usize>,
    col_span: RangeInclusive<usize>,
    operation: char,
}

fn calculate_blocks(grid: &Grid<char>) -> Vec<Block> {
    let mut blocks = Vec::new();
    let op_row = grid.rows() - 1;
    let mut right = grid.cols() - 1;
    for left in (0..right).rev() {
        let operation = grid[(op_row, left)];
        if operation == ' ' {
            continue;
        }

        let col_span = left..=right;
        let row_span = 0..=op_row - 1;
        blocks.push(Block {
            row_span,
            col_span,
            operation,
        });
        right = left.saturating_sub(2);
    }
    blocks
}

#[inline]
fn digits_to_u64(digits: impl Iterator<Item = char>) -> u64 {
    digits.fold(0, |num, ch| {
        if ch == ' ' {
            num
        } else {
            num * 10 + (ch as u8 - b'0') as u64
        }
    })
}

#[inline]
fn apply_operation(numbers: impl Iterator<Item = u64>, operation: char) -> anyhow::Result<u64> {
    match operation {
        '+' => Ok(numbers.sum()),
        '*' => Ok(numbers.product()),
        _ => Err(anyhow::anyhow!("Invalid operation: {}", operation)),
    }
}

fn solve_part_one(grid: &Grid<char>, blocks: &[Block]) -> anyhow::Result<u64> {
    blocks
        .iter()
        .map(|block| {
            let numbers = block.row_span.clone().map(|r| {
                let iter = block.col_span.clone().map(|c| grid[(r, c)]);
                digits_to_u64(iter)
            });
            apply_operation(numbers, block.operation)
        })
        .sum()
}

fn solve_part_two(grid: &Grid<char>, blocks: &[Block]) -> anyhow::Result<u64> {
    blocks
        .iter()
        .map(|block| {
            let numbers = block.col_span.clone().map(|c| {
                let iter = block.row_span.clone().map(|r| grid[(r, c)]);
                digits_to_u64(iter)
            });
            apply_operation(numbers, block.operation)
        })
        .sum()
}

fn parse_input(input: &str) -> anyhow::Result<Grid<char>> {
    Grid::from_char_grid_str(input)
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let grid = parse_input(input)?;
    let blocks = calculate_blocks(&grid);
    let part_one = solve_part_one(&grid, &blocks)?.to_string();
    let part_two = solve_part_two(&grid, &blocks)?.to_string();

    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    use super::calculate_blocks;
    use super::parse_input;
    use super::solve_part_one;
    use super::solve_part_two;
    use utils::load_test_input;

    #[test]
    fn part_one() {
        let input = load_test_input!();
        let grid = parse_input(&input).unwrap();
        let blocks = calculate_blocks(&grid);
        let solution = solve_part_one(&grid, &blocks).unwrap();
        assert_eq!(solution, 4277556);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let grid = parse_input(&input).unwrap();
        let blocks = calculate_blocks(&grid);
        let solution = solve_part_two(&grid, &blocks).unwrap();
        assert_eq!(solution, 3263827);
    }
}
