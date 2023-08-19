use anyhow::Result;
use std::str::FromStr;
use utils::parse_each_line;

#[derive(Debug)]
struct Present {
    length: u32,
    width: u32,
    height: u32,
}

impl Present {
    fn required_wrapping_paper(&self) -> u32 {
        let mut sides = [
            self.length * self.width,
            self.width * self.height,
            self.height * self.length,
        ];
        sides.sort();
        3 * sides[0] + 2 * sides[1] + 2 * sides[2]
    }

    fn required_ribbon(&self) -> u32 {
        let mut sides = [self.length, self.width, self.height];
        sides.sort();

        let wrap = 2 * (sides[0] + sides[1]);
        let bow = self.length * self.width * self.height;
        wrap + bow
    }
}

impl FromStr for Present {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut dimensions = s.split('x').filter_map(|s| s.parse::<u32>().ok());
        let length = dimensions.next().ok_or(anyhow::anyhow!("No length"))?;
        let width = dimensions.next().ok_or(anyhow::anyhow!("No width"))?;
        let height = dimensions.next().ok_or(anyhow::anyhow!("No height"))?;

        Ok(Present {
            length,
            width,
            height,
        })
    }
}

fn main() -> Result<()> {
    let input: Vec<Present> = parse_each_line("data/02.input")?;

    let part1: u32 = input.iter().map(|p| p.required_wrapping_paper()).sum();
    let part2: u32 = input.iter().map(|p| p.required_ribbon()).sum();
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day01 {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn part1() -> Result<()> {
        assert_eq!(Present::from_str("2x3x4")?.required_wrapping_paper(), 58);
        assert_eq!(Present::from_str("1x1x10")?.required_wrapping_paper(), 43);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        assert_eq!(Present::from_str("2x3x4")?.required_ribbon(), 34);
        assert_eq!(Present::from_str("1x1x10")?.required_ribbon(), 14);
        Ok(())
    }
}
