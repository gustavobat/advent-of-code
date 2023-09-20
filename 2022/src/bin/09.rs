use anyhow::Result;
use std::{cmp::Ordering, collections::HashSet};

fn move_rope(rope: &mut Vec<(i32, i32)>, direction: &str) {
    let dir_offset: (i32, i32) = match direction {
        "R" => Some((0, 1)),
        "L" => Some((0, -1)),
        "U" => Some((1, 0)),
        "D" => Some((-1, 0)),
        _ => None,
    }
    .expect("Invalid direction");

    // Move head
    rope[0].0 += dir_offset.0;
    rope[0].1 += dir_offset.1;

    for i in 1..rope.len() {
        let diff_x = rope[i - 1].0 - rope[i].0;
        let diff_y = rope[i - 1].1 - rope[i].1;
        if diff_x.abs() > 1 || diff_y.abs() > 1 || diff_x.abs() + diff_y.abs() > 2 {
            let x_offset = match diff_x.cmp(&0) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };
            let y_offset = match diff_y.cmp(&0) {
                Ordering::Greater => 1,
                Ordering::Less => -1,
                Ordering::Equal => 0,
            };
            rope[i].0 += x_offset;
            rope[i].1 += y_offset;
        }
    }
}

fn compute_visited_positions(movements: &Vec<String>, n_knots: usize) -> usize {
    let mut visited_positions = HashSet::<(i32, i32)>::new();

    let mut rope: Vec<(i32, i32)> = vec![(0, 0); n_knots];
    visited_positions.insert(rope[1]);

    for line in movements {
        let (direction, n_steps_str) = line.split_once(' ').unwrap();
        let n_steps = n_steps_str.parse::<u32>().unwrap();
        (0..n_steps).for_each(|_| {
            move_rope(&mut rope, direction);
            visited_positions.insert(*rope.last().unwrap());
        });
    }
    visited_positions.len()
}

fn main() -> Result<()> {
    let movements = std::fs::read_to_string("./data/09.input")?
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    println!("Part 1: {}", compute_visited_positions(&movements, 2));
    println!("Part 2: {}", compute_visited_positions(&movements, 10));

    Ok(())
}

#[cfg(test)]
mod day09 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let movements = std::fs::read_to_string("./data/09.test1")?
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        assert_eq!(compute_visited_positions(&movements, 2), 13);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let movements = std::fs::read_to_string("./data/09.test2")?
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        assert_eq!(compute_visited_positions(&movements, 10), 36);
        Ok(())
    }
}
