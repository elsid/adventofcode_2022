use std::collections::{HashMap, HashSet};
use std::io::BufRead;

fn main() {
    println!("{:?}", get_all_sums(std::io::stdin().lock()));
}

fn get_all_sums(mut buffer: impl BufRead) -> (u64, u64) {
    let mut copied_buffer = Vec::new();
    buffer.read_to_end(&mut copied_buffer).unwrap();
    (
        sum_priorities_for_items_in_both_compartments(copied_buffer.as_slice()),
        sum_priorities_of_group_badges(copied_buffer.as_slice()),
    )
}

fn sum_priorities_of_group_badges(buffer: impl BufRead) -> u64 {
    let mut total_priority = 0;
    let mut items_counter: HashMap<u8, usize> = HashMap::new();
    for (i, line) in buffer.lines().map(|v| v.unwrap()).enumerate() {
        if i % 3 == 0 {
            for (item, count) in items_counter.iter() {
                if *count == 3 {
                    total_priority += get_item_priority(*item);
                    break;
                }
            }
            items_counter.clear();
        }
        let unique_items: HashSet<u8> = line.as_bytes().iter().copied().collect();
        for item in unique_items.iter() {
            *items_counter.entry(*item).or_insert(0usize) += 1;
        }
    }
    for (item, count) in items_counter.iter() {
        if *count == 3 {
            total_priority += get_item_priority(*item);
            break;
        }
    }
    total_priority
}

fn sum_priorities_for_items_in_both_compartments(buffer: impl BufRead) -> u64 {
    let mut total_priority = 0;
    for line in buffer.lines().map(|v| v.unwrap()) {
        let rucksack_size = line.as_bytes().len();
        let first_compartment: HashSet<u8> = line.as_bytes()[0..rucksack_size / 2]
            .iter()
            .copied()
            .collect();
        let second_compartment: HashSet<u8> = line.as_bytes()[rucksack_size / 2..rucksack_size]
            .iter()
            .copied()
            .collect();
        for item in first_compartment.iter() {
            if second_compartment.contains(item) {
                total_priority += get_item_priority(*item);
            }
        }
    }
    total_priority
}

fn get_item_priority(item: u8) -> u64 {
    if (b'a'..=b'z').contains(&item) {
        return (item - b'a') as u64 + 1;
    }
    if (b'A'..=b'Z').contains(&item) {
        return (item - b'A') as u64 + 27;
    }
    unreachable!();
}

#[test]
fn example_test() {
    let buffer = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
"#
    .as_bytes();
    assert_eq!(get_all_sums(buffer), (157, 70));
}
