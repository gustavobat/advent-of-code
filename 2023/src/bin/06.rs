use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Race {
    time: i64,
    distance: i64,
}

fn parse_races(input: &str, concat: bool) -> Result<Vec<Race>> {
    let mut lines = input.lines();
    let time_line = lines.next().ok_or_else(|| anyhow!("Missing time line"))?;
    let dist_line = lines
        .next()
        .ok_or_else(|| anyhow!("Missing distance line"))?;

    let time_strs = time_line.split_whitespace().skip(1).collect::<Vec<&str>>();
    let dist_strs = dist_line.split_whitespace().skip(1).collect::<Vec<&str>>();

    if concat {
        let time = time_strs
            .join("")
            .parse::<i64>()
            .map_err(|_| anyhow!("Failed to parse time"))?;
        let distance = dist_strs
            .join("")
            .parse::<i64>()
            .map_err(|_| anyhow!("Failed to parse distance"))?;
        Ok(vec![Race { time, distance }])
    } else {
        let times: Vec<i64> = time_strs.iter().filter_map(|s| s.parse().ok()).collect();
        let distances: Vec<i64> = dist_strs.iter().filter_map(|s| s.parse().ok()).collect();

        if times.len() != distances.len() {
            return Err(anyhow!("Number of times and distances must be equal"));
        }

        Ok(times
            .into_iter()
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect())
    }
}

fn count_records_beaten(a: f64, b: f64, c: f64) -> i64 {
    let delta = b.powi(2) - 4.0 * a * c;
    let mut x1 = (-b - delta.sqrt()) / (2.0 * a);
    let mut x2 = (-b + delta.sqrt()) / (2.0 * a);

    if x1 > x2 {
        std::mem::swap(&mut x1, &mut x2);
    }

    let start = (x1 + 1.0).floor() as i64;
    let end = (x2 - 1.0).ceil() as i64;

    end - start + 1
}

fn solve_part1(input: &str) -> Result<i64> {
    let races = parse_races(input, false)?;
    Ok(races
        .iter()
        .map(|race| count_records_beaten(-1.0, race.time as f64, -1.0 * race.distance as f64))
        .product())
}

fn solve_part2(input: &str) -> Result<i64> {
    let race = parse_races(input, true)?;
    Ok(count_records_beaten(
        -1.0,
        race[0].time as f64,
        -1.0 * race[0].distance as f64,
    ))
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("./data/06.input")?;
    println!("Part 1: {}", solve_part1(&input)?);
    println!("Part 2: {}", solve_part2(&input)?);

    Ok(())
}

#[cfg(test)]
mod day06 {
    use super::*;
    #[test]
    fn part1() -> Result<()> {
        let input = std::fs::read_to_string("./data/06.test")?;
        let part1 = solve_part1(&input)?;
        assert_eq!(part1, 288);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let input = std::fs::read_to_string("./data/06.test")?;
        let part2 = solve_part2(&input)?;
        assert_eq!(part2, 71503);
        Ok(())
    }
}
