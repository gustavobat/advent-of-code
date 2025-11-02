use std::collections::HashMap;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 11, solve_all)
}

#[derive(Copy, Clone, Debug)]
enum StoneUpdate {
    NewValue(usize),
    Split(usize, usize),
}

fn update_stone(stone: usize) -> StoneUpdate {
    match stone {
        0 => StoneUpdate::NewValue(1),
        n => {
            let n_digits = n.ilog10() + 1;
            if n_digits % 2 == 0 {
                let half = n_digits / 2;
                let left = n / 10_usize.pow(half);
                let right = n % 10_usize.pow(half);
                StoneUpdate::Split(left, right)
            } else {
                StoneUpdate::NewValue(n * 2024)
            }
        }
    }
}

fn blink_once(stones: &mut HashMap<usize, usize>) {
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
            stones.remove(old_val); // TODO remove
        } else {
            *stones.get_mut(old_val).unwrap() -= count;
        }
    }
}

fn blink_n(stones: &mut HashMap<usize, usize>, n_blinks: usize) -> usize {
    for _ in 0..n_blinks {
        blink_once(stones);
    }
    stones.values().sum()
}

fn parse_input(input: &str) -> anyhow::Result<HashMap<usize, usize>> {
    let mut map = HashMap::new();
    for num_str in input.split_whitespace() {
        let num = num_str.parse::<usize>()?;
        *map.entry(num).or_insert(0) += 1;
    }
    Ok(map)
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let mut input = parse_input(input)?;
    let part_one = blink_n(&mut input, 25).to_string();
    let part_two = blink_n(&mut input, 50).to_string();

    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    use super::blink_n;
    use super::parse_input;
    use utils::load_test_input;

    #[test]
    fn both_parts() {
        let input = load_test_input!();
        let mut stones = parse_input(&input).unwrap();
        let solution = blink_n(&mut stones, 25);
        assert_eq!(solution, 55312);
    }
}
