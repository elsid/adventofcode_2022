use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!("{:?}", count_sand_positions(std::io::stdin().lock()));
}

fn count_sand_positions(input: impl BufRead) -> (usize, usize) {
    let grid = parse_map(input);
    (
        count_stable_sand_positions(grid.clone()),
        count_stable_sand_positions_with_floor(grid),
    )
}

const START_X: usize = 500;
const START_Y: usize = 0;

fn count_stable_sand_positions_with_floor(mut map: HashSet<(usize, usize)>) -> usize {
    let mut stable_sand_count = 0;
    let max_y = *map.iter().map(|(_, y)| y).max().unwrap() + 1;
    loop {
        let mut x = START_X;
        let mut y = START_Y;
        loop {
            if y == max_y {
                break;
            }
            if !try_move_sand(&map, &mut x, &mut y) {
                break;
            }
        }
        map.insert((x, y));
        stable_sand_count += 1;
        if x == START_X && y == START_Y {
            break;
        }
    }
    stable_sand_count
}

fn count_stable_sand_positions(mut map: HashSet<(usize, usize)>) -> usize {
    let mut stable_sand_count = 0;
    let max_y = *map.iter().map(|(_, y)| y).max().unwrap();
    loop {
        let mut x = START_X;
        let mut y = START_Y;
        let contains = loop {
            if !try_move_sand(&map, &mut x, &mut y) {
                break true;
            }
            if y > max_y {
                break false;
            }
        };
        if !contains {
            break;
        }
        map.insert((x, y));
        stable_sand_count += 1;
    }
    stable_sand_count
}

fn try_move_sand(map: &HashSet<(usize, usize)>, x: &mut usize, y: &mut usize) -> bool {
    if !map.contains(&(*x, *y + 1)) {
        *y += 1;
        true
    } else if !map.contains(&(*x - 1, *y + 1)) {
        *x -= 1;
        *y += 1;
        true
    } else if !map.contains(&(*x + 1, *y + 1)) {
        *x += 1;
        *y += 1;
        true
    } else {
        false
    }
}

fn parse_map(input: impl BufRead) -> HashSet<(usize, usize)> {
    let mut rocks = HashSet::new();
    for line in input.lines().map(|v| v.unwrap()) {
        let split = line.split(" -> ");
        for (a, b) in split.clone().zip(split.skip(1)) {
            let (ax, ay) = parse_position(a);
            let (bx, by) = parse_position(b);
            if ax == bx {
                for y in ay.min(by)..=ay.max(by) {
                    rocks.insert((ax, y));
                }
            } else {
                for x in ax.min(bx)..=ax.max(bx) {
                    rocks.insert((x, ay));
                }
            }
        }
    }
    rocks
}

fn parse_position(value: &str) -> (usize, usize) {
    let (x, y) = value.split_once(',').unwrap();
    (usize::from_str(x).unwrap(), usize::from_str(y).unwrap())
}

#[test]
fn example_test() {
    let buffer = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#
    .as_bytes();
    assert_eq!(count_sand_positions(buffer), (24, 93));
}
