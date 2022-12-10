use std::io::BufRead;
use std::str::FromStr;

fn main() {
    let (total_signal_strength, image) = compute_result(std::io::stdin().lock());
    println!("{}\n{}", total_signal_strength, image);
}

fn compute_result(input: impl BufRead) -> (i64, String) {
    let instructions = parse_instructions(input);
    (
        sum_signal_strength_at(&instructions, &[20, 60, 100, 140, 180, 220]),
        render_image(&instructions, 40, 6),
    )
}

fn render_image(instructions: &[Instruction], width: usize, height: usize) -> String {
    let mut buffer: Vec<u8> = std::iter::repeat(b'.').take(width * height).collect();
    let mut sprite_pos = 1;
    let mut draw_pos: usize = 0;
    for instruction in instructions.iter() {
        let (count, next_sprite_pos) = execute_instruction(instruction, sprite_pos);
        for _ in 0..count {
            if (sprite_pos - 1..=sprite_pos + 1).contains(&((draw_pos % width) as i64)) {
                buffer[draw_pos] = b'#';
            }
            draw_pos += 1;
        }
        sprite_pos = next_sprite_pos;
    }
    let mut result = String::new();
    for i in 0..buffer.len() / width {
        result.extend(
            buffer[width * i..width * (i + 1)]
                .iter()
                .map(|v| *v as char),
        );
        result.push('\n');
    }
    result
}

fn sum_signal_strength_at(instructions: &[Instruction], mut positions: &[usize]) -> i64 {
    let mut x = 1;
    let mut total_signal_strength = 0;
    let mut cycle = 0;
    for instruction in instructions.iter() {
        let (count, next_x) = execute_instruction(instruction, x);
        for _ in 0..count {
            cycle += 1;
            if !positions.is_empty() && cycle == positions[0] {
                let signal_strength = x * cycle as i64;
                total_signal_strength += signal_strength;
                positions = &positions[1..];
            }
        }
        x = next_x;
    }
    total_signal_strength
}

fn execute_instruction(value: &Instruction, x: i64) -> (usize, i64) {
    match value {
        Instruction::Noop => (1, x),
        Instruction::Add(v) => (2, x + *v),
    }
}

fn parse_instructions(input: impl BufRead) -> Vec<Instruction> {
    input
        .lines()
        .map(|v| parse_instruction(&v.unwrap()))
        .collect()
}

fn parse_instruction(value: &str) -> Instruction {
    if let Some(suffix) = value.strip_prefix("addx ") {
        return Instruction::Add(i64::from_str(suffix).unwrap());
    }
    if value == "noop" {
        return Instruction::Noop;
    }
    unreachable!();
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Add(i64),
}

#[test]
fn example_test() {
    let buffer = r#"noop
addx 3
addx -5
"#
    .as_bytes();
    assert_eq!(compute_result(buffer).0, 0);
}

#[test]
fn example_test_1() {
    let buffer = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#
    .as_bytes();
    assert_eq!(
        compute_result(buffer),
        (
            13140,
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            .to_string()
        )
    );
}
