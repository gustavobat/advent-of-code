use anyhow::anyhow;
use clap::Parser;
use clap::Subcommand;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Build and run a solution
    Run {
        /// Year of the puzzle (e.g., 2023)
        #[arg(short, long, value_parser = Year::from_str)]
        year: Year,

        /// Day of the puzzle (1-25)
        #[arg(short, long, value_parser = Day::from_str)]
        day: Day,

        /// Optional input file path to pass to the solution
        #[arg(short, long)]
        input: Option<PathBuf>,
    },

    /// Download puzzle input from adventofcode.com
    GetInput {
        /// Year of the puzzle (e.g., 2023)
        #[arg(short, long, value_parser = Year::from_str)]
        year: Year,

        /// Day of the puzzle (1-25)
        #[arg(short, long, value_parser = Day::from_str)]
        day: Day,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Year(u16);

impl Year {
    pub fn value(&self) -> u16 {
        self.0
    }
}

impl FromStr for Year {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val: u16 = s
            .parse()
            .map_err(|_| anyhow!("'{}' is not a valid u16 value", s))?;
        if (2015..=2025).contains(&val) {
            return Ok(Year(val));
        }
        anyhow::bail!("Year out of range: {val}")
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Day(u8);

impl FromStr for Day {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw: u8 = s
            .parse()
            .map_err(|_| anyhow!("'{}' is not a valid u8 value", s))?;
        if (1..=25).contains(&raw) {
            return Ok(Day(raw));
        }
        anyhow::bail!("Day out of range: {raw}")
    }
}

impl Day {
    pub fn value(&self) -> u8 {
        self.0
    }
}
