use std::collections::HashMap;
use std::io::BufRead;
use std::str::{Bytes, FromStr};

fn main() {
    println!("{:?}", compute_result(std::io::stdin().lock()));
}

fn compute_result(input: impl BufRead) -> (i64, i64) {
    let equations = parse_equations(input);
    let mut values = HashMap::new();
    let mut expressions = HashMap::new();
    for (variable, definition) in equations.iter() {
        match definition {
            Definition::Value(value) => {
                values.insert(*variable, *value);
            }
            Definition::Expression(expression) => {
                expressions.insert(*variable, expression.clone());
            }
        }
    }
    (
        calculate_root(values.clone(), &expressions),
        find_proper_humn(values, &expressions),
    )
}

fn calculate_root(
    mut values: HashMap<Variable, i64>,
    expressions: &HashMap<Variable, Expression>,
) -> i64 {
    calculate(b"root", expressions, &mut values);
    values[b"root"]
}

fn find_proper_humn(
    values: HashMap<Variable, i64>,
    expressions: &HashMap<Variable, Expression>,
) -> i64 {
    let low = 0;
    let (low_first, low_second) = find_root_operands_with_humn(low, values.clone(), expressions);
    if low_first < low_second {
        find_proper_humn_impl(low, values, expressions, |a, b| a < b)
    } else {
        find_proper_humn_impl(low, values, expressions, |a, b| a > b)
    }
}

fn find_proper_humn_impl<F>(
    mut low: i64,
    values: HashMap<Variable, i64>,
    expressions: &HashMap<Variable, Expression>,
    less: F,
) -> i64
where
    F: Fn(i64, i64) -> bool,
{
    let mut high = 1;
    loop {
        let (high_first, high_second) =
            find_root_operands_with_humn(high, values.clone(), expressions);
        if !less(high_first, high_second) {
            break;
        }
        high *= 2;
    }
    let result = loop {
        let value = (low + high) / 2;
        let (first, second) = find_root_operands_with_humn(value, values.clone(), expressions);
        if first == second {
            break value;
        } else if less(first, second) {
            low = value;
        } else {
            high = value;
        }
    };
    let mut min_value = result;
    loop {
        let (first, second) =
            find_root_operands_with_humn(min_value - 1, values.clone(), expressions);
        if first != second {
            break min_value;
        }
        min_value -= 1;
    }
}

fn find_root_operands_with_humn(
    humn: i64,
    mut values: HashMap<Variable, i64>,
    expressions: &HashMap<Variable, Expression>,
) -> (i64, i64) {
    values.insert(*b"humn", humn);
    calculate(b"root", expressions, &mut values);
    (
        values[&expressions[b"root"].first],
        values[&expressions[b"root"].second],
    )
}

fn calculate(
    src: &Variable,
    expressions: &HashMap<Variable, Expression>,
    values: &mut HashMap<Variable, i64>,
) {
    let mut to_calculate = vec![*src];
    while let Some(variable) = to_calculate.pop() {
        if values.contains_key(&variable) {
            continue;
        }
        if let Some(expression) = expressions.get(&variable) {
            if let Some(first) = values.get(&expression.first) {
                if let Some(second) = values.get(&expression.second) {
                    let value = match expression.operation {
                        Operation::Sum => *first + *second,
                        Operation::Sub => *first - *second,
                        Operation::Mul => *first * *second,
                        Operation::Div => *first / *second,
                    };
                    values.insert(variable, value);
                } else {
                    to_calculate.push(variable);
                    to_calculate.push(expression.second);
                }
            } else {
                to_calculate.push(variable);
                to_calculate.push(expression.first);
                if !values.contains_key(&expression.second) {
                    to_calculate.push(expression.second);
                }
            }
        }
    }
}

fn parse_equations(input: impl BufRead) -> Vec<(Variable, Definition)> {
    input
        .lines()
        .map(|v| {
            let line = v.unwrap();
            let (variable, definition) = line.split_once(": ").unwrap();
            (
                make_variable(variable.bytes()),
                if let Ok(value) = i64::from_str(definition) {
                    Definition::Value(value)
                } else if let Some((first, second)) = definition.split_once(" + ") {
                    Definition::Expression(Expression {
                        first: make_variable(first.bytes()),
                        second: make_variable(second.bytes()),
                        operation: Operation::Sum,
                    })
                } else if let Some((first, second)) = definition.split_once(" - ") {
                    Definition::Expression(Expression {
                        first: make_variable(first.bytes()),
                        second: make_variable(second.bytes()),
                        operation: Operation::Sub,
                    })
                } else if let Some((first, second)) = definition.split_once(" * ") {
                    Definition::Expression(Expression {
                        first: make_variable(first.bytes()),
                        second: make_variable(second.bytes()),
                        operation: Operation::Mul,
                    })
                } else if let Some((first, second)) = definition.split_once(" / ") {
                    Definition::Expression(Expression {
                        first: make_variable(first.bytes()),
                        second: make_variable(second.bytes()),
                        operation: Operation::Div,
                    })
                } else {
                    unreachable!();
                },
            )
        })
        .collect()
}

fn make_variable(bytes: Bytes) -> Variable {
    let mut result = [0u8; 4];
    for (i, v) in bytes.enumerate() {
        result[i] = v;
    }
    result
}

type Variable = [u8; 4];

#[derive(Debug)]
enum Definition {
    Value(i64),
    Expression(Expression),
}

#[derive(Debug, Clone)]
struct Expression {
    first: Variable,
    second: Variable,
    operation: Operation,
}

#[derive(Debug, Clone)]
enum Operation {
    Sum,
    Sub,
    Mul,
    Div,
}

#[test]
fn example_test() {
    let buffer = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), (152, 301));
}
