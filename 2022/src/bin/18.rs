use anyhow::Result;
use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
use std::str::FromStr;
use utils::parse_each_line;

#[derive(Eq, PartialEq, Debug, Hash)]
struct LavaCube {
    coord: (i32, i32, i32),
}

impl FromStr for LavaCube {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut split = s.split(',');
        let coord: (i32, i32, i32) = (
            split
                .next()
                .ok_or(anyhow::anyhow!("Could not get next coord"))?
                .parse::<i32>()?,
            split
                .next()
                .ok_or(anyhow::anyhow!("Could not get next coord"))?
                .parse::<i32>()?,
            split
                .next()
                .ok_or(anyhow::anyhow!("Could not get next coord"))?
                .parse::<i32>()?,
        );
        Ok(LavaCube { coord })
    }
}

impl LavaCube {
    fn neighbours(&self) -> [(i32, i32, i32); 6] {
        let (x, y, z) = self.coord;
        [
            (x - 1, y, z),
            (x + 1, y, z),
            (x, y - 1, z),
            (x, y + 1, z),
            (x, y, z - 1),
            (x, y, z + 1),
        ]
    }
}

fn compute_surface_area(cubes: &HashSet<LavaCube>) -> usize {
    cubes
        .iter()
        .flat_map(|cube| cube.neighbours())
        .filter(|neigh| !cubes.contains(&LavaCube { coord: *neigh }))
        .count()
}

fn compute_internal_surface_area(cubes: &HashSet<LavaCube>) -> usize {
    // Create a layer of air around the lava block
    let min_x = cubes.iter().map(|cube| cube.coord.0).min().unwrap() - 1;
    let max_x = cubes.iter().map(|cube| cube.coord.0).max().unwrap() + 1;
    let min_y = cubes.iter().map(|cube| cube.coord.1).min().unwrap() - 1;
    let max_y = cubes.iter().map(|cube| cube.coord.1).max().unwrap() + 1;
    let min_z = cubes.iter().map(|cube| cube.coord.2).min().unwrap() - 1;
    let max_z = cubes.iter().map(|cube| cube.coord.2).max().unwrap() + 1;

    let mut air_pockets = HashMap::new();
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                let current_cube = LavaCube { coord: (x, y, z) };
                if !cubes.contains(&current_cube) {
                    air_pockets.insert(current_cube.coord, true);
                }
            }
        }
    }

    // BFS traversal to identify external air pockets
    let mut air_pocket_queue: VecDeque<LavaCube> = VecDeque::new();
    air_pocket_queue.push_back(LavaCube {
        coord: (min_x, min_y, min_z),
    });
    while let Some(cube) = air_pocket_queue.pop_front() {
        if air_pockets.get(&cube.coord) == Some(&true) {
            air_pockets.insert(cube.coord, false);
            for neighbour_coord in cube.neighbours().iter() {
                if air_pockets.contains_key(neighbour_coord) && air_pockets[neighbour_coord] {
                    air_pocket_queue.push_back(LavaCube {
                        coord: *neighbour_coord,
                    });
                }
            }
        }
    }

    // Make a HashSet with cubes of trapped air and compute their surface area
    let trapped_air_pockets: HashSet<LavaCube> = air_pockets
        .keys()
        .cloned()
        .filter(|cube| air_pockets[cube])
        .map(|coord| LavaCube { coord })
        .collect();

    compute_surface_area(&trapped_air_pockets)
}

fn main() -> Result<()> {
    let cubes: HashSet<LavaCube> = parse_each_line("./data/18.input")?;

    let surface_area = compute_surface_area(&cubes);
    println!("Part 1: {surface_area}");

    let exterior_surface_area = surface_area - compute_internal_surface_area(&cubes);
    println!("Part 2: {exterior_surface_area}");

    Ok(())
}

#[cfg(test)]
mod day18 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let cubes: HashSet<LavaCube> = parse_each_line("./data/18.test")?;
        assert_eq!(compute_surface_area(&cubes), 64);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let cubes: HashSet<LavaCube> = parse_each_line("./data/18.test")?;
        let surface_area = compute_surface_area(&cubes);
        let internal_surface_area = compute_internal_surface_area(&cubes);
        let result = surface_area - internal_surface_area;
        assert_eq!(result, 58);
        Ok(())
    }
}
