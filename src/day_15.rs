use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!(
        "{:?}",
        compute_result(std::io::stdin().lock(), 2000000, 0, 4000000)
    );
}

fn compute_result(input: impl BufRead, y: i64, min: i64, max: i64) -> (usize, i64) {
    let sensors = parse_sensors(input);
    (
        count_positions_without_beacon(&sensors, y),
        find_tuning_frequency(&sensors, min, max),
    )
}

fn find_tuning_frequency(sensors: &[Sensor], min: i64, max: i64) -> i64 {
    for x in min..=max {
        let mut y = min;
        while y <= max {
            if let Some(dy) = sensors
                .iter()
                .filter(|v| manhattan_distance(&v.position, &(x, y)) <= v.radius)
                .map(|v| {
                    if y < v.position.1 {
                        v.radius - (v.position.0 - x).abs() + 1 + (v.position.1 - y)
                    } else {
                        v.radius - (v.position.0 - x).abs() + 1 - (y - v.position.1)
                    }
                })
                .max()
            {
                y += dy;
                continue;
            }
            return x * 4000000 + y;
        }
    }
    unreachable!()
}

fn count_positions_without_beacon(sensors: &[Sensor], y: i64) -> usize {
    let min_x = sensors
        .iter()
        .map(|v| v.position.0 - v.radius)
        .min()
        .unwrap();
    let max_x = sensors
        .iter()
        .map(|v| v.position.0 + v.radius)
        .max()
        .unwrap();
    (min_x..=max_x)
        .map(|x| {
            sensors
                .iter()
                .any(|v| v.beacon != (x, y) && manhattan_distance(&v.position, &(x, y)) <= v.radius)
                as usize
        })
        .sum()
}

fn manhattan_distance(a: &(i64, i64), b: &(i64, i64)) -> i64 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

fn parse_sensors(input: impl BufRead) -> Vec<Sensor> {
    input
        .lines()
        .map(|v| {
            let line = v.unwrap();
            let (sensor, beacon) = line.split_once(": closest beacon is at ").unwrap();
            let sensor_position = parse_position(sensor.strip_prefix("Sensor at ").unwrap());
            let beacon_position = parse_position(beacon);
            Sensor {
                position: sensor_position,
                beacon: beacon_position,
                radius: manhattan_distance(&sensor_position, &beacon_position),
            }
        })
        .collect()
}

fn parse_position(value: &str) -> (i64, i64) {
    let (x, y) = value.split_once(", ").unwrap();
    (
        i64::from_str(x.strip_prefix("x=").unwrap()).unwrap(),
        i64::from_str(y.strip_prefix("y=").unwrap()).unwrap(),
    )
}

struct Sensor {
    position: (i64, i64),
    beacon: (i64, i64),
    radius: i64,
}

#[test]
fn example_test() {
    let buffer = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#
    .as_bytes();
    assert_eq!(compute_result(buffer, 10, 0, 20), (26, 56000011));
}
