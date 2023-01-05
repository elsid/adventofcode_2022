use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!("{:?}", compute_result(std::io::stdin().lock(), 3));
}

const MAX_ITERATIONS: usize = 20_000_000;

fn compute_result(input: impl BufRead, blueprints_left: usize) -> (u64, u64) {
    let blueprints = parse_blueprints(input);
    (
        blueprints
            .iter()
            .map(|blueprint| blueprint.id as u64 * find_max_geodes(blueprint, 24) as u64)
            .sum(),
        blueprints
            .iter()
            .take(blueprints_left)
            .map(|blueprint| find_max_geodes(blueprint, 32) as u64)
            .product(),
    )
}

fn find_max_geodes(blueprint: &Blueprint, max_minute: u8) -> u8 {
    let initial_state = State {
        ore_robots: 1,
        ..Default::default()
    };
    let mut visited = HashMap::<StateKey, usize>::new();
    visited.insert(make_state_key(&initial_state), 0);
    let mut states = vec![initial_state];
    let mut incoming = BinaryHeap::new();
    incoming.push((Reverse(0), 0));
    let mut actions = Vec::new();
    let mut iterations = 0;
    while let Some((Reverse(_), state_index)) = incoming.pop() {
        if states[state_index].minute == max_minute {
            return states[state_index].geodes;
        }
        iterations += 1;
        assert!(iterations < MAX_ITERATIONS, "iterations={}", iterations);
        actions.clear();
        generate_actions(blueprint, &states[state_index], &mut actions);
        for action in actions.iter() {
            let mut new_state = states[state_index].clone();
            apply_action(blueprint, action, &mut new_state);
            let cost = get_cost(&new_state);
            match visited.entry(make_state_key(&new_state)) {
                Entry::Occupied(v) => {
                    if get_cost(&states[*v.get()]) > cost {
                        incoming.push((
                            Reverse(cost + get_heuristic(max_minute, &new_state)),
                            *v.get(),
                        ));
                        states[*v.get()] = new_state;
                    }
                }
                Entry::Vacant(v) => {
                    v.insert(states.len());
                    incoming.push((
                        Reverse(cost + get_heuristic(max_minute, &new_state)),
                        states.len(),
                    ));
                    states.push(new_state);
                }
            }
        }
    }
    unreachable!();
}

fn get_cost(state: &State) -> i64 {
    let theoretical_geodes_gathered = sum_from_1_to_n_minus_one(state.minute as i64);
    theoretical_geodes_gathered - state.geodes as i64
}

fn get_heuristic(max_minute: u8, state: &State) -> i64 {
    let theoretical_geodes_max = sum_from_1_to_n_minus_one(max_minute as i64);
    let theoretical_geodes_gathered = sum_from_1_to_n_minus_one(state.minute as i64);
    let time_left = (max_minute - state.minute) as i64;
    let theoretical_geodes_to_be_gathered = sum_from_1_to_n_minus_one(time_left);
    theoretical_geodes_max
        - theoretical_geodes_gathered
        - theoretical_geodes_to_be_gathered
        - state.geode_robots as i64 * time_left
}

fn sum_from_1_to_n_minus_one(n: i64) -> i64 {
    if n > 0 {
        n * (n - 1) / 2
    } else {
        0
    }
}

fn apply_action(blueprint: &Blueprint, action: &Action, state: &mut State) {
    let ore = state.ore_robots;
    let clay = state.clay_robots;
    let obsidian = state.obsidian_robots;
    let geodes = state.geode_robots;
    match action {
        Action::None => (),
        Action::BuildOreRobot => {
            state.ore -= blueprint.ore_robot_cost.ore;
            state.ore_robots += 1;
        }
        Action::BuildClayRobot => {
            state.ore -= blueprint.clay_robot_cost.ore;
            state.clay_robots += 1;
        }
        Action::BuildObsidianRobot => {
            state.ore -= blueprint.obsidian_robot_cost.ore;
            state.clay -= blueprint.obsidian_robot_cost.clay;
            state.obsidian_robots += 1;
        }
        Action::BuildGeodeRobot => {
            state.ore -= blueprint.geode_robot_cost.ore;
            state.obsidian -= blueprint.geode_robot_cost.obsidian;
            state.geode_robots += 1;
        }
    }
    state.ore += ore;
    state.clay += clay;
    state.obsidian += obsidian;
    state.geodes += geodes;
    state.minute += 1;
}

fn generate_actions(blueprint: &Blueprint, state: &State, actions: &mut Vec<Action>) {
    if state.ore >= blueprint.geode_robot_cost.ore
        && state.obsidian >= blueprint.geode_robot_cost.obsidian
    {
        actions.push(Action::BuildGeodeRobot);
    }
    if state.ore >= blueprint.obsidian_robot_cost.ore
        && state.clay >= blueprint.obsidian_robot_cost.clay
    {
        actions.push(Action::BuildObsidianRobot);
    }
    if state.ore >= blueprint.clay_robot_cost.ore {
        actions.push(Action::BuildClayRobot);
    }
    if state.ore >= blueprint.ore_robot_cost.ore {
        actions.push(Action::BuildOreRobot);
    }
    if actions.len() < 4 {
        actions.push(Action::None);
    }
}

fn parse_blueprints(input: impl BufRead) -> Vec<Blueprint> {
    input
        .lines()
        .map(|v| {
            let line = v.unwrap();
            let blueprint = line.strip_prefix("Blueprint ").unwrap();
            let (id, tail1) = blueprint.split_once(": Each ore robot costs ").unwrap();
            let (ore_robot_ore, tail2) = tail1.split_once(" ore. Each clay robot costs ").unwrap();
            let (clay_robot_ore, tail3) = tail2
                .split_once(" ore. Each obsidian robot costs ")
                .unwrap();
            let (obsidian_robot, tail4) =
                tail3.split_once(" clay. Each geode robot costs ").unwrap();
            let (geode_robot, _) = tail4.split_once(" obsidian.").unwrap();
            let (obsidian_robot_ore, obsidian_robot_clay) =
                obsidian_robot.split_once(" ore and ").unwrap();
            let (geode_robot_ore, geode_robot_obsidian) =
                geode_robot.split_once(" ore and ").unwrap();
            Blueprint {
                id: u8::from_str(id).unwrap(),
                ore_robot_cost: OreRobotCost {
                    ore: u8::from_str(ore_robot_ore).unwrap(),
                },
                clay_robot_cost: ClayRobotCost {
                    ore: u8::from_str(clay_robot_ore).unwrap(),
                },
                obsidian_robot_cost: ObsidianRobotCost {
                    ore: u8::from_str(obsidian_robot_ore).unwrap(),
                    clay: u8::from_str(obsidian_robot_clay).unwrap(),
                },
                geode_robot_cost: GeodeRobotCost {
                    ore: u8::from_str(geode_robot_ore).unwrap(),
                    obsidian: u8::from_str(geode_robot_obsidian).unwrap(),
                },
            }
        })
        .collect()
}

#[derive(Default, Debug)]
struct Blueprint {
    id: u8,
    ore_robot_cost: OreRobotCost,
    clay_robot_cost: ClayRobotCost,
    obsidian_robot_cost: ObsidianRobotCost,
    geode_robot_cost: GeodeRobotCost,
}

#[derive(Default, Debug)]
struct OreRobotCost {
    ore: u8,
}

#[derive(Default, Debug)]
struct ClayRobotCost {
    ore: u8,
}

#[derive(Default, Debug)]
struct ObsidianRobotCost {
    ore: u8,
    clay: u8,
}

#[derive(Default, Debug)]
struct GeodeRobotCost {
    ore: u8,
    obsidian: u8,
}

#[derive(Default, Debug, Clone)]
struct State {
    minute: u8,
    ore: u8,
    clay: u8,
    obsidian: u8,
    geodes: u8,
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
}

#[derive(Default, Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct StateKey {
    ore: u8,
    clay: u8,
    obsidian: u8,
    ore_robots: u8,
    clay_robots: u8,
    obsidian_robots: u8,
    geode_robots: u8,
}

#[derive(Debug)]
enum Action {
    None,
    BuildOreRobot,
    BuildClayRobot,
    BuildObsidianRobot,
    BuildGeodeRobot,
}

fn make_state_key(state: &State) -> StateKey {
    StateKey {
        ore: state.ore,
        clay: state.clay,
        obsidian: state.obsidian,
        ore_robots: state.ore_robots,
        clay_robots: state.clay_robots,
        obsidian_robots: state.obsidian_robots,
        geode_robots: state.geode_robots,
    }
}

#[test]
fn example_test() {
    let buffer = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"#
    .as_bytes();
    assert_eq!(compute_result(buffer, 1), (33, 56));
}
