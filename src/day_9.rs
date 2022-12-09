use std::collections::HashSet;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!(
        "{:?}",
        count_visited_positions_by_tails(std::io::stdin().lock())
    );
}

fn count_visited_positions_by_tails(input: impl BufRead) -> (usize, usize) {
    let movements = parse_movements(input);
    (
        count_visited_positions_by_tail(&movements, 2),
        count_visited_positions_by_tail(&movements, 10),
    )
}

fn count_visited_positions_by_tail(movements: &[Movement], size: usize) -> usize {
    let mut knots = std::iter::repeat((0i64, 0i64))
        .take(size)
        .collect::<Vec<_>>();
    let mut visited = HashSet::new();
    for movement in movements.iter() {
        for _ in 0..movement.length {
            match movement.direction {
                Direction::Up => knots[0].1 += 1,
                Direction::Down => knots[0].1 -= 1,
                Direction::Right => knots[0].0 += 1,
                Direction::Left => knots[0].0 -= 1,
            }
            for i in 1..knots.len() {
                knots[i] = adjust_position(&knots[i - 1], &knots[i]);
            }
            visited.insert(knots[knots.len() - 1]);
        }
    }
    visited.len()
}

fn adjust_position(head: &(i64, i64), tail: &(i64, i64)) -> (i64, i64) {
    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;
    if dx.abs() > 1 || dy.abs() > 1 {
        (tail.0 + dx.signum(), tail.1 + dy.signum())
    } else {
        *tail
    }
}

fn parse_movements(input: impl BufRead) -> Vec<Movement> {
    input.lines().map(|v| parse_movement(&v.unwrap())).collect()
}

fn parse_movement(value: &str) -> Movement {
    let (direction, length) = value.split_once(' ').unwrap();
    Movement {
        direction: match direction {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => unreachable!(),
        },
        length: u64::from_str(length).unwrap(),
    }
}

struct Movement {
    direction: Direction,
    length: u64,
}

enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[test]
fn example_test() {
    let buffer = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#
    .as_bytes();
    assert_eq!(count_visited_positions_by_tails(buffer), (13, 1));
}

#[test]
fn example_test_1() {
    let buffer = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#
    .as_bytes();
    assert_eq!(count_visited_positions_by_tails(buffer), (88, 36));
}
