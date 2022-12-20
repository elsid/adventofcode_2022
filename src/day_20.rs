use std::cmp::Ordering;
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!("{:?}", compute_result(std::io::stdin().lock()));
}

fn compute_result(input: impl BufRead) -> (i64, i64) {
    let numbers = parse_numbers(input);
    (
        get_groove_coordinates(&numbers, 1, 1),
        get_groove_coordinates(&numbers, 10, 811589153),
    )
}

fn get_groove_coordinates(numbers: &[i64], rounds: usize, key: i64) -> i64 {
    let zero_index = numbers
        .iter()
        .enumerate()
        .find(|(_, v)| **v == 0)
        .map(|(i, _)| i)
        .unwrap();

    let mut ordered_numbers = OrderedNumbers::new(numbers.len());

    for _ in 0..rounds {
        for (index, value) in numbers.iter().enumerate() {
            ordered_numbers.move_number(index, (*value * key) as isize);
        }
    }

    let zero_position = ordered_numbers.index_to_position[zero_index];

    numbers[ordered_numbers.position_to_index[(zero_position + 1000) % numbers.len()]] * key
        + numbers[ordered_numbers.position_to_index[(zero_position + 2000) % numbers.len()]] * key
        + numbers[ordered_numbers.position_to_index[(zero_position + 3000) % numbers.len()]] * key
}

struct OrderedNumbers {
    index_to_position: Vec<usize>,
    position_to_index: Vec<usize>,
}

impl OrderedNumbers {
    fn new(size: usize) -> Self {
        Self {
            index_to_position: (0..size).collect(),
            position_to_index: (0..size).collect(),
        }
    }

    fn move_number(&mut self, index: usize, shift: isize) {
        match shift.cmp(&0) {
            Ordering::Less => {
                for _ in 0..(-shift) as usize % (self.index_to_position.len() - 1) {
                    self.shift_left(index);
                }
            }
            Ordering::Greater => {
                for _ in 0..shift as usize % (self.index_to_position.len() - 1) {
                    self.shift_right(index);
                }
            }
            _ => (),
        }
    }

    fn shift_right(&mut self, index: usize) {
        self.swap_at(
            self.index_to_position[index],
            (self.index_to_position[index] + 1) % self.index_to_position.len(),
        );
    }

    fn shift_left(&mut self, index: usize) {
        self.swap_at(
            self.index_to_position[index],
            (self.index_to_position[index] + self.index_to_position.len() - 1)
                % self.index_to_position.len(),
        );
    }

    fn swap_at(&mut self, a: usize, b: usize) {
        self.index_to_position[self.position_to_index[a]] = b;
        self.index_to_position[self.position_to_index[b]] = a;
        self.position_to_index.swap(a, b);
    }
}

fn parse_numbers(input: impl BufRead) -> Vec<i64> {
    input
        .lines()
        .map(|v| i64::from_str(&v.unwrap()).unwrap())
        .collect()
}

#[test]
fn example_test() {
    let buffer = r#"1
2
-3
3
-2
0
4
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), (3, 1623178306));
}
