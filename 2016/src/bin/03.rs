#[derive(Debug)]
struct Triangle([u32; 3]);

impl Triangle {
    fn new(sides: [u32; 3]) -> Self {
        let mut sides = sides;
        sides.sort();
        Self(sides)
    }

    fn is_valid(&self) -> bool {
        self.0[0] + self.0[1] > self.0[2]
    }
}

fn count_part_1(input: &[[u32; 3]]) -> usize {
    input
        .iter()
        .filter(|sides| Triangle::new(**sides).is_valid())
        .count()
}

fn count_part_2(input: &[[u32; 3]]) -> usize {
    let mut count = 0;
    for i in 0..input.len() / 3 {
        let mut sides = [[0; 3]; 3];
        for j in 0..3 {
            sides[0][j] = input[i * 3][j];
            sides[1][j] = input[i * 3 + 1][j];
            sides[2][j] = input[i * 3 + 2][j];
        }
        for j in 0..3 {
            if Triangle::new([sides[0][j], sides[1][j], sides[2][j]]).is_valid() {
                count += 1;
            }
        }
    }
    count
}

fn main() -> anyhow::Result<()> {
    let input: Vec<[u32; 3]> = std::fs::read_to_string("data/03.input")?
        .lines()
        .map(|line| {
            let mut sides = [0; 3];
            for (i, side) in line.split_whitespace().enumerate() {
                sides[i] = side.parse().unwrap();
            }
            sides
        })
        .collect();

    let part_1 = count_part_1(&input);
    println!("Part 1: {part_1}");

    let part_2 = count_part_2(&input);
    println!("Part 2: {part_2}");

    Ok(())
}

#[cfg(test)]
mod day03 {
    #[test]
    fn part1() -> anyhow::Result<()> {
        Ok(())
    }

    #[test]
    fn part2() -> anyhow::Result<()> {
        Ok(())
    }
}
