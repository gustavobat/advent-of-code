use anyhow::Result;
use itertools::Itertools;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Clone, Eq, PartialEq, Debug)]
enum Value {
    Number(u32),
    Array(Vec<Value>),
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Value, Self::Err> {
        let mut num = None;
        let mut root = vec![vec![]];
        let push_num = |num: Option<u32>, vecs: &mut Vec<Vec<Value>>| {
            let depth = vecs.len() - 1;
            if let Some(num) = num {
                vecs[depth].push(Value::Number(num));
            }
            None
        };
        for c in s.chars() {
            match c {
                '[' => root.push(vec![]),
                ']' => {
                    num = push_num(num, &mut root);
                    let parent_depth = root.len() - 2;
                    let child = root.pop().unwrap();
                    root[parent_depth].push(Value::Array(child));
                }
                ' ' => (),
                ',' => num = push_num(num, &mut root),
                '0'..='9' => num = Some(num.unwrap_or(0) * 10 + c.to_digit(10).unwrap()),
                _ => (),
            };
        }
        Ok(root[0].pop().unwrap())
    }
}

impl PartialOrd<Self> for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Number(l_val), Value::Number(r_val)) => (*l_val).cmp(r_val),
            (Value::Array(l_list), Value::Array(r_list)) => {
                for (l, r) in l_list.iter().zip(r_list.iter()) {
                    match l.cmp(r) {
                        Ordering::Equal => continue,
                        other => return other,
                    };
                }
                l_list.len().cmp(&r_list.len())
            }
            (list, Value::Number(val)) => list.cmp(&Value::Array(vec![Value::Number(*val)])),
            (Value::Number(val), list) => Value::Array(vec![Value::Number(*val)]).cmp(list),
        }
    }
}

fn solve_part1(packets: &[Value]) -> Result<usize> {
    let acc_indices_of_right_order_pairs = packets
        .iter()
        .tuples()
        .enumerate()
        .filter(|(_, (left, right))| left < right)
        .map(|(i, _)| i + 1)
        .sum();
    Ok(acc_indices_of_right_order_pairs)
}

fn solve_part2(packets: &mut Vec<Value>) -> Result<usize> {
    let divider_packets: Vec<Value> = vec!["[[2]]".parse().unwrap(), "[[6]]".parse().unwrap()];
    packets.extend(divider_packets.clone());
    packets.sort();
    let decoder_key = packets
        .iter()
        .enumerate()
        .filter(|(_, packet)| divider_packets.contains(packet))
        .map(|(i, _)| i + 1)
        .product();
    Ok(decoder_key)
}

fn main() -> Result<()> {
    let mut packets: Vec<Value> = utils::parse_each_line("./data/13.input")?;

    let part1 = solve_part1(&packets)?;
    println!("Part 1: {part1}");

    let part2 = solve_part2(&mut packets)?;
    println!("Part 2: {part2}");

    Ok(())
}

#[cfg(test)]
mod day13 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let packets: Vec<Value> = utils::parse_each_line("./data/13.test")?;
        let part1 = solve_part1(&packets)?;
        assert_eq!(part1, 13);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let mut packets = utils::parse_each_line("./data/13.test")?;
        let part2 = solve_part2(&mut packets)?;
        assert_eq!(part2, 140);
        Ok(())
    }
}
