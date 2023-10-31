use anyhow::Result;
use std::cmp::max;
use std::collections::HashSet;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Position {
    row: i64,
    col: i64,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Measurement {
    sensor: Position,
    beacon: Position,
}

impl FromStr for Measurement {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.split(|c| " =,:".contains(c));
        let sensor = {
            let col = iter
                .nth(3)
                .ok_or(anyhow::anyhow!("Failed to parse sensor column"))?
                .parse()?;

            let row = iter
                .nth(2)
                .ok_or(anyhow::anyhow!("Failed to parse sensor row"))?
                .parse()?;
            Position { col, row }
        };

        let beacon = {
            let col = iter
                .nth(6)
                .ok_or(anyhow::anyhow!("Failed to parse beacon column"))?
                .parse()?;

            let row = iter
                .nth(2)
                .ok_or(anyhow::anyhow!("Failed to parse beacon row"))?
                .parse()?;
            Position { col, row }
        };
        Ok(Measurement { sensor, beacon })
    }
}

fn manhattan_dist(a: &Position, b: &Position) -> i64 {
    (a.row - b.row).abs() + (a.col - b.col).abs()
}

fn impossible_beacon_positions(measurements: &[Measurement], row: i64) -> Range<i64> {
    let mut ranges = Vec::new();

    for m in measurements.iter() {
        let distance_to_beacon = manhattan_dist(&m.sensor, &m.beacon);
        let distance_to_row = (m.sensor.row - row).abs();
        if distance_to_row <= distance_to_beacon {
            let dist_left = distance_to_beacon - distance_to_row;
            let left = m.sensor.col - dist_left;
            let right = m.sensor.col + dist_left;
            ranges.push(left..(right + 1));
        }
    }

    ranges.sort_by_key(|r| (r.start, r.end));

    let mut merged_range: Range<i64> = ranges[0].clone();
    for r in ranges.iter().skip(1) {
        if r.start <= merged_range.end {
            merged_range.end = max(r.end, merged_range.end);
        }
    }
    merged_range
}

fn solve_part1(measurements: &[Measurement], target_y: i64) -> i64 {
    let positions = impossible_beacon_positions(measurements, target_y);
    let mut answer = positions.end - positions.start;

    let mut beacons_in_target_row = HashSet::new();
    for measurement in measurements.iter() {
        if measurement.beacon.row == target_y {
            beacons_in_target_row.insert(measurement.beacon.col);
        }
    }
    answer -= beacons_in_target_row.len() as i64;
    answer
}

fn solve_part2(measurements: &[Measurement], max_pos: i64) -> Option<i64> {
    for target_y in 0..=max_pos {
        let positions = impossible_beacon_positions(measurements, target_y);
        if positions.end <= max_pos {
            return Some((positions.end) * 4_000_000i64 + target_y);
        }
    }
    None
}

fn main() -> Result<()> {
    let measurements: Vec<_> = utils::parse_each_line("./data/15.input")?;

    let part1 = solve_part1(&measurements, 2_000_000);
    println!("Part 1: {part1}");

    let part2 =
        solve_part2(&measurements, 4_000_000).ok_or(anyhow::anyhow!("No solution for part 2"))?;
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day15 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let measurements: Vec<_> = utils::parse_each_line("./data/15.test")?;
        assert_eq!(solve_part1(&measurements, 10), 26);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let measurements: Vec<_> = utils::parse_each_line("./data/15.test")?;
        assert_eq!(solve_part2(&measurements, 20), Some(56_000_011));
        Ok(())
    }
}
