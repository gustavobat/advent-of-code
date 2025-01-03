use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

pub type Position = (usize, usize);

pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    pub fn move_position_unchecked(&self, (row, col): Position) -> Position {
        match self {
            Direction::Up => (row - 1, col),
            Direction::Down => (row + 1, col),
            Direction::Left => (row, col - 1),
            Direction::Right => (row, col + 1),
            Direction::UpLeft => (row - 1, col - 1),
            Direction::UpRight => (row - 1, col + 1),
            Direction::DownLeft => (row + 1, col - 1),
            Direction::DownRight => (row + 1, col + 1),
        }
    }

    pub fn move_position(
        &self,
        (row, col): Position,
        (n_rows, n_cols): Position,
    ) -> Option<Position> {
        match self {
            Direction::Up => {
                if row == 0 {
                    None
                } else {
                    Some((row - 1, col))
                }
            }
            Direction::Down => {
                if row == n_rows - 1 {
                    None
                } else {
                    Some((row + 1, col))
                }
            }
            Direction::Left => {
                if col == 0 {
                    None
                } else {
                    Some((row, col - 1))
                }
            }
            Direction::Right => {
                if col == n_cols - 1 {
                    None
                } else {
                    Some((row, col + 1))
                }
            }
            Direction::UpLeft => {
                if row == 0 || col == 0 {
                    None
                } else {
                    Some((row - 1, col - 1))
                }
            }
            Direction::UpRight => {
                if row == 0 || col == n_cols - 1 {
                    None
                } else {
                    Some((row - 1, col + 1))
                }
            }
            Direction::DownLeft => {
                if row == n_rows - 1 || col == 0 {
                    None
                } else {
                    Some((row + 1, col - 1))
                }
            }
            Direction::DownRight => {
                if row == n_rows - 1 || col == n_cols - 1 {
                    None
                } else {
                    Some((row + 1, col + 1))
                }
            }
        }
    }
}

impl Direction {
    pub fn coord_delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (-1, 1),
            Direction::DownLeft => (1, -1),
            Direction::DownRight => (1, 1),
        }
    }
    pub fn rotate_right(&mut self) {
        *self = match &self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            _ => unimplemented!(),
        }
    }
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        let (row, col) = position;
        self.data.get(row).and_then(|r| r.get(col))
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
        let (row, col) = position;
        self.data.get_mut(row).and_then(|r| r.get_mut(col))
    }

    pub fn set(&mut self, position: Position, value: T) {
        let (row, col) = position;
        self.data[row][col] = value;
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data.first().map_or(0, |r| r.len())
    }

    pub fn iter_direction(
        &self,
        position: Position,
        direction: Direction,
    ) -> impl Iterator<Item = (Position, &T)> {
        let (dr, dc) = direction.coord_delta();
        let (row, col) = position;
        let mut r = row as i32;
        let mut c = col as i32;
        std::iter::once((position, self.get(position).unwrap())).chain(std::iter::from_fn(
            move || {
                r += dr;
                c += dc;
                if r < 0 || r >= self.rows() as i32 || c < 0 || c >= self.cols() as i32 {
                    None
                } else {
                    let pos = (r as usize, c as usize);
                    let val = self.get(pos).unwrap();
                    Some((pos, val))
                }
            },
        ))
    }

    pub fn iter(&self) -> impl Iterator<Item = Position> + use<'_, T> {
        (0..self.rows()).flat_map(move |row| (0..self.cols()).map(move |col| (row, col)))
    }

    pub fn is_out_of_bounds(&self, row: i32, col: i32) -> bool {
        row < 0 || row >= self.rows() as i32 || col < 0 || col >= self.cols() as i32
    }

    pub fn get_cardinal_neighbors(&self, position: Position) -> Vec<(Direction, Position)> {
        let mut neighbors = Vec::new();
        for direction in &[
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ] {
            if let Some(pos) = direction.move_position(position, (self.rows(), self.cols())) {
                neighbors.push((*direction, pos));
            }
        }
        neighbors
    }

    pub fn get_all_neighbors(&self, position: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let row = position.0 as i32;
        let col = position.1 as i32;
        for dr in -1..=1 {
            for dc in -1..=1 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let r = row + dr;
                let c = col + dc;
                if !self.is_out_of_bounds(r, c) {
                    neighbors.push((r as usize, c as usize));
                }
            }
        }
        neighbors
    }

    pub fn get_node_neighbors(&self, node_position: Position) -> Vec<Position> {
        let mut neighbors = Vec::new();
        let row = node_position.0 as i32;
        let col = node_position.1 as i32;
        for dr in -1..=0 {
            for dc in -1..=0 {
                let r = row + dr;
                let c = col + dc;
                if !self.is_out_of_bounds(r, c) {
                    neighbors.push((r as usize, c as usize));
                }
            }
        }
        neighbors
    }
}

impl<T: PartialEq> Grid<T> {
    pub fn find_position(&self, target: &T) -> Option<Position> {
        self.iter().find(|&pos| self.get(pos) == Some(target))
    }
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl FromStr for Grid<char> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| line.chars().collect())
            .collect::<Vec<_>>();
        Ok(Self::new(data))
    }
}

impl FromStr for Grid<u8> {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
            .collect::<Vec<_>>();
        Ok(Self::new(data))
    }
}

impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
