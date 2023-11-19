use anyhow::Result;
use itertools::Itertools;
use std::{fmt, str::FromStr};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Name([char; 2]);

impl fmt::Debug for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.0[0], self.0[1])
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Copy, Clone)]
struct State {
    seen: u64,
    current_valve: [usize; 2],
    time_left: [u32; 2],
    current_flow: u32,
    max_flow: u32,
    first_move: Option<usize>,
}

impl State {
    fn has_visited(&self, i: usize) -> bool {
        self.seen & (1 << i) != 0
    }

    fn visit(&mut self, i: usize) {
        self.seen |= 1 << i;
    }

    fn count_visited(&self) -> usize {
        self.seen.count_ones() as usize
    }
}

#[derive(Debug)]
struct Valve {
    name: Name,
    flow: u32,
    links: Vec<Name>,
}

impl FromStr for Valve {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut name_chars = s.split(' ').nth(1).unwrap().chars();
        let name = Name([name_chars.next().unwrap(), name_chars.next().unwrap()]);
        let flow: u32 = s
            .split('=')
            .nth(1)
            .unwrap()
            .split(';')
            .next()
            .unwrap()
            .parse()
            .unwrap();

        let links = s
            .split(',')
            .map(|ss| {
                let name = &mut ss[ss.len() - 2..].chars();
                Name([name.next().unwrap(), name.next().unwrap()])
            })
            .collect_vec();

        Ok(Valve { name, flow, links })
    }
}

fn floyd_warshall(valves: &[Valve]) -> Vec<Vec<u32>> {
    let mut dist = vec![vec![u32::MAX; valves.len()]; valves.len()];

    for (i, v) in valves.iter().enumerate() {
        dist[i][i] = 0;
        for l in v.links.iter() {
            let j = valves.iter().position(|v| v.name == *l).unwrap();
            dist[i][j] = 1;
        }
    }

    for k in 0..valves.len() {
        for i in 0..valves.len() {
            for j in 0..valves.len() {
                if dist[i][k] != u32::MAX && dist[k][j] != u32::MAX {
                    dist[i][j] = dist[i][j].min(dist[i][k] + dist[k][j]);
                }
            }
        }
    }

    dist
}

fn traverse(dists: &Vec<Vec<u32>>, flows: &Vec<u32>, state: &mut State) {
    if state.count_visited() == 1 && state.first_move.is_some() {
        let to = state.first_move.unwrap();
        let dist = dists[state.current_valve[0]][to] + 1;
        let total_outcome = (state.time_left[0] - dist) * flows[to];

        state.visit(to);
        state.time_left[0] -= dist;
        state.current_valve[0] = to;
        state.current_flow += total_outcome;
        state.max_flow = state.current_flow;
    }

    if state.time_left[1] > state.time_left[0] {
        state.time_left.swap(0, 1);
        state.current_valve.swap(0, 1);
    }
    let n = dists.len();
    let best = (0..n)
        .filter(|&i| !state.has_visited(i))
        .filter(|&i| dists[state.current_valve[0]][i] < state.time_left[0])
        .filter(|&i| flows[i] > 0)
        .map(|to| {
            let dist = dists[state.current_valve[0]][to] + 1;
            let total_outcome = (state.time_left[0] - dist) * flows[to];
            (to, dist, total_outcome)
        })
        .sorted_by(|(_, _, a), (_, _, b)| b.cmp(a))
        .collect::<Vec<_>>();
    let old_state = *state;
    // Assume the optimal solution is within the first 10 best options
    // This reduces exec time and is not a rigid assumption
    best.iter().take(10).for_each(|(to, dist, total_outcome)| {
        let mut new_state = old_state;
        new_state.visit(*to);
        new_state.time_left[0] -= dist;
        new_state.current_valve[0] = *to;
        new_state.current_flow += total_outcome;
        new_state.max_flow = new_state.max_flow.max(new_state.current_flow);
        traverse(dists, flows, &mut new_state);
        if new_state.max_flow > state.max_flow {
            // We cache the first move to avoid having to find it again in part 2.
            if state.count_visited() == 1 {
                state.first_move = Some(*to);
            }
            state.max_flow = new_state.max_flow;
        }
    });
}

fn solve(
    dists: &Vec<Vec<u32>>,
    flows: &Vec<u32>,
    start: usize,
    time_left: [u32; 2],
    first_move: Option<usize>,
) -> State {
    let mut state = State {
        seen: 1 << start,
        current_valve: [start; 2],
        time_left,
        max_flow: 0,
        current_flow: 0,
        first_move,
    };

    traverse(dists, flows, &mut state);
    state
}

fn get_input(path: &str) -> Result<(Vec<Vec<u32>>, Vec<u32>, usize)> {
    let valves: Vec<Valve> = utils::parse_each_line(path)?;
    let dists = floyd_warshall(&valves);
    let flows = valves.iter().map(|v| v.flow).collect_vec();
    let start = valves.iter().position(|v| v.name.0 == ['A', 'A']).unwrap();
    Ok((dists, flows, start))
}

fn main() -> Result<()> {
    let (dists, flows, start) = get_input("./data/16.input")?;
    let part1 = solve(&dists, &flows, start, [30, 0], None);
    println!("Part 1: {:?}", part1.max_flow);
    let part2 = solve(&dists, &flows, start, [26, 26], part1.first_move);
    println!("Part 2: {:?}", part2.max_flow);

    Ok(())
}

#[cfg(test)]
mod day16 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let (dists, flows, start) = get_input("./data/16.test")?;
        let part1 = solve(&dists, &flows, start, [30, 0], None);
        assert_eq!(part1.max_flow, 1651);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let (dists, flows, start) = get_input("./data/16.test")?;
        let part2 = solve(&dists, &flows, start, [26, 26], None);
        assert_eq!(part2.max_flow, 1707);
        Ok(())
    }
}
