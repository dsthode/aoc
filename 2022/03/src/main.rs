use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const LETTERS: &str = ".abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
  let mut repeated: Vec<char> = Vec::new();
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
    let prio = char_to_prio(ch);
    println!("{} prio is {}", ch, prio);
    sum_prio += char_to_prio(ch);
  }
  println!("La suma de las prioridades de fase 1 es:{}", sum_prio)
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

fn char_to_prio(ch: char) -> usize {
  return match LETTERS.find(ch) {
    Some(idx) => idx,
    None => 0,
  };
}
