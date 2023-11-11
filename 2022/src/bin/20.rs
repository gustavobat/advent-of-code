use anyhow::Result;
use utils::parse_each_line;

fn mix(orig_nums: &[i64], mixed_pos: &mut Vec<usize>) {
    for (orig_pos, &num) in orig_nums.iter().enumerate() {
        let old_pos = mixed_pos.iter().position(|&pos| pos == orig_pos).unwrap();
        mixed_pos.remove(old_pos);
        let new_pos = (old_pos as i64 + num).rem_euclid(mixed_pos.len() as i64) as usize;
        mixed_pos.insert(new_pos, orig_pos);
    }
}

fn sum_grove_coords(orig_nums: &[i64], mixed_pos: &mut Vec<usize>) -> i64 {
    let zero_pos = orig_nums.iter().position(|&num| num == 0).unwrap();
    let mixed_zero_pos = mixed_pos.iter().position(|&pos| pos == zero_pos).unwrap();

    [1000, 2000, 3000]
        .iter()
        .map(|offset| {
            let mixed_offset_pos = (mixed_zero_pos + offset) % mixed_pos.len();
            let orig_offset_pos = mixed_pos[mixed_offset_pos];
            orig_nums[orig_offset_pos]
        })
        .sum()
}

fn solve_part1(orig_nums: &[i64]) -> i64 {
    let mut mixed_pos: Vec<_> = (0..orig_nums.len()).collect();
    mix(orig_nums, &mut mixed_pos);
    sum_grove_coords(orig_nums, &mut mixed_pos)
}

fn solve_part2(orig_nums: &[i64]) -> i64 {
    let decryption_key = 811_589_153;
    let nums: Vec<_> = orig_nums.iter().map(|num| num * decryption_key).collect();
    let mut mixed_pos: Vec<_> = (0..nums.len()).collect();
    for _ in 0..10 {
        mix(&nums, &mut mixed_pos);
    }
    sum_grove_coords(&nums, &mut mixed_pos)
}

fn main() -> Result<()> {
    let orig_nums: Vec<i64> = parse_each_line("./data/20.input")?;

    println!("Part 1: {}", solve_part1(&orig_nums));
    println!("Part 2: {}", solve_part2(&orig_nums));

    Ok(())
}

#[cfg(test)]
mod day20 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let orig_nums: Vec<i64> = parse_each_line("./data/20.test")?;
        assert_eq!(solve_part1(&orig_nums), 3);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let orig_nums: Vec<i64> = parse_each_line("./data/20.test")?;
        assert_eq!(solve_part2(&orig_nums), 1623178306);
        Ok(())
    }
}
