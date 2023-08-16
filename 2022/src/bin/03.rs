use anyhow::Result;

fn item_priority(c: char) -> u32 {
    match c {
        x if x.is_lowercase() => 1 + x as u32 - 'a' as u32,
        x if x.is_uppercase() => 27 + x as u32 - 'A' as u32,
        _ => 0,
    }
}

fn priority_part1(rucksacks: &[String]) -> u32 {
    rucksacks.iter().fold(0, |mut acc, rucksack| {
        let container1: &str = &rucksack[0..rucksack.len() / 2];
        let container2: &str = &rucksack[rucksack.len() / 2..rucksack.len()];

        for item in container1.chars() {
            if container2.contains(item) {
                acc += item_priority(item);
                break;
            }
        }
        acc
    })
}

fn priority_part2(rucksacks: &[String]) -> u32 {
    rucksacks.chunks(3).fold(0, |mut acc, rucksacks| {
        let rucksack1: &str = &rucksacks[0];
        let rucksack2: &str = &rucksacks[1];
        let rucksack3: &str = &rucksacks[2];

        for item in rucksack1.chars() {
            if rucksack2.contains(item) && rucksack3.contains(item) {
                acc += item_priority(item);
                break;
            }
        }
        acc
    })
}

fn main() -> Result<()> {
    let rucksacks: Vec<String> = utils::parse_each_line("data/03.input")?;
    println!("Part 1: {}", priority_part1(&rucksacks));
    println!("Part 2: {}", priority_part2(&rucksacks));

    Ok(())
}

#[cfg(test)]
mod day03 {
    use super::*;
    #[test]
    fn item_priority_test() {
        assert_eq!(item_priority('a'), 1);
        assert_eq!(item_priority('z'), 26);
        assert_eq!(item_priority('A'), 27);
        assert_eq!(item_priority('Z'), 52);
    }

    #[test]
    fn part1() -> Result<()> {
        let rucksacks: Vec<String> = utils::parse_each_line("data/03.test")?;
        assert_eq!(priority_part1(&rucksacks), 157);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let rucksacks: Vec<String> = utils::parse_each_line("data/03.test")?;
        assert_eq!(priority_part2(&rucksacks), 70);
        Ok(())
    }
}
