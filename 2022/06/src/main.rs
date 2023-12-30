use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
  if env::args().count() < 2 {
    panic!("Missing file name");
  }
  let data = load_file(env::args().nth(1).unwrap());
  part1(&data);
  part2(&data);
}

fn load_file(name: String) -> String {
  let file = File::open(name).unwrap();
  let mut data: String = Default::default();
  let mut reader = BufReader::new(file);
  let _ = reader.read_line(&mut data);
  return data;
}

fn part1(data: &String) {
  let the_str = data.as_str().as_bytes();
  let mut set: HashSet<u8> = HashSet::new();
  for idx in 0..data.len() - 4 {
    set.drain();
    for count in 0..4 {
      set.insert(the_str[idx + count]);
    }
    if set.len() == 4 {
      println!(
        "The marker is at position {}, so {} characters have been processed",
        idx,
        idx + 4
      );
      break;
    }
  }
}

fn part2(data: &String) {
  let the_str = data.as_str().as_bytes();
  let mut set: HashSet<u8> = HashSet::new();
  for idx in 0..data.len() - 14 {
    set.drain();
    for count in 0..14 {
      set.insert(the_str[idx + count]);
    }
    if set.len() == 14 {
      println!(
        "The message is at position {}, so {} characters have been processed",
        idx,
        idx + 14
      );
      break;
    }
  }
}
