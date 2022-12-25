use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::io::BufRead;

fn main() {
    println!("{:?}", compute_result(std::io::stdin().lock()));
}

fn compute_result(input: impl BufRead) -> (u16, u16) {
    let map = parse_map(input);
    let src_x = (0..map.width)
        .find(|v| map.get_tile(*v as u8, 0) == b'.')
        .unwrap() as u8;
    let dst_x = (0..map.width)
        .find(|v| map.get_tile(*v as u8, (map.height - 1) as u8) == b'.')
        .unwrap() as u8;
    let src = (src_x, 0);
    let dst = (dst_x, (map.height - 1) as u8);
    let initial_state = State {
        position: src,
        steps: 0,
    };
    let mut blizzards_history = vec![map.blizzards.clone()];
    let mut busy_tiles_history = vec![map.tiles.iter().map(|v| *v == b'#').collect::<Vec<_>>()];
    let mut find_shortest_path = |state: State, dst: (u8, u8)| {
        find_shortest_path(
            state,
            dst,
            &map,
            &mut blizzards_history,
            &mut busy_tiles_history,
        )
    };
    (find_shortest_path(initial_state, dst).steps, {
        let state1 = find_shortest_path(initial_state, dst);
        let state2 = find_shortest_path(state1, src);
        find_shortest_path(state2, dst).steps
    })
}

const MAX_ITERATIONS: usize = 1_000_000;

fn find_shortest_path(
    initial_state: State,
    dst: (u8, u8),
    map: &Map,
    blizzards_history: &mut Vec<Vec<Blizzard>>,
    busy_tiles_history: &mut Vec<Vec<bool>>,
) -> State {
    let mut visited = HashMap::<State, usize>::new();
    visited.insert(initial_state, 0);
    let mut states = vec![initial_state];
    let mut incoming = BinaryHeap::new();
    incoming.push((Reverse(0), 0));
    let mut actions = Vec::new();
    let mut iterations = 0;
    while let Some((_, state_index)) = incoming.pop() {
        if states[state_index].position == dst {
            return states[state_index];
        }
        iterations += 1;
        if iterations >= MAX_ITERATIONS {
            break;
        }
        actions.clear();
        let depth = states[state_index].steps as usize;
        if depth + 1 >= blizzards_history.len() {
            let mut next_blizzards = blizzards_history[depth].clone();
            let mut next_busy_tiles = busy_tiles_history[depth].clone();
            move_blizzards(map, &mut next_busy_tiles, &mut next_blizzards);
            blizzards_history.push(next_blizzards);
            busy_tiles_history.push(next_busy_tiles);
        }
        let busy_tiles = &busy_tiles_history[depth + 1];
        generate_actions(&states[state_index], map, busy_tiles, &mut actions);
        for action in actions.iter() {
            let mut new_state = State {
                position: states[state_index].position,
                steps: states[state_index].steps,
            };
            apply_action(action, &mut new_state);
            let cost = get_cost(&new_state);
            match visited.entry(new_state) {
                Entry::Occupied(v) => {
                    if get_cost(&states[*v.get()]) > cost {
                        incoming.push((Reverse(cost + get_heuristic(dst, &new_state)), *v.get()));
                        states[*v.get()] = new_state;
                    }
                }
                Entry::Vacant(v) => {
                    v.insert(states.len());
                    incoming.push((Reverse(cost + get_heuristic(dst, &new_state)), states.len()));
                    states.push(new_state);
                }
            }
        }
    }
    unreachable!();
}

fn get_cost(state: &State) -> u64 {
    state.steps as u64
}

fn get_heuristic(dst: (u8, u8), state: &State) -> u64 {
    distance(dst, state.position)
}

fn distance(a: (u8, u8), b: (u8, u8)) -> u64 {
    (a.0 as i64 - b.0 as i64).unsigned_abs() + (a.1 as i64 - b.1 as i64).unsigned_abs()
}

fn apply_action(action: &Action, state: &mut State) {
    state.position = action.next_position;
    state.steps += 1;
}

fn move_blizzards(map: &Map, busy_tiles: &mut [bool], blizzards: &mut [Blizzard]) {
    for blizzard in blizzards.iter_mut() {
        busy_tiles[map.index(blizzard.position.0, blizzard.position.1)] = false;
        match blizzard.direction {
            Direction::Right => blizzard.position.0 += 1,
            Direction::Left => blizzard.position.0 -= 1,
            Direction::Down => blizzard.position.1 += 1,
            Direction::Up => blizzard.position.1 -= 1,
        }
        if map.get_tile(blizzard.position.0, blizzard.position.1) == b'#' {
            match blizzard.direction {
                Direction::Right => blizzard.position.0 = 1,
                Direction::Left => blizzard.position.0 = (map.width - 2) as u8,
                Direction::Down => blizzard.position.1 = 1,
                Direction::Up => blizzard.position.1 = (map.height - 2) as u8,
            }
        }
    }
    for blizzard in blizzards.iter() {
        busy_tiles[map.index(blizzard.position.0, blizzard.position.1)] = true;
    }
}

fn generate_actions(state: &State, map: &Map, busy_tiles: &[bool], actions: &mut Vec<Action>) {
    if state.position.0 > 0 && !busy_tiles[map.index(state.position.0 - 1, state.position.1)] {
        actions.push(Action {
            next_position: (state.position.0 - 1, state.position.1),
        });
    }
    if state.position.0 < (map.width - 1) as u8
        && !busy_tiles[map.index(state.position.0 + 1, state.position.1)]
    {
        actions.push(Action {
            next_position: (state.position.0 + 1, state.position.1),
        });
    }
    if state.position.1 > 0 && !busy_tiles[map.index(state.position.0, state.position.1 - 1)] {
        actions.push(Action {
            next_position: (state.position.0, state.position.1 - 1),
        });
    }
    if state.position.1 < (map.height - 1) as u8
        && !busy_tiles[map.index(state.position.0, state.position.1 + 1)]
    {
        actions.push(Action {
            next_position: (state.position.0, state.position.1 + 1),
        });
    }
    if !busy_tiles[map.index(state.position.0, state.position.1)] {
        actions.push(Action {
            next_position: state.position,
        });
    }
}

#[derive(Debug)]
struct Action {
    next_position: (u8, u8),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct State {
    position: (u8, u8),
    steps: u16,
}

fn parse_map(input: impl BufRead) -> Map {
    let mut width = 0;
    let mut height = 0;
    let mut blizzards = Vec::new();
    let mut tiles = Vec::new();
    for (y, line) in input.lines().map(|v| v.unwrap()).enumerate() {
        width = line.len();
        for (x, value) in line.bytes().enumerate() {
            let position = (x as u8, y as u8);
            match value {
                b'<' => {
                    blizzards.push(Blizzard {
                        position,
                        direction: Direction::Left,
                    });
                }
                b'>' => {
                    blizzards.push(Blizzard {
                        position,
                        direction: Direction::Right,
                    });
                }
                b'^' => {
                    blizzards.push(Blizzard {
                        position,
                        direction: Direction::Up,
                    });
                }
                b'v' => {
                    blizzards.push(Blizzard {
                        position,
                        direction: Direction::Down,
                    });
                }
                _ => (),
            }
            tiles.push(value);
        }
        height += 1;
    }
    Map {
        width,
        height,
        blizzards,
        tiles,
    }
}

struct Map {
    width: usize,
    height: usize,
    blizzards: Vec<Blizzard>,
    tiles: Vec<u8>,
}

impl Map {
    fn get_tile(&self, x: u8, y: u8) -> u8 {
        self.tiles[self.index(x, y)]
    }

    fn index(&self, x: u8, y: u8) -> usize {
        x as usize + y as usize * self.width
    }
}

#[derive(Debug, Clone, Copy)]
struct Blizzard {
    position: (u8, u8),
    direction: Direction,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}

#[test]
fn example_test() {
    let buffer = r#"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), (18, 54));
}
