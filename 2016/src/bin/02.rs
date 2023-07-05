use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct KeyPadLayout(Vec<Vec<Option<char>>>);

impl KeyPadLayout {
    fn key_position(&self, key: char) -> Option<(usize, usize)> {
        self.0.iter().enumerate().find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, k)| if *k == Some(key) { Some((x, y)) } else { None })
        })
    }

    fn at(&self, pos: (usize, usize)) -> Option<char> {
        self.0[pos.0][pos.1]
    }

    fn rows(&self) -> usize {
        self.0.len()
    }

    fn cols(&self) -> usize {
        self.0[0].len()
    }
}

impl std::str::FromStr for KeyPadLayout {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut layout = Vec::new();
        for line in s.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                row.push(match c {
                    '0'..='9' | 'A'..='D' => Some(c),
                    _ => None,
                });
            }
            layout.push(row);
        }
        Ok(Self(layout))
    }
}

#[derive(Clone, Debug)]
struct Keypad {
    x: usize,
    y: usize,
    layout: KeyPadLayout,
}

impl Keypad {
    fn new(layout: KeyPadLayout) -> Self {
        let start_key = '5';
        let (x, y) = layout.key_position(start_key).unwrap();
        Self { x, y, layout }
    }
}

impl std::ops::AddAssign<Direction> for Keypad {
    fn add_assign(&mut self, rhs: Direction) {
        let x = self.x;
        let y = self.y;

        match rhs {
            Direction::Up => {
                if y > 0 && self.layout.at((y - 1, x)).is_some() {
                    self.y -= 1;
                }
            }
            Direction::Down => {
                if y < self.layout.rows() - 1 && self.layout.at((y + 1, x)).is_some() {
                    self.y += 1;
                }
            }
            Direction::Left => {
                if x > 0 && self.layout.at((y, x - 1)).is_some() {
                    self.x -= 1;
                }
            }
            Direction::Right => {
                if x < self.layout.cols() - 1 && self.layout.at((y, x + 1)).is_some() {
                    self.x += 1;
                }
            }
        };
    }
}

fn solve_code(keypad: &mut Keypad, instructions: &[Vec<Direction>]) -> String {
    instructions.iter().fold(String::new(), |mut code, line| {
        for direction in line {
            *keypad += *direction;
        }
        code.push(keypad.layout.at((keypad.y, keypad.x)).unwrap());
        code
    })
}

fn parse_instructions(path: &str) -> anyhow::Result<Vec<Vec<Direction>>> {
    let instructions = std::fs::read_to_string(path)?
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Unknown direction"),
                })
                .collect()
        })
        .collect();
    Ok(instructions)
}

fn main() -> anyhow::Result<()> {
    let layout1 = KeyPadLayout::from_str("123\n456\n789\n")?;
    let layout2 = KeyPadLayout::from_str("  1  \n 234 \n56789\n ABC \n  D  \n")?;
    let instructions = parse_instructions("data/02.input")?;

    let mut keypad1 = Keypad::new(layout1);
    let mut keypad2 = Keypad::new(layout2);

    println!("Part 1: {}", solve_code(&mut keypad1, &instructions));
    println!("Part 2: {}", solve_code(&mut keypad2, &instructions));

    Ok(())
}

#[cfg(test)]
mod day02 {
    use super::*;

    #[test]
    fn part1() -> anyhow::Result<()> {
        let layout = KeyPadLayout::from_str("123\n456\n789\n")?;
        let instructions = parse_instructions("data/02.test")?;

        let mut keypad = Keypad::new(layout);
        assert_eq!(solve_code(&mut keypad, &instructions), "1985");
        Ok(())
    }

    #[test]
    fn part2() -> anyhow::Result<()> {
        let layout = KeyPadLayout::from_str("  1  \n 234 \n56789\n ABC \n  D  \n")?;
        let instructions = parse_instructions("data/02.test")?;
        let mut keypad = Keypad::new(layout);
        assert_eq!(solve_code(&mut keypad, &instructions), "5DB3");
        Ok(())
    }
}
