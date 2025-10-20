use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::str::FromStr;

pub type Coord = (usize, usize);

pub type CoordDelta = (i32, i32);

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
    pub fn all() -> [Direction; 8] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
    }

    pub fn diagonals() -> [Direction; 4] {
        [
            Direction::UpLeft,
            Direction::UpRight,
            Direction::DownLeft,
            Direction::DownRight,
        ]
    }

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
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    pub fn get(&self, coord: Coord) -> Option<&T> {
        let (row, col) = coord;
        self.data.get(row).and_then(|r| r.get(col))
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data.first().map_or(0, |r| r.len())
    }

    pub fn iter_from_start_and_direction(
        &self,
        start: Coord,
        direction: &Direction,
    ) -> impl Iterator<Item = &T> {
        let (dr, dc) = direction.coord_delta();
        let (row, col) = start;
        let mut r = row as i32;
        let mut c = col as i32;
        std::iter::from_fn(move || {
            let res = self.get((r as usize, c as usize));
            r += dr;
            c += dc;
            res
        })
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = Coord> + use<'_, T> {
        (0..self.rows()).flat_map(move |row| (0..self.cols()).map(move |col| (row, col)))
    }

    pub fn contains(&self, row: i32, col: i32) -> bool {
        row >= 0 && row < self.rows() as i32 && col >= 0 && col < self.cols() as i32
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
