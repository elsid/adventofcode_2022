use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!(
        "{:?}",
        compute_result(std::io::stdin().lock(), &CubeLayout0)
    );
}

fn compute_result<L>(input: impl BufRead, layout: &L) -> (usize, usize)
where
    L: CubeLayout,
{
    let (rows, path) = parse_input(input);
    (
        compute_plane_password_with_wrapping(&rows, &path),
        compute_cube_password(&rows, &path, layout),
    )
}

fn compute_cube_password<L>(rows: &[Row], path: &Path, layout: &L) -> usize
where
    L: CubeLayout,
{
    let max_column = rows.iter().map(|v| v.shift + v.values.len()).max().unwrap();
    let face_size = if max_column > rows.len() {
        max_column / 4
    } else {
        rows.len() / 4
    };
    let mut x = rows[0].shift;
    let mut y = 0;
    let mut direction = Direction::Right;
    for point in path.iter() {
        match point {
            Point::Steps(steps) => {
                for _ in 0..*steps {
                    let (new_x, new_y, new_direction) =
                        move_by_cube_surface(x, y, direction, face_size, layout);
                    if matches!(rows[new_y].values[new_x - rows[new_y].shift], Tile::Wall) {
                        break;
                    }
                    x = new_x;
                    y = new_y;
                    direction = new_direction;
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
    1000 * (y + 1) + 4 * (x + 1) + direction as usize
}

fn move_by_cube_surface<L>(
    x: usize,
    y: usize,
    direction: Direction,
    face_size: usize,
    layout: &L,
) -> (usize, usize, Direction)
where
    L: CubeLayout,
{
    let face_x = x / face_size;
    let face_y = y / face_size;
    let faces = Faces {
        values: layout.faces(),
    };
    let face = faces.find_face_index(face_x, face_y);
    let (next_x, next_y) = match direction {
        Direction::Right => (x.overflowing_add(1).0, y),
        Direction::Down => (x, y.overflowing_add(1).0),
        Direction::Left => (x.overflowing_sub(1).0, y),
        Direction::Up => (x, y.overflowing_sub(1).0),
    };
    if (face_x * face_size..(face_x + 1) * face_size).contains(&next_x)
        && (face_y * face_size..(face_y + 1) * face_size).contains(&next_y)
    {
        return (next_x, next_y, direction);
    }
    let (next_face, transform_x, transform_y, next_direction) =
        layout.get_next_face(face, direction);
    let (result_x, result_y) =
        faces.transform(x, y, next_face, face_size, transform_x, transform_y);
    (result_x, result_y, next_direction)
}

#[derive(Copy, Clone, Debug)]
enum TransformX {
    Id,
    Left,
    Right,
    FromY,
}

#[derive(Copy, Clone, Debug)]
enum TransformY {
    Id,
    Inv,
    Top,
    Bottom,
    FromX,
}

trait CubeLayout {
    fn faces(&self) -> &[(usize, usize); 6];

    fn get_next_face(
        &self,
        face: usize,
        direction: Direction,
    ) -> (usize, TransformX, TransformY, Direction);
}

struct CubeLayout0;

impl CubeLayout for CubeLayout0 {
    fn faces(&self) -> &[(usize, usize); 6] {
        &[(1, 0), (2, 0), (1, 1), (0, 2), (1, 2), (0, 3)]
    }

    fn get_next_face(
        &self,
        face: usize,
        direction: Direction,
    ) -> (usize, TransformX, TransformY, Direction) {
        match face {
            0 => match direction {
                Direction::Right => (1, TransformX::Left, TransformY::Id, direction),
                Direction::Down => (2, TransformX::Id, TransformY::Top, direction),
                Direction::Left => (3, TransformX::Left, TransformY::Inv, Direction::Right),
                Direction::Up => (5, TransformX::Left, TransformY::FromX, Direction::Right),
            },
            1 => match direction {
                Direction::Right => (4, TransformX::Right, TransformY::Inv, Direction::Left),
                Direction::Down => (2, TransformX::Right, TransformY::FromX, Direction::Left),
                Direction::Left => (0, TransformX::Right, TransformY::Id, direction),
                Direction::Up => (5, TransformX::Id, TransformY::Bottom, Direction::Up),
            },
            2 => match direction {
                Direction::Right => (1, TransformX::FromY, TransformY::Bottom, Direction::Up),
                Direction::Down => (4, TransformX::Id, TransformY::Top, direction),
                Direction::Left => (3, TransformX::FromY, TransformY::Top, Direction::Down),
                Direction::Up => (0, TransformX::Id, TransformY::Bottom, direction),
            },
            3 => match direction {
                Direction::Right => (4, TransformX::Left, TransformY::Id, direction),
                Direction::Down => (5, TransformX::Id, TransformY::Top, direction),
                Direction::Left => (0, TransformX::Left, TransformY::Inv, Direction::Right),
                Direction::Up => (2, TransformX::Left, TransformY::FromX, Direction::Right),
            },
            4 => match direction {
                Direction::Right => (1, TransformX::Right, TransformY::Inv, Direction::Left),
                Direction::Down => (5, TransformX::Right, TransformY::FromX, Direction::Left),
                Direction::Left => (3, TransformX::Right, TransformY::Id, direction),
                Direction::Up => (2, TransformX::Id, TransformY::Bottom, direction),
            },
            5 => match direction {
                Direction::Right => (4, TransformX::FromY, TransformY::Bottom, Direction::Up),
                Direction::Down => (1, TransformX::Id, TransformY::Top, Direction::Down),
                Direction::Left => (0, TransformX::FromY, TransformY::Top, Direction::Down),
                Direction::Up => (3, TransformX::Id, TransformY::Bottom, direction),
            },
            _ => unreachable!(),
        }
    }
}

struct Faces<'a> {
    values: &'a [(usize, usize); 6],
}

impl Faces<'_> {
    fn find_face_index(&self, x: usize, y: usize) -> usize {
        self.values
            .iter()
            .enumerate()
            .find(|(_, v)| **v == (x, y))
            .map(|(v, _)| v)
            .unwrap()
    }

    fn transform(
        &self,
        x: usize,
        y: usize,
        next_face: usize,
        face_size: usize,
        transform_x: TransformX,
        transform_y: TransformY,
    ) -> (usize, usize) {
        (
            match transform_x {
                TransformX::Id => self.values[next_face].0 * face_size + x % face_size,
                TransformX::Left => self.values[next_face].0 * face_size,
                TransformX::Right => (self.values[next_face].0 + 1) * face_size - 1,
                TransformX::FromY => self.values[next_face].0 * face_size + y % face_size,
            },
            match transform_y {
                TransformY::Id => self.values[next_face].1 * face_size + y % face_size,
                TransformY::Inv => (self.values[next_face].1 + 1) * face_size - 1 - y % face_size,
                TransformY::Top => self.values[next_face].1 * face_size,
                TransformY::Bottom => (self.values[next_face].1 + 1) * face_size - 1,
                TransformY::FromX => self.values[next_face].1 * face_size + x % face_size,
            },
        )
    }
}

fn compute_plane_password_with_wrapping(rows: &[Row], path: &Path) -> usize {
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

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

#[derive(Clone, Copy)]
#[repr(u8)]
enum Tile {
    Empty,
    Wall,
}

#[cfg(test)]
struct CubeLayoutExample;

#[cfg(test)]
impl CubeLayout for CubeLayoutExample {
    fn faces(&self) -> &[(usize, usize); 6] {
        &[(2, 0), (0, 1), (1, 1), (2, 1), (2, 2), (3, 2)]
    }

    fn get_next_face(
        &self,
        face: usize,
        direction: Direction,
    ) -> (usize, TransformX, TransformY, Direction) {
        (face, TransformX::Id, TransformY::Id, direction)
    }
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
    assert_eq!(compute_result(buffer, &CubeLayoutExample).0, 6032);
}

#[test]
fn move_by_cube_surface_with_layout0() {
    let face_size = 4;
    for initial_direction in [
        Direction::Down,
        Direction::Right,
        Direction::Up,
        Direction::Left,
    ] {
        let layout = CubeLayout0;
        for (face_x, face_y) in layout.faces().iter() {
            for shift in 0..face_size {
                let initial_x = face_x * face_size + shift;
                let initial_y = face_y * face_size + shift;
                let mut x = initial_x;
                let mut y = initial_y;
                let mut direction = initial_direction;
                for _ in 0..4 * face_size {
                    (x, y, direction) = move_by_cube_surface(x, y, direction, face_size, &layout);
                }
                assert_eq!(
                    (x, y, direction),
                    (initial_x, initial_y, initial_direction),
                    "face_x={} face_y={} shift={}",
                    face_x,
                    face_y,
                    shift
                );
            }
        }
    }
}
