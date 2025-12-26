use anyhow::anyhow;
use hashbrown::HashMap;
use hashbrown::HashSet;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2024, 5, solve_all)
}

type OrderingRules = HashMap<usize, HashSet<usize>>;

fn is_sorted(update: &[usize], ordering: &OrderingRules) -> bool {
    update.windows(2).all(|pair| {
        let less = pair[0];
        let greater = pair[1];
        ordering
            .get(&less)
            .is_some_and(|set| set.contains(&greater))
    })
}

fn sort(update: &mut [usize], ordering_rules: &OrderingRules) {
    update.sort_by(|a, b| {
        if ordering_rules.contains_key(a) {
            if ordering_rules[a].contains(b) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        } else {
            std::cmp::Ordering::Equal
        }
    });
}

fn middle_element(update: &[usize]) -> usize {
    update[update.len() / 2]
}

fn solve_part_one(ordering: &OrderingRules, updates: &[Vec<usize>]) -> usize {
    updates
        .iter()
        .filter_map(|update| {
            if is_sorted(update, ordering) {
                Some(middle_element(update))
            } else {
                None
            }
        })
        .sum()
}

fn solve_part_two(ordering: &OrderingRules, updates: &mut [Vec<usize>]) -> usize {
    updates
        .iter_mut()
        .filter_map(|update| {
            if !is_sorted(update, ordering) {
                sort(update, ordering);
                Some(middle_element(update))
            } else {
                None
            }
        })
        .sum()
}

fn parse_input(input: &str) -> anyhow::Result<(OrderingRules, Vec<Vec<usize>>)> {
    let mut parts = input.split("\n\n");
    let orders_str = parts.next().ok_or(anyhow!("No order part"))?;
    let updates_str = parts.next().ok_or(anyhow!("No update part"))?;

    let rules = orders_str
        .lines()
        .map(|line| {
            let mut parts = line.split('|');
            let less = parts.next().ok_or(anyhow!("No less part"))?.parse()?;
            let greater = parts.next().ok_or(anyhow!("No greater part"))?.parse()?;
            Ok((less, greater))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    let mut page_ordering = OrderingRules::default();
    for (less, greater) in rules {
        page_ordering.entry(less).or_default().insert(greater);
    }

    let updates = updates_str
        .lines()
        .map(|line| {
            let update: Vec<usize> = line
                .split(',')
                .map(|part| part.parse().map_err(|e| anyhow!("Parse error: {}", e)))
                .collect::<anyhow::Result<Vec<_>>>()?;
            Ok(update)
        })
        .collect::<anyhow::Result<Vec<_>>>()?;

    Ok((page_ordering, updates))
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let (ordering, mut updates) = parse_input(input)?;
    let part_one = solve_part_one(&ordering, &updates).to_string();
    let part_two = solve_part_two(&ordering, &mut updates).to_string();

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
        let (ordering, updates) = parse_input(&input).unwrap();
        let solution = solve_part_one(&ordering, &updates);
        assert_eq!(solution, 143);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!();
        let (ordering, mut updates) = parse_input(&input).unwrap();
        let solution = solve_part_two(&ordering, &mut updates);
        assert_eq!(solution, 123);
    }
}
