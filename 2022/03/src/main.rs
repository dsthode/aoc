use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const LETTERS: &str = ".abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
  let mut repeated: Vec<char> = Vec::new();
  let mut repeated2: Vec<char> = Vec::new();
  let mut sum_prio = 0;
  let file = File::open("input.txt").unwrap();
  for line in BufReader::new(file).lines() {
    match line {
      Ok(items) => {
        part1(&items.as_bytes(), &mut repeated);
      }
      _ => {}
    }
  }
  for ch in repeated {
    sum_prio += char_to_prio(ch);
  }
  println!("La suma de las prioridades de parte 1 es:{}", sum_prio);
  let file = File::open("input.txt").unwrap();
  let mut lines = BufReader::new(file).lines();
  while let Some(line1) = lines.next() {
    let line2 = lines.next().unwrap();
    let line3 = lines.next().unwrap();
    part2(
      &line1.unwrap(),
      &line2.unwrap(),
      &line3.unwrap(),
      &mut repeated2,
    )
  }
  sum_prio = 0;
  for ch in repeated2 {
    sum_prio += char_to_prio(ch);
  }
  println!("La suma de las prioridades de parte 2 es:{}", sum_prio);
}

fn part1(line: &[u8], repeated: &mut Vec<char>) {
  let mut items = HashSet::<char>::new();
  let mut local_repeated = HashSet::<char>::new();
  let mid = line.len() / 2;
  let mut idx = 0;
  while idx != mid {
    items.insert(line[idx] as char);
    idx += 1;
  }
  idx = 0;
  while idx != mid {
    if items.contains(&(line[mid + idx] as char))
      && !local_repeated.contains(&(line[mid + idx] as char))
    {
      local_repeated.insert(line[mid + idx] as char);
      repeated.push(line[mid + idx] as char);
    }
    idx += 1;
  }
}

fn part2(line1: &str, line2: &str, line3: &str, repeated: &mut Vec<char>) {
  let mut local_repeated = HashSet::<char>::new();
  for ch in line1.chars() {
    if line2.contains(ch) && line3.contains(ch) && !local_repeated.contains(&ch) {
      local_repeated.insert(ch);
      repeated.push(ch);
    }
  }
}

fn char_to_prio(ch: char) -> usize {
  return match LETTERS.find(ch) {
    Some(idx) => idx,
    None => 0,
  };
}
