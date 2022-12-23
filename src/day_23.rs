use std::collections::{HashMap, HashSet};
use std::io::BufRead;
use std::ops::RangeInclusive;

fn main() {
    println!("{:?}", compute_result(std::io::stdin().lock()));
}

fn compute_result(input: impl BufRead) -> (usize, usize) {
    let mut map = parse_map(input);
    (
        find_number_of_empty_tiles(map.clone()),
        move_elves(&mut map, usize::MAX),
    )
}

fn find_number_of_empty_tiles(mut map: HashSet<Position>) -> usize {
    move_elves(&mut map, 10);
    let min_x = map.iter().map(|v| v.0).min().unwrap();
    let max_x = map.iter().map(|v| v.0).max().unwrap();
    let min_y = map.iter().map(|v| v.1).min().unwrap();
    let max_y = map.iter().map(|v| v.1).max().unwrap();
    (max_x - min_x + 1) as usize * (max_y - min_y + 1) as usize - map.len()
}

fn move_elves(map: &mut HashSet<Position>, max_rounds: usize) -> usize {
    let mut elves: Vec<Position> = map.iter().copied().collect();
    let mut next_positions: Vec<Option<Position>> =
        std::iter::repeat(None).take(elves.len()).collect();
    let mut rounds = 0;
    for _ in 0..max_rounds {
        for (index, elf) in elves.iter_mut().enumerate() {
            if !is_another_elf_at(*elf, -1..=1, -1..=1, map) {
                next_positions[index] = None;
                continue;
            }
            next_positions[index] = find_first_valid_suggestion(rounds, *elf, map);
        }
        let mut next_positions_counter = HashMap::<Position, usize>::new();
        for next_position in next_positions.iter().flatten() {
            next_positions_counter
                .entry(*next_position)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
        for (elf, next_position) in elves.iter_mut().zip(next_positions.iter_mut()) {
            if let Some(position) = next_position {
                if next_positions_counter[position] == 1 {
                    map.remove(elf);
                    *elf = *position;
                    map.insert(*elf);
                }
            }
        }
        rounds += 1;
        if next_positions.iter().all(|v| v.is_none()) {
            break;
        }
    }
    rounds
}

fn find_first_valid_suggestion(
    first_direction: usize,
    position: Position,
    map: &HashSet<Position>,
) -> Option<Position> {
    for direction_index in 0..DIRECTIONS.len() {
        match DIRECTIONS[(direction_index + first_direction) % DIRECTIONS.len()] {
            Direction::North => {
                if !is_another_elf_at(position, -1..=1, -1..=-1, map) {
                    return Some((position.0, position.1 - 1));
                }
            }
            Direction::South => {
                if !is_another_elf_at(position, -1..=1, 1..=1, map) {
                    return Some((position.0, position.1 + 1));
                }
            }
            Direction::West => {
                if !is_another_elf_at(position, -1..=-1, -1..=1, map) {
                    return Some((position.0 - 1, position.1));
                }
            }
            Direction::East => {
                if !is_another_elf_at(position, 1..=1, -1..=1, map) {
                    return Some((position.0 + 1, position.1));
                }
            }
        }
    }
    None
}

fn is_another_elf_at(
    position: Position,
    dx_range: RangeInclusive<i16>,
    dy_range: RangeInclusive<i16>,
    map: &HashSet<Position>,
) -> bool {
    for dx in dx_range {
        for dy in dy_range.clone() {
            if (dx, dy) != (0, 0) && map.contains(&(position.0 + dx, position.1 + dy)) {
                return true;
            }
        }
    }
    false
}

const DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::South,
    Direction::West,
    Direction::East,
];

#[derive(Clone, Debug)]
enum Direction {
    North,
    South,
    West,
    East,
}

type Position = (i16, i16);

fn parse_map(input: impl BufRead) -> HashSet<Position> {
    let mut map = HashSet::new();
    for (y, line) in input.lines().map(|v| v.unwrap()).enumerate() {
        for (x, value) in line.bytes().enumerate() {
            if value == b'#' {
                map.insert((x as i16, y as i16));
            }
        }
    }
    map
}

#[test]
fn example_test() {
    let buffer = r#"..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), (110, 20));
}

#[test]
fn example_test_0() {
    let buffer = r#".....
..##.
..#..
.....
..##.
.....
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), (25, 4));
}
