use utils::solution::Solution;

use anyhow::Result;
use anyhow::anyhow;
use std::cmp::Ordering;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 9, solve_all)
}

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
struct Disk(Vec<Partition>);

impl Disk {
    fn new(partitions: Vec<Partition>) -> Self {
        Self(partitions)
    }

    fn calculate_checksum(&self) -> usize {
        self.0
            .iter()
            .flat_map(|part| std::iter::repeat_n(part, part.size()))
            .enumerate()
            .fold(0, |checksum, (i, part)| {
                if let Partition::File { id, .. } = part {
                    checksum + *id * i
                } else {
                    checksum
                }
            })
    }

    fn free_spaces(&self) -> Vec<(usize, &Partition)> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, part)| matches!(part, Partition::FreeSpace(_)))
            .collect()
    }

    fn files(&self) -> Vec<(usize, &Partition)> {
        self.0
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, part)| matches!(part, Partition::File { .. }))
            .collect()
    }

    fn move_file(&mut self, file_pos: usize, free_space_pos: usize) -> Result<()> {
        let Disk(partitions) = self;
        let Partition::File {
            id: file_id,
            size: file_size,
        } = partitions[file_pos]
        else {
            return Err(anyhow!("Expected partition is not a file"));
        };
        let Partition::FreeSpace(free_space_size) = partitions[free_space_pos] else {
            return Err(anyhow!("Expected partition is not a free space"));
        };

        let moved_size = free_space_size.min(file_size);
        partitions[free_space_pos] = Partition::File {
            id: file_id,
            size: moved_size,
        };
        match free_space_size.cmp(&file_size) {
            Ordering::Less => {
                partitions[file_pos] = Partition::File {
                    id: file_id,
                    size: file_size - moved_size,
                };
                partitions.insert(file_pos + 1, Partition::FreeSpace(moved_size));
            }
            Ordering::Equal => {
                partitions[file_pos] = Partition::FreeSpace(file_size);
            }
            Ordering::Greater => {
                partitions[file_pos] = Partition::FreeSpace(moved_size);
                partitions.insert(
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
            let free_spaces = self.free_spaces();
            for (file_pos, file) in self.files() {
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

fn solve_all(input: &str) -> Result<Solution> {
    let mut disk = parse_input(input)?;
    let part_one = solve_part_one(&mut disk.clone())?.to_string();
    let part_two = solve_part_two(&mut disk)?.to_string();

    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::solve_part_one;
    use super::solve_part_two;
    use utils::load_test_input;

    #[test]
    fn part_one() {
        let input = load_test_input!();
        let mut disk = parse_input(&input).unwrap();
        let solution = solve_part_one(&mut disk).unwrap();
        assert_eq!(solution, 1928);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let mut disk = parse_input(&input).unwrap();
        let solution = solve_part_two(&mut disk).unwrap();
        assert_eq!(solution, 2858);
    }
}
