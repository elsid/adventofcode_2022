use std::collections::HashMap;
use std::io::BufRead;
use std::ops::Range;

fn main() {
    println!("{:?}", find_max_tower_height(std::io::stdin().lock()));
}

const LEVEL: [u8; 7] = [b'.'; 7];
const ROCK_TYPE_HEIGHTS: [usize; 5] = [1, 3, 3, 4, 2];
const ROCK_TYPE_WIDTHS: [i64; 5] = [4, 3, 3, 1, 2];
const ROCK_SHAPES: [&[&[u8]]; 5] = [
    &[b"####"],
    &[b".#.", b"###", b".#."],
    &[b"..#", b"..#", b"###"],
    &[b"#", b"#", b"#", b"#"],
    &[b"##", b"##"],
];
const COUNT1: usize = 2022;
const COUNT2: usize = 1000000000000;

fn find_max_tower_height(input: impl BufRead) -> (usize, usize) {
    let jet_directions = parse_jet_directions(input);
    (simulate_falling_rocks(&jet_directions, COUNT1).1, {
        let (levels, _, count_per_height) = simulate_falling_rocks(&jet_directions, COUNT1 * 3);
        let pattern = find_loop_pattern(&levels);
        let count_per_pattern = count_per_height[&pattern.end] - count_per_height[&pattern.start];
        let base_height = (COUNT2 / count_per_pattern) * (pattern.end - pattern.start);
        let repeats = COUNT2 / count_per_pattern;
        let base_count = count_per_pattern * repeats;
        let left_count = COUNT2 - base_count;
        base_height
            + count_per_height
                .iter()
                .find(|(_, n)| **n == left_count)
                .map(|(h, _)| h)
                .unwrap()
    })
}

fn simulate_falling_rocks(
    jet_directions: &[i64],
    count: usize,
) -> (Levels, usize, HashMap<usize, usize>) {
    let mut levels: Levels = std::iter::repeat(LEVEL).take(3).collect();
    let mut rock_type_generator = Generator::new(5);
    let mut jet_direction_generator = Generator::new(jet_directions.len());
    let mut occupied_lines = 0;
    let mut count_per_occupied_lines = HashMap::new();
    for i in 0..count {
        let rock_type = rock_type_generator.next();
        while occupied_lines + 3 + ROCK_TYPE_HEIGHTS[rock_type] > levels.len() {
            levels.push(LEVEL);
        }
        let mut rock_x = 2;
        let mut rock_y = (occupied_lines + 3 + ROCK_TYPE_HEIGHTS[rock_type] - 1) as i64;
        let max_x = LEVEL.len() as i64 - ROCK_TYPE_WIDTHS[rock_type];
        let max_y = ROCK_TYPE_HEIGHTS[rock_type] as i64 - 1;
        loop {
            let shift_x = jet_directions[jet_direction_generator.next()];
            if (0..=max_x).contains(&(rock_x + shift_x))
                && can_place(rock_x + shift_x, rock_y, rock_type, &levels)
            {
                rock_x += shift_x;
            }
            if rock_y <= max_y || !can_place(rock_x, rock_y - 1, rock_type, &levels) {
                render_rock(rock_x, rock_y, rock_type, &mut levels);
                occupied_lines = occupied_lines.max(rock_y as usize + 1);
                break;
            }
            rock_y -= 1;
        }
        count_per_occupied_lines.insert(occupied_lines, i + 1);
    }
    (levels, occupied_lines, count_per_occupied_lines)
}

fn find_loop_pattern(levels: &[[u8; 7]]) -> Range<usize> {
    for size in (2..=levels.len() / 3).rev() {
        for start in 0..size {
            if levels[start..start + size] == levels[start + size..start + 2 * size] {
                return start..start + size;
            }
        }
    }
    unreachable!()
}

type Levels = Vec<[u8; 7]>;

fn can_place(rock_x: i64, rock_y: i64, rock_type: usize, levels: &Levels) -> bool {
    for (y, line) in ROCK_SHAPES[rock_type].iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if *tile == b'.' {
                continue;
            }
            if levels[rock_y as usize - y][rock_x as usize + x] != b'.' {
                return false;
            }
        }
    }
    true
}

fn render_rock(rock_x: i64, rock_y: i64, rock_type: usize, levels: &mut Levels) {
    for (y, line) in ROCK_SHAPES[rock_type].iter().enumerate() {
        for (x, tile) in line.iter().enumerate() {
            if *tile == b'.' {
                continue;
            }
            levels[rock_y as usize - y][rock_x as usize + x] = *tile;
        }
    }
}

struct Generator {
    value: usize,
    max: usize,
}

impl Generator {
    fn new(max: usize) -> Self {
        Self { value: 0, max }
    }

    fn next(&mut self) -> usize {
        let result = self.value;
        self.value += 1;
        if self.value >= self.max {
            self.value = 0;
        }
        result
    }
}

fn parse_jet_directions(mut input: impl BufRead) -> Vec<i64> {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).unwrap();
    buffer
        .iter()
        .map(|v| match v {
            b'<' => -1,
            b'>' => 1,
            _ => unreachable!(),
        })
        .collect()
}

#[test]
fn example_test() {
    let buffer = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#.as_bytes();
    assert_eq!(find_max_tower_height(buffer), (3068, 1514285714288));
}
