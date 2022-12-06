use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    println!("{:?}", find_starts(std::io::stdin().lock()));
}

fn find_starts(mut input: impl BufRead) -> (usize, usize) {
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer).unwrap();
    (
        find_start_of_the_packet(&buffer),
        find_start_of_the_message(&buffer),
    )
}

fn find_start_of_the_packet(buffer: &[u8]) -> usize {
    find_first_unique_sequence(buffer, 4)
}

fn find_start_of_the_message(buffer: &[u8]) -> usize {
    find_first_unique_sequence(buffer, 14)
}

fn find_first_unique_sequence(buffer: &[u8], size: usize) -> usize {
    for i in 0..buffer.len() - size {
        if is_unique_sequence(&buffer[i..i + size]) {
            return i + size;
        }
    }
    unreachable!()
}

fn is_unique_sequence(values: &[u8]) -> bool {
    values.iter().copied().collect::<HashSet<u8>>().len() == values.len()
}

#[test]
fn example_test() {
    let buffer = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb
"#
    .as_bytes();
    assert_eq!(find_starts(buffer), (7, 19));
}

#[test]
fn example_test_1() {
    let buffer = r#"bvwbjplbgvbhsrlpgdmjqwftvncz
"#
    .as_bytes();
    assert_eq!(find_starts(buffer), (5, 23));
}

#[test]
fn example_test_2() {
    let buffer = r#"nppdvjthqldpwncqszvftbrmjlhg
"#
    .as_bytes();
    assert_eq!(find_starts(buffer), (6, 23));
}

#[test]
fn example_test_3() {
    let buffer = r#"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg
"#
    .as_bytes();
    assert_eq!(find_starts(buffer), (10, 29));
}

#[test]
fn example_test_4() {
    let buffer = r#"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw
"#
    .as_bytes();
    assert_eq!(find_starts(buffer), (11, 26));
}
