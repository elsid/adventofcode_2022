use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!("{:?}", figure_out_monkey_business(std::io::stdin().lock()));
}

fn figure_out_monkey_business(input: impl BufRead) -> (u64, u64) {
    let monkeys = parse_monkeys(input);
    let common_divisor: u64 = monkeys
        .iter()
        .map(|v| v.test.condition.divisible_by)
        .product();
    (
        simulate_monkeys(monkeys.clone(), 20, |v| v / 3),
        simulate_monkeys(monkeys, 10000, |v| v % common_divisor),
    )
}

fn simulate_monkeys<F>(mut monkeys: Vec<Monkey>, rounds: usize, reduce_worry_level: F) -> u64
where
    F: Fn(u64) -> u64,
{
    let mut inspections_by_monkey: Vec<u64> = std::iter::repeat(0).take(monkeys.len()).collect();
    for _ in 0..rounds {
        for monkey_index in 0..monkeys.len() {
            while let Some(worry_level) = monkeys[monkey_index].starting_items.pop() {
                inspections_by_monkey[monkey_index] += 1;
                let monkey = &monkeys[monkey_index];
                let new_worry_level =
                    reduce_worry_level(evaluate_expression(&monkey.operation, worry_level));
                let next = if new_worry_level % monkey.test.condition.divisible_by == 0 {
                    monkey.test.if_true
                } else {
                    monkey.test.if_false
                };
                monkeys[next].starting_items.push(new_worry_level);
            }
        }
    }
    inspections_by_monkey.sort();
    inspections_by_monkey.iter().rev().take(2).product()
}

fn evaluate_expression(expression: &Expression, old: u64) -> u64 {
    let first = get_operand_value(&expression.first, old);
    let second = get_operand_value(&expression.second, old);
    match expression.operation {
        Operation::Plus => first + second,
        Operation::Mult => first * second,
    }
}

fn get_operand_value(operand: &Operand, old: u64) -> u64 {
    match operand {
        Operand::Old => old,
        Operand::Const(v) => *v,
    }
}

fn parse_monkeys(input: impl BufRead) -> Vec<Monkey> {
    let mut result = Vec::new();
    for line in input.lines().map(|v| v.unwrap()) {
        if line.starts_with("Monkey") {
            result.push(Monkey {
                starting_items: Default::default(),
                operation: Expression {
                    first: Operand::Old,
                    operation: Operation::Plus,
                    second: Operand::Old,
                },
                test: Default::default(),
            });
        } else if let Some(starting_items) = line.strip_prefix("  Starting items: ") {
            result.last_mut().unwrap().starting_items = starting_items
                .split(", ")
                .map(|v| u64::from_str(v).unwrap())
                .collect();
        } else if let Some(expression) = line.strip_prefix("  Operation: new = ") {
            let mut split = expression.split(' ');
            let expression = &mut result.last_mut().unwrap().operation;
            expression.first = parse_operand(split.next().unwrap());
            expression.operation = parse_operation(split.next().unwrap());
            expression.second = parse_operand(split.next().unwrap());
        } else if let Some(divisible_by) = line.strip_prefix("  Test: divisible by ") {
            result.last_mut().unwrap().test.condition = Condition {
                divisible_by: u64::from_str(divisible_by).unwrap(),
            };
        } else if let Some(monkey) = line.strip_prefix("    If true: throw to monkey ") {
            result.last_mut().unwrap().test.if_true = usize::from_str(monkey).unwrap();
        } else if let Some(monkey) = line.strip_prefix("    If false: throw to monkey ") {
            result.last_mut().unwrap().test.if_false = usize::from_str(monkey).unwrap();
        }
    }
    result
}

fn parse_operand(value: &str) -> Operand {
    if value == "old" {
        Operand::Old
    } else {
        Operand::Const(u64::from_str(value).unwrap())
    }
}

fn parse_operation(value: &str) -> Operation {
    match value {
        "+" => Operation::Plus,
        "*" => Operation::Mult,
        _ => unreachable!(),
    }
}

#[derive(Clone, Debug)]
struct Monkey {
    starting_items: Vec<u64>,
    operation: Expression,
    test: Test,
}

#[derive(Clone, Debug)]
struct Expression {
    first: Operand,
    operation: Operation,
    second: Operand,
}

#[derive(Clone, Debug)]
enum Operand {
    Old,
    Const(u64),
}

#[derive(Clone, Debug)]
enum Operation {
    Plus,
    Mult,
}

#[derive(Default, Clone, Debug)]
struct Test {
    condition: Condition,
    if_true: usize,
    if_false: usize,
}

#[derive(Default, Clone, Debug)]
struct Condition {
    divisible_by: u64,
}

#[test]
fn example_test() {
    let buffer = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
"#
    .as_bytes();
    assert_eq!(figure_out_monkey_business(buffer), (10605, 2713310158));
}
