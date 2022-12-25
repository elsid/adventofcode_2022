use std::io::BufRead;

fn main() {
    println!("{}", sum_snafu_numbers(std::io::stdin().lock()));
}

fn sum_snafu_numbers(input: impl BufRead) -> String {
    let snafu_numbers: Vec<String> = input.lines().map(|v| v.unwrap()).collect();
    let decimal_sum = snafu_numbers.iter().map(|v| snafu_to_decimal(v)).sum();
    decimal_to_snafu(decimal_sum).unwrap()
}

fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut result = 0;
    let mut power = 1;
    for digit in snafu.chars().rev() {
        let decimal = match digit {
            '=' => -2,
            '-' => -1,
            '0' => 0,
            '1' => 1,
            '2' => 2,
            _ => unreachable!(),
        };
        result += decimal * power;
        power *= 5;
    }
    result
}

fn decimal_to_snafu(mut decimal: i64) -> Result<String, String> {
    let mut base = find_snafu_base(decimal);
    let mut limit = get_limit(base / 5);
    let mut result = String::new();
    while base > 1 {
        let factor = find_factor(decimal, base, limit);
        decimal -= base * factor;
        base /= 5;
        limit -= 2 * base;
        result.push(decimal_to_snafu_digit(factor)?);
    }
    result.push(decimal_to_snafu_digit(decimal)?);
    Ok(result)
}

fn get_limit(mut base: i64) -> i64 {
    let mut result = 0;
    while base > 0 {
        result += 2 * base;
        base /= 5;
    }
    result
}

fn find_snafu_base(decimal: i64) -> i64 {
    if (-2..=2).contains(&decimal) {
        return 1;
    }
    let mut base = 5;
    loop {
        if decimal > 0 && decimal - 2 * base <= 2 || decimal < 0 && decimal + 2 * base >= -2 {
            break;
        }
        base *= 5;
    }
    base
}

fn find_factor(decimal: i64, base: i64, limit: i64) -> i64 {
    if decimal == 0 {
        return 0;
    }
    for factor in 0..=2 {
        if decimal > 0 && decimal - base * factor <= limit {
            return factor;
        } else if decimal < 0 && decimal + base * factor >= -limit {
            return -factor;
        }
    }
    unreachable!();
}

fn decimal_to_snafu_digit(decimal: i64) -> Result<char, String> {
    match decimal {
        -2 => Ok('='),
        -1 => Ok('-'),
        0 => Ok('0'),
        1 => Ok('1'),
        2 => Ok('2'),
        v => Err(format!("invalid SNAFU digit: {}", v)),
    }
}

#[test]
fn example_test() {
    let buffer = r#"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
"#
    .as_bytes();
    assert_eq!(sum_snafu_numbers(buffer), "2=-1=0");
}

#[test]
fn conversion_test() {
    const SAMPLES: &[(i64, &str)] = &[
        (-2, "="),
        (-1, "-"),
        (0, "0"),
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (11, "21"),
        (12, "22"),
        (13, "1=="),
        (14, "1=-"),
        (15, "1=0"),
        (16, "1=1"),
        (17, "1=2"),
        (18, "1-="),
        (19, "1--"),
        (20, "1-0"),
        (21, "1-1"),
        (22, "1-2"),
        (23, "10="),
        (24, "10-"),
        (25, "100"),
        (26, "101"),
        (27, "102"),
        (30, "110"),
        (31, "111"),
        (32, "112"),
        (33, "12="),
        (34, "12-"),
        (35, "120"),
        (36, "121"),
        (84, "1=2-"),
        (151, "1101"),
        (161, "1121"),
        (751, "11001"),
        (801, "11201"),
        (2017, "1=11=2"),
        (2018, "1=11-="),
        (2019, "1=11--"),
        (2020, "1=11-0"),
        (2021, "1=11-1"),
        (2022, "1=11-2"),
        (2023, "1=110="),
        (2024, "1=110-"),
        (2025, "1=1100"),
        (2026, "1=1101"),
        (2027, "1=1102"),
        (3751, "110001"),
        (4001, "112001"),
        (12345, "1-0---0"),
        (15625, "1000000"),
        (20001, "1120001"),
        (78125, "10000000"),
        (390625, "100000000"),
        (1953125, "1000000000"),
        (9765625, "10000000000"),
        (48828125, "100000000000"),
        (244140625, "1000000000000"),
        (292968750, "1100000000000"),
        (302734375, "1110000000000"),
        (304687500, "1111000000000"),
        (312500000, "1120000000000"),
        (312500001, "1120000000001"),
        (312500005, "1120000000010"),
        (312500025, "1120000000100"),
        (312500125, "1120000001000"),
        (312500625, "1120000010000"),
        (312503125, "1120000100000"),
        (312515625, "1120001000000"),
        (312578125, "1120010000000"),
        (312890625, "1120100000000"),
        (314453125, "1121000000000"),
        (314159263, "1121-1110-1=="),
        (314159264, "1121-1110-1=-"),
        (314159265, "1121-1110-1=0"),
        (488281250, "2000000000000"),
        (1220703125, "10000000000000"),
    ];

    for (decimal, snafu) in SAMPLES.iter() {
        assert_eq!(
            snafu_to_decimal(snafu),
            *decimal,
            "snafu to decimal: {} -> {}",
            snafu,
            decimal
        );
    }

    for (decimal, snafu) in SAMPLES.iter() {
        assert_eq!(
            decimal_to_snafu(*decimal).as_ref().map(|v| v.as_str()),
            Ok(*snafu),
            "decimal to snafu: {} -> {}",
            decimal,
            snafu
        );
    }
}

#[test]
fn conversion_test_1() {
    for value in -100000..100000 {
        let snafu = decimal_to_snafu(value).unwrap();
        assert_eq!(value, snafu_to_decimal(&snafu));
    }
}
