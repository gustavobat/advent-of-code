use anyhow::Result;
use regex::Regex;
use std::collections::HashMap;

fn calc_dir_sizes(console_lines: Vec<String>) -> Result<HashMap<std::path::PathBuf, usize>> {
    let file_regex = Regex::new(r"(\d+) .*").unwrap();
    let cd_regex = Regex::new(r"\$ cd (.*)").unwrap();

    let mut current_path = std::path::PathBuf::new();
    let mut size_per_dir: HashMap<std::path::PathBuf, usize> = HashMap::new();

    for line in console_lines {
        if line == "$ cd .." {
            current_path.pop();
        } else if cd_regex.is_match(&line) {
            let dir = &cd_regex
                .captures(&line)
                .expect("Unable to get regex capture group")[1];
            current_path.push(dir);
            size_per_dir.insert(current_path.clone(), 0);
        } else if file_regex.is_match(&line) {
            let file_size = file_regex
                .captures(&line)
                .expect("Unable to get regex capture group")[1]
                .parse::<usize>()?;
            let mut parent_dirs = current_path.clone();

            let mut should_update_parent = true;
            while should_update_parent {
                *size_per_dir
                    .get_mut(&parent_dirs)
                    .expect("Error reading dir size") += file_size;
                should_update_parent = parent_dirs.pop();
            }
        }
    }
    Ok(size_per_dir)
}

fn solve_part1(size_per_dir: &HashMap<std::path::PathBuf, usize>) -> usize {
    size_per_dir
        .iter()
        .filter(|(_, size)| **size <= 100_000)
        .fold(0, |acc, (_, size)| acc + size)
}

fn solve_part2(size_per_dir: &HashMap<std::path::PathBuf, usize>) -> usize {
    let root_size = *size_per_dir
        .get(std::path::Path::new("/"))
        .expect("Error reading root dir size");
    let space_to_be_freed = 30_000_000 - (70_000_000 - root_size);
    *size_per_dir
        .iter()
        .filter(|(_, size)| **size > space_to_be_freed)
        .min_by_key(|(_, size)| **size)
        .unwrap()
        .1
}

fn main() -> Result<()> {
    let console_lines: Vec<String> = utils::parse_each_line("./data/07.input")?;
    let size_per_dir = calc_dir_sizes(console_lines)?;
    println!("Part 1: {}", solve_part1(&size_per_dir));
    println!("Part 2: {}", solve_part2(&size_per_dir));

    Ok(())
}

#[cfg(test)]
mod day07 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let console_lines: Vec<String> = utils::parse_each_line("./data/07.test")?;
        let size_per_dir = calc_dir_sizes(console_lines)?;
        assert_eq!(solve_part1(&size_per_dir), 95_437);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let console_lines: Vec<String> = utils::parse_each_line("./data/07.test")?;
        let size_per_dir = calc_dir_sizes(console_lines)?;
        assert_eq!(solve_part2(&size_per_dir), 24_933_642);
        Ok(())
    }
}
