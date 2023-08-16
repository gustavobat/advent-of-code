use anyhow::Result;

fn read_input(path: &str) -> Result<Vec<u32>> {
    let mut calories_per_elf = std::fs::read_to_string(path)?
        .split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse::<u32>().expect("Error parsing input to u32"))
                .sum()
        })
        .collect::<Vec<u32>>();

    calories_per_elf.sort_by(|a, b| b.partial_cmp(a).unwrap());
    Ok(calories_per_elf)
}

fn sum_first_n_elements(vec: &[u32], n: usize) -> u32 {
    vec.iter().take(n).sum()
}

fn main() -> Result<()> {
    let input = read_input("./data/01.input")?;
    println!("Part 1: {:?}", sum_first_n_elements(&input, 1));
    println!("Part 2: {:?}", sum_first_n_elements(&input, 3));

    Ok(())
}

#[cfg(test)]
mod day01 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let test_input = read_input("./data/01.test")?;
        assert_eq!(sum_first_n_elements(&test_input, 1), 24000);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let test_input = read_input("./data/01.test")?;
        assert_eq!(sum_first_n_elements(&test_input, 3), 45000);
        Ok(())
    }
}
