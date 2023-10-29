use anyhow::Result;
use regex::Regex;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Operation {
    Addition(usize),
    Multiplication(usize),
    Square,
}

impl FromStr for Operation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"Operation: new = old ([+*]) (\w+)").unwrap();
        let captures = re.captures(s).unwrap();

        let op = captures
            .get(1)
            .ok_or(anyhow::anyhow!("Invalid operation"))?
            .as_str();

        let rhs = captures
            .get(2)
            .ok_or(anyhow::anyhow!("Invalid operation"))?
            .as_str();

        match op {
            "+" => Ok(Operation::Addition(rhs.parse::<usize>()?)),
            "*" => match rhs {
                "old" => Ok(Operation::Square),
                _ => Ok(Operation::Multiplication(rhs.parse::<usize>()?)),
            },
            _ => Err(anyhow::anyhow!("Invalid operation")),
        }
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: (usize, usize, usize),
}

impl FromStr for Monkey {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let _ = lines.next();
        let items = lines
            .next()
            .ok_or(anyhow::anyhow!("Invalid monkey"))?
            .split(": ")
            .nth(1)
            .ok_or(anyhow::anyhow!("Invalid monkey"))?
            .split(", ")
            .map(|s| s.parse::<usize>())
            .collect::<Result<Vec<usize>, _>>()?;

        let operation = lines
            .next()
            .ok_or(anyhow::anyhow!("Invalid monkey"))?
            .parse()?;

        let test = lines
            .map(|line| {
                line.split_whitespace()
                    .last()
                    .ok_or(anyhow::anyhow!("Invalid monkey"))
                    .and_then(|s| {
                        s.parse::<usize>()
                            .map_err(|_| anyhow::anyhow!("Invalid monkey"))
                    })
            })
            .collect::<Result<Vec<usize>, _>>()?;

        Ok(Monkey {
            items,
            operation,
            test: (test[0], test[1], test[2]),
        })
    }
}

fn process_round(
    monkeys: &mut Vec<Monkey>,
    items_inspected_per_monkey: &mut [usize],
    worry_relief: fn(usize, &[Monkey]) -> usize,
) {
    for i in 0..monkeys.len() {
        items_inspected_per_monkey[i] += monkeys[i].items.len();
        for item in monkeys[i].items.clone() {
            let mut worry_level = match monkeys[i].operation {
                Operation::Addition(rhs) => item + rhs,
                Operation::Multiplication(rhs) => item * rhs,
                Operation::Square => item * item,
            };
            worry_level = worry_relief(worry_level, monkeys);

            let (dividend, monkey_true, monkey_false) = monkeys[i].test;
            let target_monkey = if worry_level % dividend == 0 {
                monkey_true
            } else {
                monkey_false
            };
            monkeys[target_monkey].items.push(worry_level);
        }
        monkeys[i].items.clear();
    }
}

fn solve(
    mut monkeys: Vec<Monkey>,
    n_rounds: usize,
    worry_relief: fn(usize, &[Monkey]) -> usize,
) -> usize {
    let mut items_inspected_per_monkey: Vec<usize> = vec![0; monkeys.len()];
    (0..n_rounds).for_each(|_| {
        process_round(&mut monkeys, &mut items_inspected_per_monkey, worry_relief);
    });
    items_inspected_per_monkey.sort();
    items_inspected_per_monkey
        .iter()
        .rev()
        .take(2)
        .product::<usize>()
}

fn worry_relief_part_1(level: usize, _: &[Monkey]) -> usize {
    level / 3
}

fn worry_relief_part_2(level: usize, monkeys: &[Monkey]) -> usize {
    let common_denominator: usize = monkeys.iter().map(|m| m.test.0).product();
    level % common_denominator
}

fn main() -> Result<()> {
    let monkeys: Vec<Monkey> = utils::parse_each_split("./data/11.input", "\n\n")?;

    println!(
        "Part 1: {:?}",
        solve(monkeys.clone(), 20, worry_relief_part_1)
    );
    println!("Part 2: {:?}", solve(monkeys, 10_000, worry_relief_part_2));

    Ok(())
}

#[cfg(test)]
mod day11 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let monkeys: Vec<Monkey> = utils::parse_each_split("./data/11.test", "\n\n")?;
        assert_eq!(solve(monkeys, 20, worry_relief_part_1), 10_605);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let monkeys: Vec<Monkey> = utils::parse_each_split("./data/11.test", "\n\n")?;
        assert_eq!(solve(monkeys, 10_000, worry_relief_part_2), 2_713_310_158);
        Ok(())
    }
}
