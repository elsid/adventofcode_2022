use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!("{:?}", compute_result(std::io::stdin().lock()));
}

fn compute_result(input: impl BufRead) -> usize {
    let (rows, path) = parse_input(input);
    let max_column = rows.iter().map(|v| v.shift + v.values.len()).max().unwrap();
    let columns = (0..max_column)
        .map(|column| {
            (
                rows.iter()
                    .enumerate()
                    .find(|(_, v)| (v.shift..v.shift + v.values.len()).contains(&column))
                    .map(|(v, _)| v)
                    .unwrap(),
                rows.iter()
                    .enumerate()
                    .rev()
                    .find(|(_, v)| (v.shift..v.shift + v.values.len()).contains(&column))
                    .map(|(v, _)| v)
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();
    compute_plane_password_with_wrapping(&rows, &columns, &path)
}

fn compute_plane_password_with_wrapping(
    rows: &[Row],
    columns: &[(usize, usize)],
    path: &Path,
) -> usize {
    let mut direction = Direction::Right;
    let mut column = rows[0].shift;
    let mut row = 0;
    for point in path.iter() {
        match point {
            Point::Steps(steps) => {
                for _ in 0..*steps {
                    let (new_column, new_row) = match direction {
                        Direction::Right => {
                            if column >= rows[row].shift + rows[row].values.len() - 1 {
                                (rows[row].shift, row)
                            } else {
                                (column + 1, row)
                            }
                        }
                        Direction::Down => {
                            if row >= columns[column].1 {
                                (column, columns[column].0)
                            } else {
                                (column, row + 1)
                            }
                        }
                        Direction::Left => {
                            if column <= rows[row].shift {
                                (rows[row].shift + rows[row].values.len() - 1, row)
                            } else {
                                (column - 1, row)
                            }
                        }
                        Direction::Up => {
                            if row <= columns[column].0 {
                                (column, columns[column].1)
                            } else {
                                (column, row - 1)
                            }
                        }
                    };
                    if matches!(
                        rows[new_row].values[new_column - rows[new_row].shift],
                        Tile::Wall
                    ) {
                        break;
                    }
                    column = new_column;
                    row = new_row;
                }
            }
            Point::Right => {
                direction = match direction {
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Up => Direction::Right,
                }
            }
            Point::Left => {
                direction = match direction {
                    Direction::Right => Direction::Up,
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Right,
                }
            }
        }
    }
    1000 * (row + 1) + 4 * (column + 1) + direction as usize
}

fn parse_input(input: impl BufRead) -> (Vec<Row>, Path) {
    let mut lines = Vec::new();
    let mut path = Vec::new();
    let mut parse_map = true;
    for line in input.lines().map(|v| v.unwrap()) {
        if line.is_empty() {
            parse_map = false;
            continue;
        }
        if parse_map {
            let mut shift = 0;
            let mut values = Vec::new();
            for value in line.bytes() {
                if value == b' ' {
                    shift += 1;
                } else {
                    values.push(match value {
                        b'.' => Tile::Empty,
                        b'#' => Tile::Wall,
                        _ => unreachable!(),
                    });
                }
            }
            lines.push(Row { values, shift });
        } else {
            let mut start = 0;
            for (i, value) in line.chars().enumerate() {
                match value {
                    'R' => {
                        path.push(Point::Steps(usize::from_str(&line[start..i]).unwrap()));
                        path.push(Point::Right);
                        start = i + 1;
                    }
                    'L' => {
                        path.push(Point::Steps(usize::from_str(&line[start..i]).unwrap()));
                        path.push(Point::Left);
                        start = i + 1;
                    }
                    _ => (),
                }
            }
            path.push(Point::Steps(
                usize::from_str(&line[start..line.len()]).unwrap(),
            ));
        }
    }
    (lines, path)
}

#[derive(Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

type Path = Vec<Point>;

#[derive(Debug)]
enum Point {
    Steps(usize),
    Right,
    Left,
}

#[derive(Default)]
struct Row {
    values: Vec<Tile>,
    shift: usize,
}

#[repr(u8)]
enum Tile {
    Empty,
    Wall,
}

#[test]
fn example_test() {
    let buffer = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), 6032);
}
