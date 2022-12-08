use std::io::BufRead;

fn main() {
    println!("{:?}", compute_result(std::io::stdin().lock()));
}

fn compute_result(input: impl BufRead) -> (usize, usize) {
    let (tree_map, width) = parse_tree_map(input);
    (
        count_visible_trees(&tree_map, width),
        find_highest_scenic_score(&tree_map, width),
    )
}

fn find_highest_scenic_score(tree_map: &[u8], width: usize) -> usize {
    let height = tree_map.len() / width;
    let mut max_scenic_score = 0;
    for i in 0..width {
        for j in 0..height {
            max_scenic_score = compute_scenic_score(i, j, tree_map, width).max(max_scenic_score);
        }
    }
    max_scenic_score
}

fn compute_scenic_score(x: usize, y: usize, tree_map: &[u8], width: usize) -> usize {
    let height = tree_map.len() / width;
    let max = tree_map[get_index(x, y, width)];
    let mut up = 0;
    for j in (0..y).rev() {
        up += 1;
        if tree_map[get_index(x, j, width)] >= max {
            break;
        }
    }
    let mut down = 0;
    for j in y + 1..height {
        down += 1;
        if tree_map[get_index(x, j, width)] >= max {
            break;
        }
    }
    let mut left = 0;
    for i in (0..x).rev() {
        left += 1;
        if tree_map[get_index(i, y, width)] >= max {
            break;
        }
    }
    let mut right = 0;
    for i in x + 1..width {
        right += 1;
        if tree_map[get_index(i, y, width)] >= max {
            break;
        }
    }
    up * down * right * left
}

fn count_visible_trees(tree_map: &[u8], width: usize) -> usize {
    let height = tree_map.len() / width;
    let mut visibility: Vec<bool> = std::iter::repeat(false).take(tree_map.len()).collect();
    for i in 0..width {
        visibility[get_index(i, 0, width)] = true;
        visibility[get_index(i, height - 1, width)] = true;
    }
    for j in 0..height {
        visibility[get_index(0, j, width)] = true;
        visibility[get_index(width - 1, j, width)] = true;
    }
    let columns = 1..width - 1;
    let rows = 1..height - 1;
    for i in columns.clone() {
        let mut max = tree_map[get_index(i, 0, width)];
        for j in rows.clone() {
            let index = get_index(i, j, width);
            if max < tree_map[index] {
                visibility[index] = true;
                max = tree_map[index];
            }
        }
        max = tree_map[get_index(i, height - 1, width)];
        for j in rows.clone().rev() {
            let index = get_index(i, j, width);
            if max < tree_map[index] {
                visibility[index] = true;
                max = tree_map[index];
            }
        }
    }
    for j in rows {
        let mut max = tree_map[get_index(0, j, width)];
        for i in columns.clone() {
            let index = get_index(i, j, width);
            if max < tree_map[index] {
                visibility[index] = true;
                max = tree_map[index];
            }
        }
        max = tree_map[get_index(width - 1, j, width)];
        for i in columns.clone().rev() {
            let index = get_index(i, j, width);
            if max < tree_map[index] {
                visibility[index] = true;
                max = tree_map[index];
            }
        }
    }
    visibility.iter().filter(|v| **v).count()
}

fn parse_tree_map(input: impl BufRead) -> (Vec<u8>, usize) {
    let mut width = 0;
    let mut values = Vec::new();
    for (i, symbol) in input.bytes().map(|v| v.unwrap()).enumerate() {
        if symbol != b'\n' {
            values.push(symbol);
        } else if width == 0 {
            width = i;
        }
    }
    (values, width)
}

fn get_index(i: usize, j: usize, width: usize) -> usize {
    i + j * width
}

#[test]
fn example_test() {
    let buffer = r#"30373
25512
65332
33549
35390
"#
    .as_bytes();
    assert_eq!(compute_result(buffer), (21, 8));
}
