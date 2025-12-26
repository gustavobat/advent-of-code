use std::collections::HashMap;
use utils::solution::Solution;
use utils::solution::Solver;

inventory::submit! {
    Solver::new(2025, 11, solve_all)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct MemoKey<'a> {
    device: &'a str,
    seen_fft: bool,
    seen_dac: bool,
}

fn count_paths<'a>(
    device: &'a str,
    device_outputs: &HashMap<&'a str, Vec<&'a str>>,
    require_fft_and_dac: bool,
    seen_fft: bool,
    seen_dac: bool,
    memo: &mut HashMap<MemoKey<'a>, usize>,
) -> usize {
    if device == "out" {
        let fft_ok = !require_fft_and_dac || seen_fft;
        let dac_ok = !require_fft_and_dac || seen_dac;
        return if fft_ok && dac_ok { 1 } else { 0 };
    }

    let cache_key = MemoKey {
        device,
        seen_fft,
        seen_dac,
    };

    if let Some(&cached) = memo.get(&cache_key) {
        return cached;
    }

    let new_seen_fft = seen_fft || device == "fft";
    let new_seen_dac = seen_dac || device == "dac";

    let outputs = device_outputs
        .get(device)
        .expect("All devices that are not 'out' should be in the map");

    let mut total = 0;
    for output in outputs {
        total += count_paths(
            output,
            device_outputs,
            require_fft_and_dac,
            new_seen_fft,
            new_seen_dac,
            memo,
        );
    }

    memo.insert(cache_key, total);
    total
}

fn solve_part_one(device_outputs: &HashMap<&str, Vec<&str>>) -> usize {
    let mut memo = HashMap::new();
    count_paths("you", device_outputs, false, false, false, &mut memo)
}

fn solve_part_two(device_outputs: &HashMap<&str, Vec<&str>>) -> usize {
    let mut memo = HashMap::new();
    count_paths("svr", device_outputs, true, false, false, &mut memo)
}

fn parse_input<'a>(input: &'a str) -> anyhow::Result<HashMap<&'a str, Vec<&'a str>>> {
    let mut map: HashMap<&'a str, Vec<&'a str>> = HashMap::new();
    for (i, line) in input.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Some((left, right)) = line.split_once(':') else {
            return Err(anyhow::anyhow!("Invalid line {}: missing ':'", i + 1));
        };
        let key = left.trim();
        let values_str = right.trim();
        let values: Vec<&'a str> = if values_str.is_empty() {
            Vec::new()
        } else {
            values_str.split_whitespace().collect()
        };
        map.insert(key, values);
    }
    Ok(map)
}

fn solve_all(input: &str) -> anyhow::Result<Solution> {
    let device_outputs = parse_input(input)?;
    let part_one = solve_part_one(&device_outputs).to_string();
    let part_two = solve_part_two(&device_outputs).to_string();

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
        let input = load_test_input!(1);
        let device_outputs = parse_input(&input).unwrap();
        let solution = solve_part_one(&device_outputs);
        assert_eq!(solution, 5);
    }

    #[test]
    fn part_two() {
        let input = load_test_input!(2);
        let device_outputs = parse_input(&input).unwrap();
        let solution = solve_part_two(&device_outputs);
        assert_eq!(solution, 2);
    }
}
