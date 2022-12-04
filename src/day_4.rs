use std::io::BufRead;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() {
    println!("{:?}", count_assignment_pairs(std::io::stdin().lock()));
}

fn count_assignment_pairs(buffer: impl BufRead) -> (usize, usize) {
    let pairs = parse_pairs(buffer);
    (
        count_fully_overlapping_assignment_pairs(&pairs),
        count_partially_overlapping_assignment_pairs(&pairs),
    )
}

fn count_partially_overlapping_assignment_pairs(
    pairs: &[(RangeInclusive<u64>, RangeInclusive<u64>)],
) -> usize {
    pairs
        .iter()
        .filter(|(first, second)| overlaps(first, second))
        .count()
}

fn overlaps(a: &RangeInclusive<u64>, b: &RangeInclusive<u64>) -> bool {
    a.contains(b.start()) || a.contains(b.end()) || b.contains(a.start()) || b.contains(a.end())
}

fn count_fully_overlapping_assignment_pairs(
    pairs: &[(RangeInclusive<u64>, RangeInclusive<u64>)],
) -> usize {
    pairs
        .iter()
        .filter(|(first, second)| contains(first, second) || contains(second, first))
        .count()
}

fn contains(containing: &RangeInclusive<u64>, contained: &RangeInclusive<u64>) -> bool {
    containing.start() <= contained.start() && contained.end() <= containing.end()
}

fn parse_pairs(buffer: impl BufRead) -> Vec<(RangeInclusive<u64>, RangeInclusive<u64>)> {
    buffer.lines().map(|v| parse_pair(&v.unwrap())).collect()
}

fn parse_pair(value: &str) -> (RangeInclusive<u64>, RangeInclusive<u64>) {
    let (first, second) = value.split_once(',').unwrap();
    (parse_range(first), parse_range(second))
}

fn parse_range(value: &str) -> RangeInclusive<u64> {
    let (start, end) = value.split_once('-').unwrap();
    u64::from_str(start).unwrap()..=u64::from_str(end).unwrap()
}

#[test]
fn example_test() {
    let buffer = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
"#
    .as_bytes();
    assert_eq!(count_assignment_pairs(buffer), (2, 4));
}
