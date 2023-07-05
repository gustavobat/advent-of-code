use utils::parse_each_split;

#[derive(Clone, Copy, Debug, Default)]
enum Direction {
    #[default]
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn turn_right(&mut self) {
        *self = match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(&mut self) {
        *self = match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Rotation {
    Left,
    Right,
}

#[derive(Clone, Copy)]
struct Step {
    rotation: Rotation,
    distance: u32,
}

impl std::str::FromStr for Step {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (rotation, distance) = s.split_at(1);
        let rotation = match rotation {
            "L" => Rotation::Left,
            "R" => Rotation::Right,
            _ => panic!("Unknown rotation"),
        };
        let distance = distance.parse::<u32>()?;
        Ok(Step { rotation, distance })
    }
}

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
struct Position(i32, i32);

impl Position {
    pub fn distance(&self) -> u32 {
        (self.0.abs() + self.1.abs()) as u32
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Gps {
    position: Position,
    direction: Direction,
}

fn find_destination<'a, I>(steps: I) -> Position
where
    I: IntoIterator<Item = &'a Step>,
{
    let mut gps = Gps::default();
    for step in steps {
        match step.rotation {
            Rotation::Left => gps.direction.turn_left(),
            Rotation::Right => gps.direction.turn_right(),
        }
        match gps.direction {
            Direction::North => gps.position.1 += step.distance as i32,
            Direction::South => gps.position.1 -= step.distance as i32,
            Direction::East => gps.position.0 += step.distance as i32,
            Direction::West => gps.position.0 -= step.distance as i32,
        }
    }
    gps.position
}

fn find_first_repeated<'a, I>(steps: I) -> Option<Position>
where
    I: IntoIterator<Item = &'a Step>,
{
    let mut position = Position::default();
    let mut visited = std::collections::HashSet::new();
    visited.insert(position);
    let mut gps = Gps::default();
    for step in steps {
        match step.rotation {
            Rotation::Left => gps.direction.turn_left(),
            Rotation::Right => gps.direction.turn_right(),
        }
        for _ in 0..step.distance {
            match gps.direction {
                Direction::North => position.1 += 1,
                Direction::South => position.1 -= 1,
                Direction::East => position.0 += 1,
                Direction::West => position.0 -= 1,
            }
            if !visited.insert(position) {
                return Some(position);
            }
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let steps: Vec<Step> = parse_each_split("data/01.input", ", ")?;

    let hq_location_1 = find_destination(&steps);
    println!("Part 1: {}", hq_location_1.distance());

    let hq_location_2 = find_first_repeated(&steps).unwrap();
    println!("Part 2: {}", hq_location_2.distance());

    Ok(())
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn part1() -> anyhow::Result<()> {
        let steps: Vec<Step> = parse_each_split("data/01-1.test", ", ")?;
        let hq_location = find_destination(&steps);
        assert_eq!(hq_location.distance(), 12);
        Ok(())
    }

    #[test]
    fn part2() -> anyhow::Result<()> {
        let steps: Vec<Step> = parse_each_split("data/01-2.test", ", ")?;
        let hq_location = find_first_repeated(&steps).unwrap();
        assert_eq!(hq_location.distance(), 4);
        Ok(())
    }
}
