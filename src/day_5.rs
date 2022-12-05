use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!("{:?}", rearrange_crates(std::io::stdin().lock()));
}

fn rearrange_crates(buffer: impl BufRead) -> (String, String) {
    let (mut stacks, commands) = parse_input(buffer);
    let mut stacks_one_by_one = stacks.clone();
    move_crates_by_one(&commands, &mut stacks_one_by_one);
    move_crates_at_once(&commands, &mut stacks);
    (get_top_crates(&stacks_one_by_one), get_top_crates(&stacks))
}

fn get_top_crates(stacks: &[Vec<u8>]) -> String {
    String::from_utf8(stacks.iter().filter_map(|v| v.last()).copied().collect()).unwrap()
}

fn move_crates_by_one(commands: &[Command], stacks: &mut [Vec<u8>]) {
    for command in commands {
        for _ in 0..command.amount {
            let value = stacks[command.src - 1].pop().unwrap();
            stacks[command.dst - 1].push(value);
        }
    }
}

fn move_crates_at_once(commands: &[Command], stacks: &mut [Vec<u8>]) {
    for command in commands {
        let mut buffer = Vec::new();
        for _ in 0..command.amount {
            buffer.push(stacks[command.src - 1].pop().unwrap());
        }
        while let Some(value) = buffer.pop() {
            stacks[command.dst - 1].push(value);
        }
    }
}

fn parse_input(buffer: impl BufRead) -> (Vec<Vec<u8>>, Vec<Command>) {
    let mut stack_lines: Vec<String> = Vec::new();
    let mut stacks = Vec::new();
    let mut commands = Vec::new();
    for line in buffer.lines().map(|v| v.unwrap()) {
        if !stacks.is_empty() {
            commands.push(parse_command(&line));
        } else if line.is_empty() {
            stacks = (0..stack_lines[stack_lines.len() - 1]
                .split(' ')
                .filter(|v| !v.is_empty())
                .count())
                .map(|_| Vec::new())
                .collect();
            for stack_line in stack_lines
                .iter()
                .rev()
                .map(|v| v.bytes().collect::<Vec<u8>>())
            {
                for (i, stack) in stacks.iter_mut().enumerate() {
                    let crate_index = (i + 1) * 4 - 3;
                    if crate_index < stack_line.len() && stack_line[crate_index] != b' ' {
                        stack.push(stack_line[crate_index]);
                    }
                }
            }
        } else {
            stack_lines.push(line);
        }
    }
    (stacks, commands)
}

fn parse_command(value: &str) -> Command {
    let after_move = value.split_once("move ").unwrap().1;
    let (amount, after_from) = after_move.split_once(" from ").unwrap();
    let (src, dst) = after_from.split_once(" to ").unwrap();
    Command {
        amount: usize::from_str(amount).unwrap(),
        src: usize::from_str(src).unwrap(),
        dst: usize::from_str(dst).unwrap(),
    }
}

#[derive(Debug)]
struct Command {
    amount: usize,
    src: usize,
    dst: usize,
}

#[test]
fn example_test() {
    let buffer = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#
    .as_bytes();
    assert_eq!(
        rearrange_crates(buffer),
        ("CMZ".to_string(), "MCD".to_string())
    );
}
