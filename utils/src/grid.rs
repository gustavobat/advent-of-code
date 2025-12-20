use anyhow::anyhow;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Index;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GridVector(pub i32, pub i32);

impl GridVector {
    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }
}

impl std::ops::Add for GridVector {
    type Output = GridVector;
    fn add(self, other: GridVector) -> Self::Output {
        GridVector(self.0 + other.0, self.1 + other.1)
    }
}

impl std::ops::AddAssign for GridVector {
    fn add_assign(&mut self, other: GridVector) {
        *self = *self + other;
    }
}

impl std::ops::Sub for GridVector {
    type Output = GridVector;
    fn sub(self, other: GridVector) -> Self::Output {
        GridVector(self.0 - other.0, self.1 - other.1)
    }
}

impl std::ops::SubAssign for GridVector {
    fn sub_assign(&mut self, other: GridVector) {
        *self = *self - other;
    }
}

impl std::ops::Neg for GridVector {
    type Output = GridVector;
    fn neg(self) -> Self::Output {
        GridVector(-self.0, -self.1)
    }
}

impl std::ops::Mul<i32> for GridVector {
    type Output = GridVector;
    fn mul(self, other: i32) -> Self::Output {
        GridVector(self.0 * other, self.1 * other)
    }
}

impl Display for GridVector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl TryFrom<GridVector> for (usize, usize) {
    type Error = anyhow::Error;
    fn try_from(value: GridVector) -> Result<Self, Self::Error> {
        if value.0 < 0 || value.1 < 0 {
            return Err(anyhow!("Negative coordinate cannot be converted to usize"));
        }
        Ok((value.0 as usize, value.1 as usize))
    }
}

impl From<(usize, usize)> for GridVector {
    fn from(value: (usize, usize)) -> Self {
        GridVector(value.0 as i32, value.1 as i32)
    }
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

    pub fn cardinals() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
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

    pub fn as_grid_vector(&self) -> GridVector {
        match self {
            Direction::Up => GridVector(-1, 0),
            Direction::Down => GridVector(1, 0),
            Direction::Left => GridVector(0, -1),
            Direction::Right => GridVector(0, 1),
            Direction::UpLeft => GridVector(-1, -1),
            Direction::UpRight => GridVector(-1, 1),
            Direction::DownLeft => GridVector(1, -1),
            Direction::DownRight => GridVector(1, 1),
        }
    }

    pub fn rotate_right(&self) -> Direction {
        match &self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
            _ => unimplemented!(),
        }
    }
}

pub struct Entry<'a, T> {
    pub pos: (usize, usize),
    pub value: &'a T,
}

pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        Self { data }
    }

    pub fn get(&self, index: (usize, usize)) -> Option<&T> {
        self.data.get(index.0).and_then(|r| r.get(index.1))
    }

    pub fn set(&mut self, index: (usize, usize), value: T) -> anyhow::Result<()> {
        if !self.contains(index) {
            return Err(anyhow!("Invalid coordinate"));
        }
        self.data[index.0][index.1] = value;
        Ok(())
    }

    pub fn rows(&self) -> usize {
        self.data.len()
    }

    pub fn cols(&self) -> usize {
        self.data.first().map_or(0, |r| r.len())
    }

    pub fn iter_from_start_and_direction(
        &self,
        start: (usize, usize),
        direction: Direction,
    ) -> impl Iterator<Item = Entry<'_, T>> {
        let GridVector(dr, dc) = direction.as_grid_vector();
        let (row, col) = start;
        let mut r = row as i32;
        let mut c = col as i32;
        std::iter::from_fn(move || {
            let entry = self.get((r as usize, c as usize)).map(|v| Entry {
                pos: (r as usize, c as usize),
                value: v,
            });
            r += dr;
            c += dc;
            entry
        })
    }

    pub fn iter_flat_indices(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.rows()).flat_map(move |row| (0..self.cols()).map(move |col| (row, col)))
    }

    pub fn contains(&self, index: (usize, usize)) -> bool {
        index.0 < self.rows() && index.1 < self.cols()
    }

    pub fn find(&self, predicate: impl Fn(&T) -> bool) -> Option<(usize, usize)> {
        self.iter_flat_indices()
            .find(|pos| predicate(self.get(*pos).unwrap()))
    }

    pub fn get_neighbors(
        &self,
        index: (usize, usize),
        directions: &[Direction],
    ) -> impl Iterator<Item = (usize, usize)> {
        directions.iter().filter_map(move |dir| {
            let GridVector(dr, dc) = dir.as_grid_vector();
            let new_row = index.0 as i32 + dr;
            let new_col = index.1 as i32 + dc;
            if new_row >= 0 && new_col >= 0 {
                let new_index = (new_row as usize, new_col as usize);
                if self.contains(new_index) {
                    return Some(new_index);
                }
            }
            None
        })
    }

    /// Get the grid cells that are connected to a given corner coordinate.
    /// For a grid of size n x m, there are (n + 1) x (m + 1) corners.
    pub fn get_corner_neighbors(
        &self,
        corner_coord: (usize, usize),
    ) -> impl Iterator<Item = (usize, usize)> {
        self.get_neighbors(
            corner_coord,
            &[Direction::Up, Direction::Left, Direction::UpLeft],
        )
        .chain(std::iter::once(corner_coord))
    }

    pub fn get_relative_cells(
        &self,
        reference: (usize, usize),
        offsets: &[GridVector],
    ) -> impl Iterator<Item = (usize, usize)> {
        offsets.iter().filter_map(move |offset| {
            let new_row = reference.0 as i32 + offset.0;
            let new_col = reference.1 as i32 + offset.1;
            if new_row >= 0 && new_col >= 0 {
                let new_index = (new_row as usize, new_col as usize);
                if self.contains(new_index) {
                    return Some(new_index);
                }
            }
            None
        })
    }
}

impl<T> Grid<T>
where
    T: FromStr,
    T::Err: std::error::Error + Send + Sync + 'static,
{
    pub fn from_char_grid_str(s: &str) -> anyhow::Result<Self> {
        let mut n_cols = None;
        let mut data = Vec::new();
        for line in s.lines() {
            if let Some(n_cols) = n_cols {
                if n_cols != line.chars().count() {
                    return Err(anyhow!("Inconsistent number of columns in grid"));
                }
            } else {
                n_cols = Some(line.chars().count());
            }

            let mut buffer = [0u8; 4];
            let row = line
                .chars()
                .map(|c| c.encode_utf8(&mut buffer).parse())
                .collect::<Result<Vec<T>, _>>()?;
            data.push(row);
        }
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

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}
