use anyhow::Result;
use std::str::FromStr;

/// Parse a file into a generic container C, which implements FromIterator<T>,
/// by parsing each substring yielded by the split
pub fn parse_each_split<T, C>(path: &str, pattern: &str) -> Result<C>
where
    T: FromStr,
    C: FromIterator<T>,
{
    Ok(std::fs::read_to_string(path)?
        .trim_end()
        .split(pattern)
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

/// Parse a file into a generic container C, which implements FromIterator<T>,
/// by parsing each line into a type `T` object.
pub fn parse_each_line<T, C>(path: &str) -> Result<C>
where
    T: FromStr,
    C: FromIterator<T>,
{
    parse_each_split(path, "\n")
}