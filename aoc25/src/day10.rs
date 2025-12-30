use anyhow::anyhow;
use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashMap;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 10, solve_all)
}

type BitSet = u64;

#[derive(Debug, Clone, PartialEq)]
struct Machine {
    lights: BitSet,
    buttons: Vec<BitSet>,
    joltage: Vec<usize>,
}

impl std::str::FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start = s.find('[').ok_or("Missing opening bracket")?;
        let end = s.find(']').ok_or("Missing closing bracket")?;
        let lights_str = &s[start + 1..end];
        let lights = bitset_from_bools(lights_str.chars().map(|c| c == '#'));

        let mut buttons = Vec::new();
        let mut rest = &s[end + 1..];
        while let Some(start) = rest.find('(') {
            let end = rest.find(')').ok_or("Missing closing parenthesis")?;
            let button_str = &rest[start + 1..end];
            let mut button_bitset = 0u64;
            for id_str in button_str.split(',') {
                let id: usize = id_str
                    .trim()
                    .parse()
                    .map_err(|e| format!("Parse error: {}", e))?;
                button_bitset |= 1u64 << id;
            }
            buttons.push(button_bitset);
            rest = &rest[end + 1..];
        }

        let start = rest.find('{').ok_or("Missing opening brace")?;
        let end = rest.find('}').ok_or("Missing closing brace")?;
        let joltage_str = &rest[start + 1..end];
        let joltage = joltage_str
            .split(',')
            .map(|n| n.trim().parse().map_err(|e| format!("Parse error: {}", e)))
            .collect::<Result<_, _>>()?;

        Ok(Machine {
            lights,
            buttons,
            joltage,
        })
    }
}

fn bitset_from_bools(bools: impl IntoIterator<Item = bool>) -> BitSet {
    bools
        .into_iter()
        .enumerate()
        .fold(0u64, |acc, (i, b)| acc | ((b as u64) << i))
}

fn min_presses_for_light(machine: &Machine) -> Option<usize> {
    let mut min_cost = None;

    for n_pressed in 1..=machine.buttons.len() {
        if min_cost.is_some_and(|cost| n_pressed > cost) {
            break;
        }
        for combination in machine.buttons.iter().copied().combinations(n_pressed) {
            let lights = combination
                .iter()
                .copied()
                .fold(0u64, |acc, button| acc ^ button);
            if lights == machine.lights {
                let cost = combination.len();
                min_cost = Some(min_cost.map_or(cost, |current| std::cmp::min(current, cost)));
            }
        }
    }
    min_cost
}

fn min_presses_for_joltage(
    joltage: &[usize],
    patterns: &HashMap<BitSet, Vec<(Vec<usize>, usize)>>,
) -> Option<usize> {
    if joltage.iter().all(|&j| j == 0) {
        return Some(0);
    }

    let current_parity = bitset_from_bools(joltage.iter().map(|&j| j % 2 == 1));
    let pattern_list = patterns.get(&current_parity)?;

    let mut min_cost = None;
    for (counter_pattern, cost) in pattern_list {
        let can_subtract = joltage
            .iter()
            .zip(counter_pattern.iter())
            .all(|(&j, &p)| p <= j);

        if !can_subtract {
            continue;
        }

        let new_joltage: Vec<usize> = joltage
            .iter()
            .zip(counter_pattern.iter())
            .map(|(&j, &p)| (j - p) / 2)
            .collect();

        let Some(reduced_cost) = min_presses_for_joltage(&new_joltage, patterns) else {
            continue;
        };

        let total_cost = cost + 2 * reduced_cost;
        min_cost = Some(min_cost.map_or(total_cost, |current| std::cmp::min(current, total_cost)));
    }

    min_cost
}

fn precompute_patterns(
    buttons: &[BitSet],
    n_counters: usize,
) -> HashMap<BitSet, Vec<(Vec<usize>, usize)>> {
    let mut result: HashMap<BitSet, HashMap<Vec<usize>, usize>> = HashMap::new();

    let n_buttons = buttons.len();
    for n_pressed in 0..=n_buttons {
        for combination in (0..n_buttons).combinations(n_pressed) {
            let mut counter_pattern = vec![0usize; n_counters];
            let mut parity = 0u64;

            for &button_id in &combination {
                let button = buttons[button_id];
                for (counter_id, count) in counter_pattern.iter_mut().enumerate() {
                    if (button & (1u64 << counter_id)) != 0 {
                        *count += 1;
                        parity ^= 1u64 << counter_id;
                    }
                }
            }

            let cost = combination.len();
            result
                .entry(parity)
                .or_default()
                .entry(counter_pattern)
                .or_insert(cost);
        }
    }

    result
        .into_iter()
        .map(|(parity, patterns_map)| (parity, patterns_map.into_iter().collect()))
        .collect()
}

fn par_try_reduce<F>(machines: &[Machine], solve_fn: F) -> anyhow::Result<usize>
where
    F: Fn(&Machine) -> Option<usize> + Sync,
{
    machines
        .par_iter()
        .map(|m| solve_fn(m).ok_or_else(|| anyhow!("Failed to find solution")))
        .try_reduce(|| 0, |a, b| Ok(a + b))
}

fn solve_part_one(machines: &[Machine]) -> anyhow::Result<usize> {
    par_try_reduce(machines, min_presses_for_light)
}

fn solve_part_two(machines: &[Machine]) -> anyhow::Result<usize> {
    par_try_reduce(machines, |m| {
        let patterns = precompute_patterns(&m.buttons, m.joltage.len());
        min_presses_for_joltage(&m.joltage, &patterns)
    })
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Machine>> {
    input
        .lines()
        .map(|l| l.parse::<Machine>().map_err(|e| anyhow!(e)))
        .collect::<Result<_, _>>()
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let machines = parse_input(input)?;
    let part_one = solve_part_one(&machines)?.to_string();
    let part_two = solve_part_two(&machines)?.to_string();

    Ok(Solution { part_one, part_two })
}

#[cfg(test)]
mod tests {
    use super::parse_input;
    use super::solve_part_one;
    use super::solve_part_two;
    use utils::load_test_input;

    #[test]
    fn part_one() {
        let input = load_test_input!();
        let machines = parse_input(&input).unwrap();
        let solution = solve_part_one(&machines).unwrap();
        assert_eq!(solution, 7);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let machines = parse_input(&input).unwrap();
        let solution = solve_part_two(&machines).unwrap();
        assert_eq!(solution, 33);
    }
}
