use anyhow::Result;
use std::{path::Path, str::FromStr};

/// Parse a file into a generic container C, which implements FromIterator<T>,
/// by parsing each substring yielded by the split
pub fn parse_each_split<P, T, C>(path: P, pattern: &str) -> Result<C>
where
    P: AsRef<Path>,
    T: FromStr,
    C: FromIterator<T>,
{
    Ok(std::fs::read_to_string(path)?
        .trim_end()
        .split(pattern)
        .filter(|str| !str.is_empty())
        .filter_map(|str| str.parse::<T>().ok())
        .collect())
}

/// Parse a file into a generic container C, which implements FromIterator<T>,
/// by parsing each line into a type `T` object.
pub fn parse_each_line<P, T, C>(path: P) -> Result<C>
where
    P: AsRef<Path>,
    T: FromStr,
    C: FromIterator<T>,
{
    parse_each_split(path, "\n")
}
