use regex::{Match, Regex};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Move {
  amount: u32,
  from: usize,
  to: usize,
}

fn main() {
  let mut stack: Vec<VecDeque<char>> = Vec::new();
  let mut moves: Vec<Move> = Vec::new();
  for _ in 0..9 {
    stack.push(VecDeque::new());
  }
  load_file(&mut stack, &mut moves);
  part1(stack.clone(), &moves);
  println!("");
}

fn load_file(stack: &mut Vec<VecDeque<char>>, moves: &mut Vec<Move>) {
  let file = File::open("input.txt").unwrap();
  for line in BufReader::new(file).lines() {
    process_line(&line.unwrap(), stack, moves);
  }
}

fn process_line(line: &str, stack: &mut Vec<VecDeque<char>>, moves: &mut Vec<Move>) {
  let crates_rx: regex::Regex = Regex::new(
    r"(\[[A-Z]\]| {3}) (\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})?",
  ).unwrap();
  let moves_rx: regex::Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
  match crates_rx.captures(line) {
    Some(caps) => {
      for i in 1..=9 {
        prepare_storage(process_match(caps.get(i)), i, stack)
      }
    }
    None => {}
  }
  match moves_rx.captures(line) {
    Some(caps) => {
      let amount = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
      let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
      let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
      moves.push(Move { amount, from, to })
    }
    None => {}
  }
}

fn process_match(mtch: Option<Match>) -> Option<char> {
  match mtch {
    Some(data) => match data.as_str().as_bytes()[0] as char {
      '[' => Some(data.as_str().as_bytes()[1] as char),
      _ => None,
    },
    None => None,
  }
}

fn prepare_storage(chr: Option<char>, pos: usize, data: &mut Vec<VecDeque<char>>) {
  match chr {
    Some(car) => data[pos - 1].push_back(car),
    None => {}
  }
}

fn part1(mut stack: Vec<VecDeque<char>>, moves: &Vec<Move>) {
  for step in moves {
    for _ in 0..step.amount {
      let chr = stack[step.from - 1].pop_front().unwrap();
      stack[step.to - 1].push_front(chr);
    }
  }
  for v in stack.into_iter() {
    print!("{}", v.get(0).unwrap());
  }
}
