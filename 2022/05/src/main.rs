use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  let mut data: Vec<Vec<char>> = Vec::new();
  for _ in 0..9 {
    data.push(Vec::new());
  }
  let file = File::open("input.txt").unwrap();
  for line in BufReader::new(file).lines() {
    part1(&line.unwrap(), &data);
  }
}

fn part1(line: &str, data: &Vec<Vec<char>>) {
  let crates_rx: regex::Regex = Regex::new(
    r"(\[[A-Z]\]|\s{3})\s(\[[A-Z]\]|\s{3})\s(\[[A-Z]\]|\s{3})\s(\[[A-Z]\]|\s{3})\s(\[[A-Z]\]|\s{3})\s(\[[A-Z]\]|\s{3})\s(\[[A-Z]\]|\s{3})\s(\[[A-Z]\]|\s{3})\s(\[[A-Z]\]|\s{3})",
  ).unwrap();
  let other_rx: regex::Regex = Regex::new("").unwrap();
  let moves_rx: regex::Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
  match crates_rx.captures(line) {
    Some(caps) => {
      println!("line:{}", line);
      for i in 1..=9 {
        println!("caps({}):{}", i, caps.get(i).unwrap().as_str());
      }
    }
    None => {}
  }
}
