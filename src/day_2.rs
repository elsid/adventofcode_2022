use std::io::BufRead;

fn main() {
    println!("{:?}", get_total_score_for_guide(std::io::stdin().lock()));
}

fn get_total_score_for_guide(buffer: impl BufRead) -> (u64, u64) {
    let lines: Vec<String> = buffer.lines().map(|v| v.unwrap()).collect();
    (
        get_total_score_for_play_guide(&lines),
        get_total_score_for_outcome_guide(&lines),
    )
}

fn get_total_score_for_play_guide(lines: &[String]) -> u64 {
    let mut total_score = 0;
    for line in lines {
        let mut values = line.split(' ');
        let opponent = parse_opponent_play(values.next().unwrap());
        let my = parse_my_play(values.next().unwrap());
        total_score += get_round_score(opponent, my) + get_play_score(my);
    }
    total_score
}

fn get_total_score_for_outcome_guide(lines: &[String]) -> u64 {
    let mut total_score = 0;
    for line in lines {
        let mut values = line.split(' ');
        let opponent = parse_opponent_play(values.next().unwrap());
        let my = get_my_play(parse_outcome(values.next().unwrap()), opponent);
        total_score += get_round_score(opponent, my) + get_play_score(my);
    }
    total_score
}

fn get_my_play(outcome: Outcome, opponent: Play) -> Play {
    match outcome {
        Outcome::Lose => match opponent {
            Play::Rock => Play::Scissors,
            Play::Paper => Play::Rock,
            Play::Scissors => Play::Paper,
        },
        Outcome::Draw => opponent,
        Outcome::Win => match opponent {
            Play::Rock => Play::Paper,
            Play::Paper => Play::Scissors,
            Play::Scissors => Play::Rock,
        },
    }
}

fn parse_outcome(value: &str) -> Outcome {
    match value {
        "X" => Outcome::Lose,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => unreachable!(),
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

fn get_play_score(play: Play) -> u64 {
    match play {
        Play::Rock => 1,
        Play::Paper => 2,
        Play::Scissors => 3,
    }
}

fn get_round_score(opponent: Play, my: Play) -> u64 {
    if opponent == my {
        3
    } else if beats(opponent, my) {
        0
    } else {
        6
    }
}

fn beats(a: Play, b: Play) -> bool {
    match a {
        Play::Rock => matches!(b, Play::Scissors),
        Play::Paper => matches!(b, Play::Rock),
        Play::Scissors => matches!(b, Play::Paper),
    }
}

fn parse_opponent_play(value: &str) -> Play {
    match value {
        "A" => Play::Rock,
        "B" => Play::Paper,
        "C" => Play::Scissors,
        _ => unreachable!(),
    }
}

fn parse_my_play(value: &str) -> Play {
    match value {
        "X" => Play::Rock,
        "Y" => Play::Paper,
        "Z" => Play::Scissors,
        _ => unreachable!(),
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[test]
fn example_test() {
    let buffer = r#"A Y
B X
C Z
"#
    .as_bytes();
    assert_eq!(get_total_score_for_guide(buffer), (15, 12));
}
