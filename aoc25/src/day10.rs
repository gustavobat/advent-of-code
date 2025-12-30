use anyhow::anyhow;
use std::collections::HashSet;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 10, solve_all)
}

use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

impl FromStr for Machine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start = s.find('[').ok_or("Missing opening bracket")?;
        let end = s.find(']').ok_or("Missing closing bracket")?;
        let lights_str = &s[start + 1..end];
        let lights = lights_str.chars().map(|c| c == '#').collect();

        let mut buttons = Vec::new();
        let mut rest = &s[end + 1..];
        while let Some(start) = rest.find('(') {
            let end = rest.find(')').ok_or("Missing closing parenthesis")?;
            let button_str = &rest[start + 1..end];

            let button: Vec<usize> = button_str
                .split(',')
                .map(|n| n.trim().parse().map_err(|e| format!("Parse error: {}", e)))
                .collect::<Result<_, _>>()?;

            buttons.push(button);
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

fn dfs(matrix: &[Vec<bool>], target: Vec<bool>, clicked: HashSet<usize>) -> Option<usize> {
    if target.iter().all(|&x| !x) {
        return Some(clicked.len());
    }

    let next_light = target.iter().position(|&x| x)?;
    let candidates = (0..matrix.len())
        .filter(|&button| !clicked.contains(&button) && matrix[button][next_light]);

    let mut min_presses: Option<usize> = None;
    for button in candidates {
        let mut new_target = target.clone();
        for (i, &affects) in matrix[button].iter().enumerate() {
            if affects {
                new_target[i] ^= true;
            }
        }

        let mut new_clicked = clicked.clone();
        new_clicked.insert(button);

        if let Some(count) = dfs(matrix, new_target, new_clicked) {
            min_presses = Some(match min_presses {
                None => count,
                Some(b) => std::cmp::min(b, count),
            });
        }
    }

    min_presses
}

fn dfs_joltage_reach(
    matrix: &[Vec<usize>],
    current: Vec<usize>,
    target: &[usize],
    total_presses: usize,
) -> Option<usize> {
    // Check if current matches target (solved)
    if current == target {
        return Some(total_presses);
    }

    // Find the first position where current doesn't match target
    let next_pos = current
        .iter()
        .zip(target.iter())
        .position(|(c, t)| c != t)?;

    // Filter buttons that can affect this position
    let mut min_presses: Option<usize> = None;
    for button in (0..matrix.len()).filter(|&button| matrix[button][next_pos] != 0) {
        // Create new current by adding this button's effects
        let mut new_current = current.clone();
        let mut valid = true;
        for (i, &increment) in matrix[button].iter().enumerate() {
            new_current[i] += increment;
            // Check if we've overshot the target
            if new_current[i] > target[i] {
                valid = false;
                break;
            }
        }

        if !valid {
            continue;
        }

        // Recurse with incremented press count
        if let Some(count) = dfs_joltage_reach(matrix, new_current, target, total_presses + 1) {
            min_presses = Some(match min_presses {
                None => count,
                Some(b) => std::cmp::min(b, count),
            });
        }
    }

    min_presses
}

fn button_to_light_matrix(machine: &Machine) -> Vec<Vec<bool>> {
    let n_lights = machine.lights.len();
    let n_buttons = machine.buttons.len();
    let mut matrix = vec![vec![false; n_lights]; n_buttons];
    for (i, button) in machine.buttons.iter().enumerate() {
        for &j in button {
            matrix[i][j] = true;
        }
    }

    // Sort by number of affected lights in descending order (most affected first)
    matrix.sort_unstable_by_key(|row| std::cmp::Reverse(row.iter().filter(|&&x| x).count()));

    matrix
}

fn button_to_joltage_matrix(machine: &Machine) -> Vec<Vec<usize>> {
    let n_joltages = machine.joltage.len();
    let n_buttons = machine.buttons.len();
    let mut matrix = vec![vec![0; n_joltages]; n_buttons];
    for (button_idx, button) in machine.buttons.iter().enumerate() {
        for &counter_idx in button {
            if counter_idx < n_joltages {
                matrix[button_idx][counter_idx] = 1;
            }
        }
    }

    // Sort by number of affected counters in descending order (most affected first)
    matrix.sort_unstable_by_key(|row| std::cmp::Reverse(row.iter().filter(|&&x| x != 0).count()));

    matrix
}

fn calc_min_presses(machine: &Machine) -> usize {
    let matrix = button_to_light_matrix(machine);
    let target = machine.lights.clone();
    let clicked = HashSet::new();
    dfs(&matrix, target, clicked).unwrap_or(0)
}

fn calc_min_joltage_presses(machine: &Machine) -> usize {
    let matrix = button_to_joltage_matrix(machine);
    let target = machine.joltage.clone();
    let initial = vec![0; target.len()];
    dfs_joltage_reach(&matrix, initial, &target, 0).unwrap_or(0)
}

fn solve_part_one(machines: &[Machine]) -> usize {
    println!("Solving part1");
    let mut res = 0;
    for (i, machine) in machines.iter().enumerate() {
        let new = calc_min_presses(machine);
        println!("{}: {}", i, new);
        res += new;
    }
    res
}

fn solve_part_two(machines: &[Machine]) -> usize {
    println!("Solving part2");
    let mut res = 0;
    for (i, machine) in machines.iter().enumerate() {
        let new = calc_min_joltage_presses(machine);
        println!("{}: {}", i, new);
        res += new;
    }
    res
}

fn parse_input(input: &str) -> anyhow::Result<Vec<Machine>> {
    input
        .lines()
        .map(|l| l.parse::<Machine>().map_err(|e| anyhow!(e)))
        .collect::<Result<_, _>>()
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let machines = parse_input(input)?;
    let part_one = solve_part_one(&machines).to_string();
    let part_two = solve_part_two(&machines).to_string();

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
        let solution = solve_part_one(&machines);
        assert_eq!(solution, 7);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let machines = parse_input(&input).unwrap();
        let solution = solve_part_two(&machines);
        assert_eq!(solution, 33);
    }
}
