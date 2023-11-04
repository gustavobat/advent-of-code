use anyhow::Result;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Clone, Debug)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

#[derive(Clone, Debug)]
enum MonkeyJob {
    Number(i64),
    Op(Operation, String, String),
}

impl FromStr for MonkeyJob {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(num) => Ok(MonkeyJob::Number(num)),
            Err(_) => {
                let mut split = s.split_whitespace();
                let monkey_left = split.next().unwrap().parse::<String>().unwrap();
                let op = split.next().unwrap().parse::<char>().unwrap();
                let monkey_right = split.next().unwrap().parse::<String>().unwrap();
                let operation = match op {
                    '+' => Operation::Addition,
                    '-' => Operation::Subtraction,
                    '*' => Operation::Multiplication,
                    '/' => Operation::Division,
                    _ => panic!("Unavailable operation."),
                };
                Ok(MonkeyJob::Op(operation, monkey_left, monkey_right))
            }
        }
    }
}

#[derive(Debug)]
struct Node {
    monkey_job: MonkeyJob,
    parent: Option<String>,
}

#[derive(Debug, Default)]
struct Tree {
    nodes: HashMap<String, Node>,
}

impl Tree {
    fn insert_nodes(&mut self, nodes: Vec<(String, MonkeyJob)>) {
        nodes.iter().for_each(|(name, monkey_job)| {
            self.nodes.insert(
                name.clone(),
                Node {
                    monkey_job: monkey_job.clone(),
                    parent: None,
                },
            );
        });

        // Update parent info
        for node in nodes {
            match node.1 {
                MonkeyJob::Number(_) => {}
                MonkeyJob::Op(_, monkey_left, monkey_right) => {
                    self.nodes.get_mut(&monkey_left).unwrap().parent = Some(node.0.clone());
                    self.nodes.get_mut(&monkey_right).unwrap().parent = Some(node.0.clone());
                }
            }
        }
    }
}

fn calc_monkey_job(monkey: String, tree: &Tree) -> i64 {
    let node = tree.nodes.get(&monkey).unwrap();
    match &node.monkey_job {
        MonkeyJob::Number(num) => *num,
        MonkeyJob::Op(op, monkey_left, monkey_right) => {
            let num1 = calc_monkey_job(monkey_left.clone(), tree);
            let num2 = calc_monkey_job(monkey_right.clone(), tree);
            match op {
                Operation::Addition => num1 + num2,
                Operation::Subtraction => num1 - num2,
                Operation::Multiplication => num1 * num2,
                Operation::Division => num1 / num2,
            }
        }
    }
}

fn find_humn_value(monkey: String, tree: &Tree) -> i64 {
    let mut unknown_monkeys: Vec<String> = Vec::new();
    let mut known_monkeys: Vec<String> = Vec::new();
    let mut cur_monkey = monkey;
    while let Some(parent) = tree.nodes.get(&cur_monkey).unwrap().parent.clone() {
        let parent_job = tree.nodes.get(&parent).unwrap().monkey_job.clone();
        match parent_job {
            MonkeyJob::Number(_) => {}
            MonkeyJob::Op(_, monkey_left, monkey_right) => {
                if monkey_left == cur_monkey {
                    known_monkeys.push(monkey_right);
                } else {
                    known_monkeys.push(monkey_left);
                }
            }
        }
        unknown_monkeys.push(cur_monkey);
        cur_monkey = parent;
    }
    unknown_monkeys.pop();
    let mut cur_unknown_val = calc_monkey_job(known_monkeys.pop().unwrap(), tree);
    for (unknown_name, known_name) in unknown_monkeys.iter().zip(known_monkeys.iter()).rev() {
        let unknown = tree.nodes.get(unknown_name.as_str()).unwrap();
        let parent = unknown.parent.clone().unwrap();
        let known_val = calc_monkey_job(known_name.clone(), tree);
        let (parent_op, is_right_side) = match &tree.nodes.get(&*parent).unwrap().monkey_job {
            MonkeyJob::Number(_) => panic!("Parent is a number."),
            MonkeyJob::Op(op, monkey_left, _) => {
                if *monkey_left == *known_name {
                    (op, false)
                } else {
                    (op, true)
                }
            }
        };
        match parent_op {
            Operation::Addition => {
                cur_unknown_val -= known_val;
            }
            Operation::Subtraction => {
                if is_right_side {
                    cur_unknown_val += known_val;
                } else {
                    cur_unknown_val = known_val - cur_unknown_val;
                }
            }
            Operation::Multiplication => {
                cur_unknown_val /= known_val;
            }
            Operation::Division => {
                if is_right_side {
                    cur_unknown_val *= known_val;
                } else {
                    cur_unknown_val = known_val / cur_unknown_val;
                }
            }
        }
    }
    cur_unknown_val
}

fn get_monkey_tree_from_input(path: impl AsRef<std::path::Path>) -> Result<Tree> {
    let mut tree = Tree::default();
    let input = std::fs::read_to_string(path)?;

    let monkeys: Vec<_> = input
        .lines()
        .map(|line| {
            let mut split = line.split(": ");
            let monkey = split.next().unwrap().parse::<String>().unwrap();
            let monkey_job = split.next().unwrap().parse::<MonkeyJob>().unwrap();
            (monkey, monkey_job)
        })
        .collect();

    tree.insert_nodes(monkeys);
    Ok(tree)
}

fn main() -> Result<()> {
    let tree = get_monkey_tree_from_input("./data/21.input")?;

    let part1 = calc_monkey_job("root".to_string(), &tree);
    println!("Part 1: {}", part1);

    let part2 = find_humn_value("humn".to_string(), &tree);
    println!("Part 2: {}", part2);

    Ok(())
}

#[cfg(test)]
mod day21 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let tree = get_monkey_tree_from_input("./data/21.test")?;
        let part1 = calc_monkey_job("root".to_string(), &tree);
        assert_eq!(part1, 152);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let tree = get_monkey_tree_from_input("./data/21.test")?;
        let part2 = find_humn_value("humn".to_string(), &tree);
        assert_eq!(part2, 301);
        Ok(())
    }
}
