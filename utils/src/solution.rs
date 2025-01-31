use std::fmt::Display;

pub struct Solution {
    pub part_one: String,
    pub part_two: String,
}

pub struct Solver {
    pub year: u16,
    pub day: u8,
    pub solver: fn(&str) -> anyhow::Result<Solution>,
}

impl Solver {
    pub const fn new(year: u16, day: u8, solver: fn(&str) -> anyhow::Result<Solution>) -> Self {
        Solver { year, day, solver }
    }
}

impl Display for Solver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Solver for year {} day {}", self.year, self.day)
    }
}

inventory::collect!(Solver);
