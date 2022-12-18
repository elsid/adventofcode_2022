use std::cmp::Reverse;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::io::BufRead;
use std::str::FromStr;

fn main() {
    println!("{:?}", compute_result(std::io::stdin().lock()));
}

type Cube = (i8, i8, i8);

fn compute_result(input: impl BufRead) -> (usize, usize) {
    let cubes = parse_cubes(input);
    let mut free = HashMap::new();
    (
        count_unconnected_cube_sides(&cubes, &mut free),
        calculate_exterior_surface_area(&cubes, &free),
    )
}

fn count_unconnected_cube_sides(cubes: &HashSet<Cube>, free: &mut HashMap<Cube, usize>) -> usize {
    let mut try_add_free = |x, y, z| {
        if !cubes.contains(&(x, y, z)) {
            free.entry((x, y, z)).and_modify(|v| *v += 1).or_insert(1);
            1
        } else {
            0
        }
    };
    cubes
        .iter()
        .map(|&(x, y, z)| {
            try_add_free(x + 1, y, z)
                + try_add_free(x - 1, y, z)
                + try_add_free(x, y + 1, z)
                + try_add_free(x, y - 1, z)
                + try_add_free(x, y, z + 1)
                + try_add_free(x, y, z - 1)
        })
        .sum()
}

fn calculate_exterior_surface_area(cubes: &HashSet<Cube>, free: &HashMap<Cube, usize>) -> usize {
    let min_x = cubes.iter().map(|&(x, _, _)| x).min().unwrap() - 1;
    let min_y = cubes.iter().map(|&(_, y, _)| y).min().unwrap() - 1;
    let min_z = cubes.iter().map(|&(_, _, z)| z).min().unwrap() - 1;
    let max_x = cubes.iter().map(|&(x, _, _)| x).max().unwrap() + 1;
    let max_y = cubes.iter().map(|&(_, y, _)| y).max().unwrap() + 1;
    let max_z = cubes.iter().map(|&(_, _, z)| z).max().unwrap() + 1;
    let x_range = min_x..=max_x;
    let y_range = min_y..=max_y;
    let z_range = min_z..=max_z;
    let src = (min_x, min_y, min_z);
    let mut to_visit = BinaryHeap::new();
    to_visit.push((Reverse(0), src));
    let mut distances: HashMap<Cube, usize> = HashMap::new();
    distances.insert(src, 0);
    let mut result = 0;
    while let Some((_, (x, y, z))) = to_visit.pop() {
        if let Some(count) = free.get(&(x, y, z)) {
            result += *count;
        }
        let distance = distances[&(x, y, z)];
        let mut try_push = |position: Cube| {
            if !x_range.contains(&position.0)
                || !y_range.contains(&position.1)
                || !z_range.contains(&position.2)
                || cubes.contains(&position)
            {
                return;
            }
            let new_distance = distance + 1;
            match distances.entry(position) {
                Entry::Occupied(mut v) => {
                    if *v.get() > new_distance {
                        v.insert(new_distance);
                        to_visit.push((Reverse(new_distance), position));
                    }
                }
                Entry::Vacant(v) => {
                    v.insert(new_distance);
                    to_visit.push((Reverse(new_distance), position));
                }
            }
        };
        try_push((x + 1, y, z));
        try_push((x - 1, y, z));
        try_push((x, y + 1, z));
        try_push((x, y - 1, z));
        try_push((x, y, z + 1));
        try_push((x, y, z - 1));
    }
    result
}

fn parse_cubes(input: impl BufRead) -> HashSet<Cube> {
    input
        .lines()
        .map(|v| {
            let line = v.unwrap();
            let (x, tail) = line.split_once(',').unwrap();
            let (y, z) = tail.split_once(',').unwrap();
            (
                i8::from_str(x).unwrap(),
                i8::from_str(y).unwrap(),
                i8::from_str(z).unwrap(),
            )
        })
        .collect()
}

#[test]
fn example_test_0() {
    let buffer = r#"1,1,1
2,1,1
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), (10, 10));
}

#[test]
fn example_test() {
    let buffer = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), (64, 58));
}
