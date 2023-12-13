use regex::{Match, Regex};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let mut data: Vec<VecDeque<char>> = Vec::new();
  for _ in 0..9 {
    data.push(VecDeque::new());
  }
  let file = File::open("input.txt").unwrap();
  for line in BufReader::new(file).lines() {
    part1(&line.unwrap(), &mut data);
  }
  for v in data.into_iter() {
    print!("{}", v.get(0).unwrap());
  }
  println!("");
}

fn part1(line: &str, data: &mut Vec<VecDeque<char>>) {
  let crates_rx: regex::Regex = Regex::new(
    r"(\[[A-Z]\]| {3}) (\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})? ?(\[[A-Z]\]| {3})?",
  ).unwrap();
  let moves_rx: regex::Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
  match crates_rx.captures(line) {
    Some(caps) => {
      for i in 1..=9 {
        prepare_storage(process_match(caps.get(i)), i, data)
      }
    }
    None => {}
  }
  match moves_rx.captures(line) {
    Some(caps) => perform_move(
      caps.get(1).unwrap().as_str().parse::<u32>().unwrap(),
      caps.get(2).unwrap().as_str().parse::<usize>().unwrap(),
      caps.get(3).unwrap().as_str().parse::<usize>().unwrap(),
      data,
    ),
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

fn perform_move(amount: u32, from: usize, to: usize, data: &mut Vec<VecDeque<char>>) {
  for _ in 0..amount {
    let chr = data[from - 1].pop_front().unwrap();
    data[to - 1].push_front(chr);
  }
}
