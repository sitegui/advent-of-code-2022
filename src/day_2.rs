use crate::data::Data;
use crate::parser::Parser;

#[derive(Debug, Clone, Copy)]
enum Play {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

pub fn solve() -> (i64, i64) {
    Data::read(2)
        .lines()
        .map(|mut line| {
            let opponent_play = parse_opponent_play(line.consume_words(1));
            let second_column = line.consume_words(1);

            let my_play_p1 = parse_my_play(second_column);
            let score_p1 = my_play_p1.score() + outcome(opponent_play, my_play_p1).score();

            let outcome_p2 = parse_outcome(second_column);
            let my_play_p2 = my_play(opponent_play, outcome_p2);

            let score_p2 = my_play_p2.score() + outcome_p2.score();

            (score_p1, score_p2)
        })
        .fold((0, 0), |acc, round| (acc.0 + round.0, acc.1 + round.1))
}

fn parse_opponent_play(bytes: &[u8]) -> Play {
    match bytes {
        [b'A'] => Play::Rock,
        [b'B'] => Play::Paper,
        [b'C'] => Play::Scissors,
        _ => unreachable!(),
    }
}

fn parse_my_play(bytes: &[u8]) -> Play {
    match bytes {
        [b'X'] => Play::Rock,
        [b'Y'] => Play::Paper,
        [b'Z'] => Play::Scissors,
        _ => unreachable!(),
    }
}

fn parse_outcome(bytes: &[u8]) -> Outcome {
    match bytes {
        [b'X'] => Outcome::Lose,
        [b'Y'] => Outcome::Draw,
        [b'Z'] => Outcome::Win,
        _ => unreachable!(),
    }
}

fn outcome(opponent: Play, me: Play) -> Outcome {
    match (opponent, me) {
        (Play::Rock, Play::Rock) => Outcome::Draw,
        (Play::Rock, Play::Paper) => Outcome::Win,
        (Play::Rock, Play::Scissors) => Outcome::Lose,

        (Play::Paper, Play::Rock) => Outcome::Lose,
        (Play::Paper, Play::Paper) => Outcome::Draw,
        (Play::Paper, Play::Scissors) => Outcome::Win,

        (Play::Scissors, Play::Rock) => Outcome::Win,
        (Play::Scissors, Play::Paper) => Outcome::Lose,
        (Play::Scissors, Play::Scissors) => Outcome::Draw,
    }
}

fn my_play(opponent: Play, outcome: Outcome) -> Play {
    match (opponent, outcome) {
        (_, Outcome::Draw) => opponent,
        (Play::Rock, Outcome::Win) => Play::Paper,
        (Play::Rock, Outcome::Lose) => Play::Scissors,

        (Play::Paper, Outcome::Win) => Play::Scissors,
        (Play::Paper, Outcome::Lose) => Play::Rock,

        (Play::Scissors, Outcome::Win) => Play::Rock,
        (Play::Scissors, Outcome::Lose) => Play::Paper,
    }
}

impl Play {
    fn score(self) -> i64 {
        match self {
            Play::Rock => 1,
            Play::Paper => 2,
            Play::Scissors => 3,
        }
    }
}

impl Outcome {
    fn score(self) -> i64 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}
