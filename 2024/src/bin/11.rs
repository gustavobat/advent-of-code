use anyhow::Result;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum StoneUpdate {
    NewValue(usize),
    Split(usize, usize),
}

fn update_stone(stone: usize) -> StoneUpdate {
    match stone {
        0 => StoneUpdate::NewValue(1),
        n => {
            let n_str = n.to_string();
            if n_str.len() % 2 == 0 {
                let half = n_str.len() / 2;
                let (left, right) = n_str.split_at(half);
                let left = left.parse().unwrap();
                let right = right.parse().unwrap();
                StoneUpdate::Split(left, right)
            } else {
                StoneUpdate::NewValue(n * 2024)
            }
        }
    }
}

fn blink(stones: &mut HashMap<usize, usize>) {
    for (old_val, old_count) in stones.clone().iter() {
        let update = update_stone(*old_val);
        let count = *old_count;
        match update {
            StoneUpdate::NewValue(new_val) => {
                *stones.entry(new_val).or_insert(0) += count;
            }
            StoneUpdate::Split(left_val, right_val) => {
                *stones.entry(left_val).or_insert(0) += count;
                *stones.entry(right_val).or_insert(0) += count;
            }
        }
        if stones[old_val] == count {
            stones.remove(old_val);
        } else {
            *stones.get_mut(old_val).unwrap() -= count;
        }
    }
}

fn solve(stones: &mut HashMap<usize, usize>, max_blinks: usize) -> usize {
    for _ in 0..max_blinks {
        blink(stones);
    }
    stones.values().sum()
}

fn parse_input(input: &str) -> Result<HashMap<usize, usize>> {
    let mut map = HashMap::new();
    for num_str in input.split_whitespace() {
        let num = num_str.parse::<usize>()?;
        *map.entry(num).or_insert(0) += 1;
    }
    Ok(map)
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("data/11.input")?;
    let mut input = parse_input(&input)?;

    let part1 = solve(&mut input, 25);
    let part2 = solve(&mut input, 50);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day11 {
    use super::*;

    #[test]
    fn both() {
        let input = std::fs::read_to_string("data/11.test").unwrap();
        let mut input = parse_input(&input).unwrap();
        let part1 = solve(&mut input, 25);
        assert_eq!(part1, 55312);
    }
}
