use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!("{:?}", compute_result(std::io::stdin().lock()));
}

const MAX_MINUTE: u8 = 30;
const OPEN_VALVE_TIME: u8 = 1;
const MOVE_TIME: u8 = 1;
const TEACH_ELEPHANT_TIME: u8 = 4;
const MAX_STATES: usize = 32000000;

fn compute_result(input: impl BufRead) -> (u16, u16) {
    let (nodes, start) = parse_valve_graph(input);
    let context = Context {
        start,
        valves_with_non_zero_flow_rate: nodes
            .iter()
            .enumerate()
            .filter(|(_, v)| v.flow_rate > 0)
            .map(|(v, _)| (v, find_all_shortest_paths(v, &nodes)))
            .collect(),
        max_flow_rate: nodes.iter().map(|v| v.flow_rate).sum(),
        nodes,
    };
    (
        find_max_released_pressure(&context),
        find_max_released_pressure_with_elephant(&context),
    )
}

fn find_max_released_pressure(context: &Context) -> u16 {
    let mut states = vec![context.make_initial_state()];
    let mut new_states = BinaryHeap::new();
    new_states.push((0, 0));
    let mut max_released_pressure = 0;
    let mut visited: HashMap<Key, usize> = HashMap::new();
    visited.insert(make_state_key(&states[0]), 0);
    while let Some((_, state_index)) = new_states.pop() {
        if states[state_index].minute == MAX_MINUTE {
            max_released_pressure = states[state_index].released_pressure;
            break;
        }
        if states.len() >= MAX_STATES {
            continue;
        }
        let flow_rate = context.get_total_flow_rate(&states[state_index].open_valves);
        for action in context.generate_actions(0, &states[state_index]) {
            let mut new_state = states[state_index].clone();
            let duration = apply_action(0, &action, &mut new_state);
            new_state.released_pressure += flow_rate * duration as u16;
            new_state.minute += duration;
            let priority = context.get_priority(&new_state);
            let new_state_index = match visited.entry(make_state_key(&new_state)) {
                Entry::Occupied(v) => {
                    let new_state_index = *v.get();
                    if states[new_state_index].released_pressure >= new_state.released_pressure {
                        continue;
                    }
                    states[new_state_index].released_pressure = new_state.released_pressure;
                    new_state_index
                }
                Entry::Vacant(v) => {
                    let new_state_index = states.len();
                    v.insert(new_state_index);
                    states.push(new_state);
                    new_state_index
                }
            };
            new_states.push((priority, new_state_index));
        }
    }
    max_released_pressure
}

type Key = (u8, usize, Vec<usize>);

fn make_state_key(state: &State) -> Key {
    let mut open_valves = state.open_valves.clone();
    open_valves.sort();
    (state.minute, state.positions[0], open_valves)
}

fn find_max_released_pressure_with_elephant(context: &Context) -> u16 {
    let mut states = vec![context.make_initial_state_with_elephant()];
    let mut new_states = BinaryHeap::new();
    new_states.push((0, 0));
    let mut max_released_pressure = 0;
    let mut visited: HashMap<FullKey, usize> = HashMap::new();
    visited.insert(make_state_key_full(&states[0]), 0);
    while let Some((_, state_index)) = new_states.pop() {
        if states[state_index].minute == MAX_MINUTE {
            max_released_pressure = states[state_index].released_pressure;
            break;
        }
        if states.len() >= MAX_STATES {
            continue;
        }
        let flow_rate = context.get_total_flow_rate(&states[state_index].open_valves);
        let actions1 = context.generate_actions(0, &states[state_index]);
        let actions2 = context.generate_actions(1, &states[state_index]);
        for action1 in actions1.iter() {
            for action2 in actions2.iter() {
                if are_conflicting_actions(action1, action2, &states[state_index]) {
                    continue;
                }
                let mut new_state = states[state_index].clone();
                let duration1 = apply_action(0, action1, &mut new_state);
                let duration2 = apply_action(1, action2, &mut new_state);
                let duration = duration1.min(duration2);
                new_state.busy[0] = duration1;
                new_state.busy[1] = duration2;
                new_state.minute += duration;
                new_state.released_pressure += flow_rate * duration as u16;
                for busy in new_state.busy.iter_mut() {
                    *busy -= duration;
                }
                let priority = context.get_priority(&new_state);
                let new_state_index = match visited.entry(make_state_key_full(&new_state)) {
                    Entry::Occupied(v) => {
                        let new_state_index = *v.get();
                        if states[new_state_index].released_pressure >= new_state.released_pressure
                        {
                            continue;
                        }
                        states[new_state_index].released_pressure = new_state.released_pressure;
                        new_state_index
                    }
                    Entry::Vacant(v) => {
                        let new_state_index = states.len();
                        v.insert(new_state_index);
                        states.push(new_state);
                        new_state_index
                    }
                };
                new_states.push((priority, new_state_index));
            }
        }
    }
    max_released_pressure
}

type FullKey = (u8, [usize; 2], [bool; 2], Vec<usize>);

fn make_state_key_full(state: &State) -> FullKey {
    let mut open_valves = state.open_valves.clone();
    open_valves.sort();
    (
        state.minute,
        state.positions,
        [state.busy[0] > 0, state.busy[1] > 0],
        open_valves,
    )
}

fn are_conflicting_actions(a: &Action, b: &Action, state: &State) -> bool {
    match (a, b) {
        (Action::OpenValve, Action::OpenValve) => state.positions[0] == state.positions[1],
        (Action::MoveTo { dst: dst_a, .. }, Action::MoveTo { dst: dst_b, .. }) => dst_a == dst_b,
        _ => false,
    }
}

struct Context {
    start: usize,
    valves_with_non_zero_flow_rate: HashMap<usize, Vec<u8>>,
    max_flow_rate: u16,
    nodes: Vec<Node>,
}

impl Context {
    fn make_initial_state(&self) -> State {
        State {
            minute: 0,
            released_pressure: 0,
            positions: [self.start; 2],
            busy: [0; 2],
            open_valves: Vec::new(),
        }
    }

    fn make_initial_state_with_elephant(&self) -> State {
        State {
            minute: TEACH_ELEPHANT_TIME,
            released_pressure: 0,
            positions: [self.start; 2],
            busy: [0; 2],
            open_valves: Vec::new(),
        }
    }

    fn get_priority(&self, state: &State) -> u16 {
        state.released_pressure + self.max_flow_rate * (MAX_MINUTE - state.minute) as u16
    }

    fn generate_actions(&self, agent: usize, state: &State) -> Vec<Action> {
        let mut actions = Vec::new();
        if state.busy[agent] > 0 {
            actions.push(Action::Busy {
                duration: state.busy[agent],
            });
        } else {
            let position = state.positions[agent];
            if !state.open_valves.contains(&position) && self.nodes[position].flow_rate > 0 {
                actions.push(Action::OpenValve);
            }
            if state.open_valves.len() < self.valves_with_non_zero_flow_rate.len() {
                for (dst, distances) in self.valves_with_non_zero_flow_rate.iter() {
                    if *dst == position || state.open_valves.contains(dst) {
                        continue;
                    }
                    let duration = distances[position] * MOVE_TIME;
                    if state.minute + duration > MAX_MINUTE {
                        continue;
                    }
                    actions.push(Action::MoveTo {
                        dst: *dst,
                        duration,
                    });
                }
            }
            actions.push(Action::Idle);
        }
        actions
    }

    fn get_total_flow_rate(&self, open_valves: &[usize]) -> u16 {
        open_valves.iter().map(|v| self.nodes[*v].flow_rate).sum()
    }
}

fn apply_action(agent: usize, action: &Action, state: &mut State) -> u8 {
    match action {
        Action::Busy { duration } => *duration,
        Action::Idle => MAX_MINUTE - state.minute,
        Action::OpenValve => {
            assert!(!state.open_valves.contains(&state.positions[agent]));
            state.open_valves.push(state.positions[agent]);
            OPEN_VALVE_TIME
        }
        Action::MoveTo { dst, duration } => {
            state.positions[agent] = *dst;
            *duration
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum Action {
    Busy { duration: u8 },
    Idle,
    OpenValve,
    MoveTo { dst: usize, duration: u8 },
}

fn find_all_shortest_paths(src: usize, nodes: &[Node]) -> Vec<u8> {
    let mut distances: Vec<u8> = std::iter::repeat(u8::MAX).take(nodes.len()).collect();
    distances[src] = 0;
    let mut to_visit = BinaryHeap::new();
    to_visit.push((Reverse(0), src));
    while let Some((_, cur)) = to_visit.pop() {
        for &next in nodes[cur].tunnels_to.iter() {
            let new_length = distances[cur] + 1;
            if distances[next] <= new_length {
                continue;
            }
            distances[next] = new_length;
            to_visit.push((Reverse(new_length), next));
        }
    }
    distances
}

#[derive(Clone, Debug)]
struct State {
    minute: u8,
    released_pressure: u16,
    positions: [usize; 2],
    busy: [u8; 2],
    open_valves: Vec<usize>,
}

fn parse_valve_graph(input: impl BufRead) -> (Vec<Node>, usize) {
    let valves: Vec<Valve> = input
        .lines()
        .map(|v| {
            let line = v.unwrap();
            let tail1 = line.strip_prefix("Valve ").unwrap();
            let (name, tail2) = tail1.split_once(" has flow rate=").unwrap();
            let (flow_rate, tail3) = tail2.split_once("; ").unwrap();
            Valve {
                name: name.to_string(),
                flow_rate: u16::from_str(flow_rate).unwrap(),
                tunnels_to: if let Some(tunnels) = tail3.strip_prefix("tunnels lead to valves ") {
                    tunnels.split(", ").map(|v| v.to_string()).collect()
                } else if let Some(tunnel) = tail3.strip_prefix("tunnel leads to valve ") {
                    vec![tunnel.to_string()]
                } else {
                    unreachable!()
                },
            }
        })
        .collect();
    let mut valve_index_by_name = HashMap::new();
    for (i, valve) in valves.iter().enumerate() {
        valve_index_by_name.insert(valve.name.as_str(), i);
    }
    (
        valves
            .iter()
            .map(|v| Node {
                flow_rate: v.flow_rate,
                tunnels_to: v
                    .tunnels_to
                    .iter()
                    .map(|v| valve_index_by_name[v.as_str()])
                    .collect(),
            })
            .collect(),
        valve_index_by_name["AA"],
    )
}

#[derive(Debug)]
struct Node {
    flow_rate: u16,
    tunnels_to: Vec<usize>,
}

struct Valve {
    name: String,
    flow_rate: u16,
    tunnels_to: Vec<String>,
}

#[test]
fn example_test() {
    let buffer = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), (1651, 1707));
}

#[test]
fn example_test_1() {
    let buffer = r#"Valve AA has flow rate=0; tunnels lead to valves BB
Valve BB has flow rate=13; tunnels lead to valves AA
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), (364, 312));
}

#[test]
fn example_test_2() {
    let buffer = r#"Valve AA has flow rate=0; tunnels lead to valves BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves BB
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), (416, 358));
}
