use utils::grid::Grid;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 7, solve_all)
}

fn analyse_tachyon_beams(grid: &Grid<char>) -> anyhow::Result<(u64, Vec<u64>)> {
    let start = grid
        .find(|c| *c == 'S')
        .ok_or_else(|| anyhow::anyhow!("No start position found"))?;

    let center_col = start.1;
    let mut split_count = 0;
    let mut beam_count = vec![0; grid.cols()];
    beam_count[center_col] = 1;

    for (max_deflection, r) in (0..grid.rows()).skip(2).enumerate() {
        let col_span = (center_col - max_deflection / 2)..=(center_col + max_deflection / 2);
        for c in col_span {
            let n_arriving_beams = beam_count[c];
            if n_arriving_beams > 0 && grid[(r, c)] == '^' {
                beam_count[c] = 0;
                beam_count[c - 1] += n_arriving_beams;
                beam_count[c + 1] += n_arriving_beams;
                split_count += 1;
            }
        }
    }
    Ok((split_count, beam_count))
}

fn solve_part_one(grid: &Grid<char>) -> anyhow::Result<u64> {
    let (split_count, _) = analyse_tachyon_beams(grid)?;
    Ok(split_count)
}

fn solve_part_two(grid: &Grid<char>) -> anyhow::Result<u64> {
    let (_, beam_count) = analyse_tachyon_beams(grid)?;
    Ok(beam_count.iter().sum::<u64>())
}

fn parse_input(input: &str) -> anyhow::Result<Grid<char>> {
    Grid::from_char_grid_str(input)
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let grid = parse_input(input)?;
    let part_one = solve_part_one(&grid)?.to_string();
    let part_two = solve_part_two(&grid)?.to_string();

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
        assert_eq!(solution, 21);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let grid = parse_input(&input).unwrap();
        let solution = solve_part_two(&grid).unwrap();
        assert_eq!(solution, 40);
    }
}
