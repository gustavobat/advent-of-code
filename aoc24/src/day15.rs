use anyhow::Result;
use anyhow::anyhow;
use hashbrown::HashMap;
use hashbrown::HashSet;
use utils::grid::Direction;
use utils::grid::Grid;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 15, solve_all)
}

#[derive(Copy, Clone, Debug)]
enum WarehouseTile {
    Free,
    Wall,
    Box,
    WideBox {
        left: (usize, usize),
        right: (usize, usize),
    },
}

#[derive(Copy, Clone, Debug)]
enum PathStatus {
    Free,
    Blocked,
}

impl std::ops::BitOr for PathStatus {
    type Output = PathStatus;

    fn bitor(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (PathStatus::Free, PathStatus::Free) => PathStatus::Free,
            _ => PathStatus::Blocked,
        }
    }
}

fn warehouse_tile(grid: &Grid<char>, pos: (usize, usize)) -> WarehouseTile {
    let c = grid.get(pos).unwrap();
    if *c == '.' {
        return WarehouseTile::Free;
    }
    if *c == 'O' {
        return WarehouseTile::Box;
    }
    if *c == '[' {
        let right = grid
            .get_neighbors(pos, &[Direction::Right])
            .next()
            .expect("Missing box part is out of bounds");
        return WarehouseTile::WideBox { left: pos, right };
    }
    if *c == ']' {
        let left = grid
            .get_neighbors(pos, &[Direction::Left])
            .next()
            .expect("Missing box part is out of bounds");
        return WarehouseTile::WideBox { left, right: pos };
    }
    if *c == '#' {
        return WarehouseTile::Wall;
    }
    unimplemented!("Invalid character: {:?} at position: {:?}", c, pos);
}

fn analyse_path(
    grid: &mut Grid<char>,
    start: (usize, usize),
    direction: Direction,
    movements: &mut HashMap<(usize, usize), (usize, usize)>,
) -> PathStatus {
    let end = grid
        .get_neighbors(start, &[direction])
        .next()
        .expect("Next position is out of bounds");

    let next_tile = warehouse_tile(grid, end);
    match next_tile {
        WarehouseTile::Free => {
            movements.insert(end, start);
            PathStatus::Free
        }
        WarehouseTile::Wall => PathStatus::Blocked,
        WarehouseTile::Box => {
            movements.insert(end, start);
            analyse_path(grid, end, direction, movements)
        }
        WarehouseTile::WideBox { left, right } => match direction {
            Direction::Left => {
                movements.insert(right, start);
                movements.insert(left, right);
                analyse_path(grid, left, direction, movements)
            }
            Direction::Right => {
                movements.insert(left, start);
                movements.insert(right, left);
                analyse_path(grid, right, direction, movements)
            }
            Direction::Up | Direction::Down => {
                let (_, from_col) = start;
                let (_, left_col) = left;
                if from_col == left_col {
                    movements.insert(left, start);
                } else {
                    movements.insert(right, start);
                }
                analyse_path(grid, right, direction, movements)
                    | analyse_path(grid, left, direction, movements)
            }
            _ => unimplemented!("Invalid direction"),
        },
    }
}

fn try_move(
    grid: &mut Grid<char>,
    start: (usize, usize),
    direction: Direction,
) -> Option<(usize, usize)> {
    let Some(target) = grid.get_neighbors(start, &[direction]).next() else {
        panic!("Position is out of bounds");
    };

    let mut movements = HashMap::new();
    let PathStatus::Free = analyse_path(grid, start, direction, &mut movements) else {
        return None;
    };
    let ends: HashSet<_> = movements.keys().copied().collect();
    let starts: HashSet<_> = movements.values().copied().collect();

    let spaces_to_fill = ends.difference(&starts);
    for space in spaces_to_fill {
        let mut cell_to_update = *space;
        while let Some(previous) = movements.remove(&cell_to_update) {
            let parent_char = grid.get(previous).unwrap();
            grid.set(cell_to_update, *parent_char).ok()?;
            cell_to_update = previous;
        }
    }

    let spaces_left = starts.difference(&ends);
    for space in spaces_left {
        grid.set(*space, '.').ok()?;
    }
    Some(target)
}

fn rescale_grid(grid: &Grid<char>) -> Result<Grid<char>> {
    let mut new_grid_data = vec![vec!['.'; 2 * grid.cols()]; grid.rows()];
    for (r, c) in grid.iter_flat_indices() {
        let old_char = grid.get((r, c)).unwrap();
        let (left, right) = match old_char {
            '#' => ('#', '#'),
            'O' => ('[', ']'),
            '@' => ('@', '.'),
            '.' => continue,
            c => anyhow::bail!("Found invalid character during rescaling: {c}"),
        };
        new_grid_data[r][2 * c] = left;
        new_grid_data[r][2 * c + 1] = right;
    }
    let new_grid = Grid::new(new_grid_data);
    Ok(new_grid)
}

fn calculate_gps_coords(grid: &Grid<char>) -> usize {
    grid.iter_flat_indices()
        .filter_map(|(r, c)| {
            if grid.get((r, c)).is_some_and(|&c| c == 'O' || c == '[') {
                Some(r * 100 + c)
            } else {
                None
            }
        })
        .sum()
}

fn parse_input(input: &str) -> Result<(Grid<char>, Vec<Direction>)> {
    let (grid_str, directions_str) = input.split_once("\n\n").ok_or(anyhow!("No directions"))?;
    let grid = Grid::from_char_grid_str(grid_str)?;
    let directions = directions_str
        .lines()
        .flat_map(|line| {
            line.chars().filter_map(|c| match c {
                '^' => Some(Direction::Up),
                'v' => Some(Direction::Down),
                '>' => Some(Direction::Right),
                '<' => Some(Direction::Left),
                _ => None,
            })
        })
        .collect::<Vec<_>>();
    Ok((grid, directions))
}

fn solve_part_one(grid: &mut Grid<char>, directions: &[Direction]) -> Result<usize> {
    let mut current_pos = grid
        .find(|c| c == &'@')
        .ok_or(anyhow!("Could not find start position"))?;

    directions.iter().for_each(|d| {
        if let Some(next_pos) = try_move(grid, current_pos, *d) {
            current_pos = next_pos;
        }
    });

    Ok(calculate_gps_coords(grid))
}

fn solve_part_two(grid: &Grid<char>, directions: &[Direction]) -> Result<usize> {
    let mut grid = rescale_grid(grid)?;
    let mut current_pos = grid
        .find(|c| c == &'@')
        .ok_or(anyhow!("Could not find start position"))?;

    directions.iter().for_each(|d| {
        if let Some(next_pos) = try_move(&mut grid, current_pos, *d) {
            current_pos = next_pos;
        }
    });

    Ok(calculate_gps_coords(&grid))
}

fn solve_all(input: &str) -> Result<Solution> {
    let (grid, directions) = parse_input(input)?;
    let part_one = solve_part_one(&mut grid.clone(), &directions)?.to_string();
    let part_two = solve_part_two(&grid, &directions)?.to_string();

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
        let (mut grid, directions) = parse_input(&input).unwrap();
        let part_one = solve_part_one(&mut grid, &directions).unwrap();
        assert_eq!(part_one, 10092);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let (grid, directions) = parse_input(&input).unwrap();
        let part_two = solve_part_two(&grid, &directions).unwrap();
        assert_eq!(part_two, 9021);
    }
}
