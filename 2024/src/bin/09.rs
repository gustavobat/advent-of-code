use anyhow::{anyhow, Result};
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone)]
enum Partition {
    File { id: usize, size: usize },
    FreeSpace(usize),
}

impl Partition {
    fn size(&self) -> usize {
        match self {
            Partition::File { size, .. } => *size,
            Partition::FreeSpace(size) => *size,
        }
    }
}

#[derive(Debug, Clone)]
struct Disk {
    partitions: Vec<Partition>,
}

impl Disk {
    fn new(partitions: Vec<Partition>) -> Self {
        Self { partitions }
    }

    fn calculate_checksum(&self) -> usize {
        self.partitions
            .iter()
            .flat_map(|part| std::iter::repeat(part).take(part.size()))
            .enumerate()
            .fold(0, |checksum, (i, part)| {
                if let Partition::File { id, size: _ } = part {
                    checksum + *id * i
                } else {
                    checksum
                }
            })
    }

    fn free_spaces(&self) -> Vec<(usize, &Partition)> {
        self.partitions
            .iter()
            .enumerate()
            .filter(|(_, part)| matches!(part, Partition::FreeSpace(_)))
            .collect()
    }

    fn files(&self) -> Vec<(usize, &Partition)> {
        self.partitions
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, part)| matches!(part, Partition::File { .. }))
            .collect()
    }

    fn move_file(&mut self, file_pos: usize, free_space_pos: usize) -> Result<()> {
        let Partition::File {
            id: file_id,
            size: file_size,
        } = self.partitions[file_pos]
        else {
            return Err(anyhow!("Expected partition is not a file"));
        };
        let Partition::FreeSpace(free_space_size) = self.partitions[free_space_pos] else {
            return Err(anyhow!("Expected partition is not a free space"));
        };

        let moved_size = free_space_size.min(file_size);
        self.partitions[free_space_pos] = Partition::File {
            id: file_id,
            size: moved_size,
        };
        match free_space_size.cmp(&file_size) {
            Ordering::Less => {
                self.partitions[file_pos] = Partition::File {
                    id: file_id,
                    size: file_size - moved_size,
                };
                self.partitions
                    .insert(file_pos + 1, Partition::FreeSpace(moved_size));
            }
            Ordering::Equal => {
                self.partitions[file_pos] = Partition::FreeSpace(file_size);
            }
            Ordering::Greater => {
                self.partitions[file_pos] = Partition::FreeSpace(moved_size);
                self.partitions.insert(
                    free_space_pos + 1,
                    Partition::FreeSpace(free_space_size - moved_size),
                );
            }
        };
        Ok(())
    }

    fn compress(
        &mut self,
        move_condition: impl Fn((usize, &Partition), (usize, &Partition)) -> bool,
    ) -> Result<()> {
        let mut compressible = true;
        while compressible {
            compressible = false;
            let files = self.files();
            let free_spaces = self.free_spaces();
            for (file_pos, file) in files {
                if let Some((free_space_pos, _)) =
                    free_spaces.iter().find(|(free_space_pos, free_space)| {
                        move_condition((file_pos, file), (*free_space_pos, free_space))
                    })
                {
                    self.move_file(file_pos, *free_space_pos)?;
                    compressible = true;
                    break;
                }
            }
        }
        Ok(())
    }
}

fn solve_part_one(disk: &mut Disk) -> Result<usize> {
    disk.compress(|(file_pos, _), (free_space_pos, _)| file_pos > free_space_pos)?;
    Ok(disk.calculate_checksum())
}

fn solve_part_two(disk: &mut Disk) -> Result<usize> {
    disk.compress(|(file_pos, file), (free_space_pos, free_space)| {
        free_space.size() >= file.size() && file_pos > free_space_pos
    })?;
    Ok(disk.calculate_checksum())
}

fn parse_input(input: &str) -> Result<Disk> {
    let line = input.lines().next().ok_or(anyhow!("Missing line"))?;
    let disk: Vec<Partition> = line
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let size = c as usize - '0' as usize;
            if i % 2 == 0 {
                Partition::File { id: i / 2, size }
            } else {
                Partition::FreeSpace(size)
            }
        })
        .collect();
    Ok(Disk::new(disk))
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/09.input")?;
    let mut input = parse_input(&input)?;

    let part1 = solve_part_one(&mut input.clone())?;
    let part2 = solve_part_two(&mut input)?;

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day09 {
    use super::*;

    #[test]
    fn part1() {
        let input = std::fs::read_to_string("data/09.test").unwrap();
        let mut input = parse_input(&input).unwrap();
        let part1 = solve_part_one(&mut input).unwrap();
        assert_eq!(part1, 1928);
    }

    #[test]
    fn part2() {
        let input = std::fs::read_to_string("data/09.test").unwrap();
        let mut input = parse_input(&input).unwrap();
        let part2 = solve_part_two(&mut input).unwrap();
        assert_eq!(part2, 2858);
    }
}
