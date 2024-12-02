use anyhow::Result;
use std::collections::{HashMap, HashSet};

type Position = (usize, usize);

fn get_input(path: &std::path::Path) -> (Vec<Vec<u8>>, Position, Position) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    let hill: Vec<Vec<u8>> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .enumerate()
        .map(|(j, line)| {
            line.parse::<String>()
                .unwrap()
                .chars()
                .enumerate()
                .map(|(i, c)| match c {
                    'S' => {
                        start = (i, j);
                        0
                    }
                    'E' => {
                        end = (i, j);
                        26
                    }
                    _ => c as u8 - b'a' + 1,
                })
                .collect()
        })
        .collect();
    (hill, start, end)
}

fn solve_part1(hill: &[Vec<u8>], start: Position, end: Position) -> Result<usize> {
    let mut dist = HashMap::new();
    let mut unvisited = HashSet::new();

    for (j, _) in hill.iter().enumerate() {
        for (i, _) in hill[j].iter().enumerate() {
            dist.insert((i, j), 9999);
            unvisited.insert((i, j));
        }
    }
    dist.insert(start, 0);

    while !unvisited.is_empty() {
        let current: Position = *dist
            .iter()
            .filter(|item| unvisited.contains(item.0))
            .min_by_key(|k| k.1)
            .ok_or(anyhow::anyhow!("No path found"))?
            .0;
        let (x, y) = current;

        unvisited.remove(&current);
        let mut neighbours: Vec<Position> = Vec::with_capacity(4);
        if x > 0 {
            neighbours.push((x - 1, y))
        };
        if y > 0 {
            neighbours.push((x, y - 1))
        };
        if x < hill[0].len() - 1 {
            neighbours.push((x + 1, y))
        };
        if y < hill.len() - 1 {
            neighbours.push((x, y + 1))
        };

        let dist_to_neigh = dist[&current] + 1;
        for neigh in neighbours {
            let (neigh_x, neigh_y) = neigh;
            if hill[neigh_y][neigh_x] <= hill[y][x] + 1 && dist[&neigh] > dist_to_neigh {
                dist.insert((neigh_x, neigh_y), dist_to_neigh);
            }
        }
    }
    Ok(dist[&end])
}

fn solve_part2(hill: &[Vec<u8>], end: Position) -> Result<usize> {
    let mut length_of_possible_trails = HashSet::new();
    for (j, _) in hill.iter().enumerate() {
        // Optimization: we can iterate only first column, since
        // 'a' chars placed elsewhere are completely surrounded by 'c' chars.
        if hill[j][0] <= 1 {
            let start = (0, j);
            length_of_possible_trails.insert(solve_part1(hill, start, end)?);
        }
    }
    Ok(*length_of_possible_trails
        .iter()
        .min()
        .ok_or(anyhow::anyhow!("No path found"))?)
}

fn main() -> Result<()> {
    let (hill, start, end) = get_input(std::path::Path::new("./data/12.input"));

    println!("Part 1: {}", solve_part1(&hill, start, end)?);
    println!("Part 2: {}", solve_part2(&hill, end)?);

    Ok(())
}

#[cfg(test)]
mod day12 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let (hill, start, end) = get_input(std::path::Path::new("./data/12.test"));
        assert_eq!(solve_part1(&hill, start, end)?, 31);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let (hill, _start, end) = get_input(std::path::Path::new("./data/12.test"));
        assert_eq!(solve_part2(&hill, end)?, 29);
        Ok(())
    }
}
