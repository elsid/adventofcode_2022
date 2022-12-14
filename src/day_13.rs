use serde_json::Value;
use std::cmp::Ordering;
use std::io::BufRead;

fn main() {
    println!("{:?}", compute_result(std::io::stdin().lock()));
}

fn compute_result(input: impl BufRead) -> (usize, usize) {
    let pairs = parse_input(input);
    (
        sum_indices_of_the_pairs_in_the_right_order(&pairs),
        multiply_indices_of_the_divider_packets(
            &pairs,
            &Packet::List(vec![Packet::List(vec![Packet::Number(2)])]),
            &Packet::List(vec![Packet::List(vec![Packet::Number(6)])]),
        ),
    )
}

fn multiply_indices_of_the_divider_packets(
    pairs: &[Pair],
    first_divider: &Packet,
    second_divider: &Packet,
) -> usize {
    let mut packets = Vec::new();
    for (l, r) in pairs.iter() {
        packets.push(l.clone());
        packets.push(r.clone());
    }
    packets.sort();
    let first_index = packets.partition_point(|v| v < first_divider);
    let second_index = packets.partition_point(|v| v < second_divider);
    (first_index + 1) * (second_index + 2)
}

fn sum_indices_of_the_pairs_in_the_right_order(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (l, r))| matches!(compare_packets(l, r), std::cmp::Ordering::Less))
        .map(|(i, _)| i + 1)
        .sum()
}

fn compare_packets(left: &Packet, right: &Packet) -> std::cmp::Ordering {
    match (left, right) {
        (Packet::Number(l), Packet::Number(r)) => l.cmp(r),
        (Packet::List(_), Packet::Number(_)) => {
            compare_packets(left, &Packet::List(vec![right.clone()]))
        }
        (Packet::Number(_), Packet::List(_)) => {
            compare_packets(&Packet::List(vec![left.clone()]), right)
        }
        (Packet::List(l), Packet::List(r)) => compare_lists(l, r),
    }
}

fn compare_lists(left_values: &[Packet], right_values: &[Packet]) -> std::cmp::Ordering {
    for (l, r) in left_values.iter().zip(right_values.iter()) {
        match compare_packets(l, r) {
            Ordering::Equal => (),
            v => return v,
        }
    }
    left_values.len().cmp(&right_values.len())
}

fn parse_input(input: impl BufRead) -> Vec<Pair> {
    let mut values = Vec::new();
    let mut first = None;
    let mut second = None;
    for line in input.lines().map(|v| v.unwrap()) {
        if line.is_empty() {
            values.push((first.take().unwrap(), second.take().unwrap()));
        } else if first.is_none() {
            first = Some(parse_list(&line));
        } else if second.is_none() {
            second = Some(parse_list(&line));
        }
    }
    if let (Some(first), Some(second)) = (first, second) {
        values.push((first, second));
    }
    values
}

fn parse_list(value: &str) -> Packet {
    make_packet(&serde_json::from_str::<Value>(value).unwrap())
}

fn make_packet(value: &Value) -> Packet {
    match value {
        Value::Number(v) => Packet::Number(v.as_u64().unwrap()),
        Value::Array(values) => Packet::List(values.iter().map(make_packet).collect()),
        _ => unreachable!(),
    }
}

type Pair = (Packet, Packet);

#[derive(Clone)]
enum Packet {
    Number(u64),
    List(Vec<Packet>),
}

impl PartialEq<Self> for Packet {
    fn eq(&self, other: &Self) -> bool {
        matches!(compare_packets(self, other), Ordering::Equal)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare_packets(self, other))
    }
}

impl Eq for Packet {}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        compare_packets(self, other)
    }
}

#[test]
fn example_test() {
    let buffer = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), (13, 140));
}
