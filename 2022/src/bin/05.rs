use anyhow::Result;
use std::collections::VecDeque;

#[derive(Debug)]
struct Move {
    from: usize,
    to: usize,
    n: usize,
}

impl std::str::FromStr for Move {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let re = regex::Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        if let Some(captures) = re.captures(s) {
            let n = captures[1].parse::<usize>()?;
            let from = captures[2].parse::<usize>()? - 1;
            let to = captures[3].parse::<usize>()? - 1;
            Ok(Self { from, to, n })
        } else {
            Err(anyhow::format_err!("Error parsing move"))
        }
    }
}

#[derive(Debug, Clone)]
struct CrateState {
    crates: Vec<VecDeque<char>>,
}

impl CrateState {
    fn move_one_crate_at_a_time(&mut self, move_: &Move) {
        for _ in 0..move_.n {
            let c = self.crates[move_.from].pop_back().unwrap();
            self.crates[move_.to].push_back(c);
        }
    }

    fn move_multiples_crates_at_once(&mut self, move_: &Move) {
        for i in 0..move_.n {
            let c = self.crates[move_.from][self.crates[move_.from].len() - move_.n + i];
            self.crates[move_.to].push_back(c);
        }
        for _ in 0..move_.n {
            self.crates[move_.from].pop_back();
        }
    }

    fn get_top_crates(&self) -> String {
        let mut top_crates = String::new();
        for stack in &self.crates {
            top_crates.push(stack[stack.len() - 1]);
        }
        top_crates
    }
}

impl std::str::FromStr for CrateState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut crates = Vec::new();
        for line in s.lines() {
            for (i, c) in line.chars().enumerate() {
                if i % 4 == 1 && c.is_alphabetic() {
                    if crates.len() <= i / 4 {
                        crates.resize(i / 4 + 1, VecDeque::new());
                    }
                    crates[i / 4].push_front(c);
                }
            }
        }
        Ok(Self { crates })
    }
}

fn solve_part1(crate_state: &mut CrateState, moves: &[Move]) -> String {
    for m in moves {
        crate_state.move_one_crate_at_a_time(m);
    }
    crate_state.get_top_crates()
}

fn solve_part2(crate_state: &mut CrateState, moves: &[Move]) -> String {
    for m in moves {
        crate_state.move_multiples_crates_at_once(m);
    }
    crate_state.get_top_crates()
}

fn get_input(path: &str) -> Result<(CrateState, Vec<Move>)> {
    let input: String = std::fs::read_to_string(path)?;
    let (crate_state, moves) = input.split_once("\n\n").unwrap();
    let crate_state = crate_state.parse::<CrateState>()?;
    let moves = moves
        .lines()
        .filter_map(|line| line.parse::<Move>().ok())
        .collect::<Vec<_>>();
    Ok((crate_state, moves))
}

fn main() -> Result<()> {
    let (mut initial_state, moves) = get_input("data/05.input")?;

    let part1 = solve_part1(&mut initial_state.clone(), &moves);
    let part2 = solve_part2(&mut initial_state, &moves);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod day05 {
    use super::*;
    #[test]
    fn part1() -> Result<()> {
        let (mut crate_state, moves) = get_input("data/05.test")?;
        let part1 = solve_part1(&mut crate_state, &moves);
        assert_eq!(part1, "CMZ");
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let (mut crate_state, moves) = get_input("data/05.test")?;
        let part2 = solve_part2(&mut crate_state, &moves);
        assert_eq!(part2, "MCD");
        Ok(())
    }
}
