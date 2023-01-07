use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Clone, Copy)]
enum Move {
    None, Rock, Paper, Scissors
}

struct Round {
    other: Move,
    me: Move
}

fn main() {
    let mut game: Vec<Round> = Vec::new();
    let file = File::open("input.txt").unwrap();
    for line in BufReader::new(file).lines() {
        match line {
            Ok(round) => {
                if round.len() >= 1 {
                    game.push(Round { other: parse_from_move(round.get(0..1).unwrap()), me: parse_from_move(round.get(2..3).unwrap())});
                }
            }
            _ => {}
        }
    }
    let mut score: u32 = 0;
    for round in game.iter() {
        score += score_round(round.other, round.me);
    }
    println!("Total score is {}", score);
    score = 0;
    for round in game.iter() {
        score += score_round_part2(round.other, round.me);
    }
    println!("Total score in part2 is {}", score);
}

fn parse_from_move(played: &str) -> Move {
    match played {
        "A"| "X" => Move::Rock,
        "B"| "Y" => Move::Paper,
        "C"| "Z" => Move::Scissors,
        _ => Move::None
    }
}

fn score_round(other: Move, me: Move) -> u32 {
    let mut score: u32 = 0;
    match me {
        Move::Rock => { score += 1 },
        Move::Paper => { score += 2 },
        Move::Scissors => { score += 3 }
        _ => {}
    };
    match (other, me) {
        (Move::Rock, Move::Rock) |
        (Move::Paper, Move::Paper) |
        (Move::Scissors, Move::Scissors) => { score += 3 },
        (Move::Scissors, Move::Rock) |
        (Move::Rock, Move::Paper) |
        (Move::Paper, Move::Scissors) => { score += 6 },
        (_, _) => {}
    };
    return score
}

fn score_round_part2(other: Move, me: Move) -> u32 {
    let mut score: u32 = 0;
    match me {
        Move::Rock => {
            score += 0;
            match other {
                Move::Rock => { score += 3 },
                Move::Paper => { score += 1 },
                Move::Scissors => { score += 2 },
                _ => {}
            };
        },
        Move::Paper => {
            score += 3;
            match other {
                Move::Rock => { score += 1 },
                Move::Paper => { score += 2 },
                Move::Scissors => { score += 3 },
                _ => {}
            };
        },
        Move::Scissors => {
            score += 6;
            match other {
                Move::Rock => { score += 2 },
                Move::Paper => { score += 3 },
                Move::Scissors => { score += 1 },
                _ => {}
            };
        }
        _ => {}
    };
    return score;
}
