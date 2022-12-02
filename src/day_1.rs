use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!("{:?}", get_max_calories_per_elf(std::io::stdin().lock()));
}

fn get_max_calories_per_elf(buffer: impl BufRead) -> (u64, u64) {
    let mut max = 0;
    let mut sum = 0;
    let mut calories_per_elf = Vec::new();
    for line in buffer.lines().map(|v| v.unwrap()) {
        if line.is_empty() {
            if max < sum {
                max = sum;
            }
            calories_per_elf.push(sum);
            sum = 0;
        } else {
            sum += u64::from_str(line.as_str()).unwrap();
        }
    }
    if max < sum {
        max = sum;
    }
    calories_per_elf.push(sum);
    calories_per_elf.sort_by_key(|v| u64::MAX - v);
    (max, calories_per_elf.iter().take(3).sum())
}

#[test]
fn example_test() {
    let buffer = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
"#
    .as_bytes();
    assert_eq!(get_max_calories_per_elf(buffer), (24000, 45000));
}
