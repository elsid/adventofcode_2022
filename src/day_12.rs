use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::BufRead;

fn main() {
    println!("{:?}", find_shortest_path_lengths(std::io::stdin().lock()));
}

fn find_shortest_path_lengths(input: impl BufRead) -> (u64, u64) {
    let (map, src, dst) = parse_input(input);
    (
        find_shortest_path_length(src, dst, &map),
        map.values
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == b'a')
            .map(|(i, _)| find_shortest_path_length(i, dst, &map))
            .min()
            .unwrap(),
    )
}

fn find_shortest_path_length(src: usize, dst: usize, map: &Grid<u8>) -> u64 {
    let height = map.values.len() / map.width;
    let mut indices = BinaryHeap::new();
    let mut distances = Grid {
        values: std::iter::repeat(u64::MAX).take(map.values.len()).collect(),
        width: map.width,
    };
    distances.values[src] = 0;
    indices.push((Reverse(0), src));
    let mut try_push_neighbour = |cur, next, positions: &mut BinaryHeap<(Reverse<u64>, usize)>| {
        if (map.values[next] as i16 - map.values[cur] as i16) > 1 {
            return;
        }
        let new_length = distances.values[cur] + 1;
        if distances.values[next] <= new_length {
            return;
        }
        distances.values[next] = new_length;
        positions.push((Reverse(new_length), next));
    };
    while let Some((_, index)) = indices.pop() {
        if index == dst {
            break;
        }
        let (x, y) = map.position(index);
        if x > 0 {
            try_push_neighbour(index, map.index(x - 1, y), &mut indices);
        }
        if x < map.width - 1 {
            try_push_neighbour(index, map.index(x + 1, y), &mut indices);
        }
        if y > 0 {
            try_push_neighbour(index, map.index(x, y - 1), &mut indices);
        }
        if y < height - 1 {
            try_push_neighbour(index, map.index(x, y + 1), &mut indices);
        }
    }
    distances.values[dst]
}

fn parse_input(input: impl BufRead) -> (Grid<u8>, usize, usize) {
    let mut values = Vec::new();
    let mut width = 0;
    let mut src = (0, 0);
    let mut dst = (0, 0);
    for (y, line) in input.lines().map(|v| v.unwrap()).enumerate() {
        width = line.len();
        for (x, v) in line.bytes().enumerate() {
            match v {
                b'S' => {
                    values.push(b'a');
                    src = (x, y);
                }
                b'E' => {
                    values.push(b'z');
                    dst = (x, y);
                }
                v => values.push(v),
            }
        }
    }
    let grid = Grid { values, width };
    let src_index = grid.index(src.0, src.1);
    let dst_index = grid.index(dst.0, dst.1);
    (grid, src_index, dst_index)
}

struct Grid<T> {
    values: Vec<T>,
    width: usize,
}

impl<T> Grid<T> {
    fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    fn position(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }
}

#[test]
fn example_test() {
    let buffer = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#
    .as_bytes();
    assert_eq!(find_shortest_path_lengths(buffer), (31, 29));
}

#[test]
fn example_test_1() {
    let buffer = r#"SabcdefghijklmnopqrstuvwxyzE
"#
    .as_bytes();
    assert_eq!(find_shortest_path_lengths(buffer), (27, 26));
}

#[test]
fn example_test_2() {
    let buffer = r#"SabcdefghijklmnopqrstuvwxyzabcdefghijklmnopqrstuvwxyzE
"#
    .as_bytes();
    assert_eq!(find_shortest_path_lengths(buffer), (53, 26));
}

#[test]
fn example_test_3() {
    let buffer = r#"abefijmnqruvyz
ScdghklopstwxE"#
        .as_bytes();
    assert_eq!(find_shortest_path_lengths(buffer), (27, 26));
}
