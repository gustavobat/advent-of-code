fn solve_part_1(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| match c {
        '(' => acc + 1,
        ')' => acc - 1,
        _ => acc,
    })
}

fn solve_part_2(input: &str) -> usize {
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => (),
        }
        if floor == -1 {
            return i + 1;
        }
    }
    0
}

fn main() -> anyhow::Result<()> {
    let input = std::fs::read_to_string("data/01.input")?;

    let part1 = solve_part_1(&input);
    let part2 = solve_part_2(&input);

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(solve_part_1("(())"), 0);
        assert_eq!(solve_part_1("()()"), 0);
        assert_eq!(solve_part_1("((("), 3);
        assert_eq!(solve_part_1("(()(()("), 3);
        assert_eq!(solve_part_1("))((((("), 3);
        assert_eq!(solve_part_1("())"), -1);
        assert_eq!(solve_part_1("))("), -1);
        assert_eq!(solve_part_1(")))"), -3);
        assert_eq!(solve_part_1(")())())"), -3);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part_2(")"), 1);
        assert_eq!(solve_part_2("()())"), 5);
    }
}
