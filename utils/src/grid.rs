use std::fmt::{Debug, Formatter};
use std::str::FromStr;

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
    pub fn move_position_unchecked(&self, (row, col): (usize, usize)) -> (usize, usize) {
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

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.data.get(row).and_then(|r| r.get(col))
    }

    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.data.get_mut(row).and_then(|r| r.get_mut(col))
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
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
        row: usize,
        col: usize,
        direction: Direction,
    ) -> impl Iterator<Item = &T> {
        let (dr, dc) = direction.coord_delta();
        let mut r = row as i32;
        let mut c = col as i32;
        std::iter::once(self.get(row, col).unwrap()).chain(std::iter::from_fn(move || {
            r += dr;
            c += dc;
            if r < 0 || r >= self.rows() as i32 || c < 0 || c >= self.cols() as i32 {
                None
            } else {
                Some(self.get(r as usize, c as usize).unwrap())
            }
        }))
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, usize)> + use<'_, T> {
        (0..self.rows()).flat_map(move |row| (0..self.cols()).map(move |col| (row, col)))
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