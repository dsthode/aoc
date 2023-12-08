use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
  let mut count1 = 0;
  let mut count2 = 0;
  let file = File::open("input.txt").unwrap();
  for line in BufReader::new(file).lines() {
    let strin = line.unwrap();
    if part1(&strin) {
      count1 += 1;
    }
    if part2(&strin) {
      count2 += 1;
    }
  }
  println!("Part 1 overlapping assignments: {}", count1);
  println!("Part 2 overlapping assignments: {}", count2);
}

fn part1(line: &str) -> bool {
  let ranges: Vec<&str> = line.split(",").collect();
  let range1: Vec<&str> = ranges[0].split("-").collect();
  let range2: Vec<&str> = ranges[1].split("-").collect();

  return range_contained(
    i64::from_str(range1[0]).unwrap(),
    i64::from_str(range1[1]).unwrap(),
    i64::from_str(range2[0]).unwrap(),
    i64::from_str(range2[1]).unwrap(),
  );
}

fn part2(line: &str) -> bool {
  let ranges: Vec<&str> = line.split(",").collect();
  let range1: Vec<&str> = ranges[0].split("-").collect();
  let range2: Vec<&str> = ranges[1].split("-").collect();

  return range_overlapping(
    i64::from_str(range1[0]).unwrap(),
    i64::from_str(range1[1]).unwrap(),
    i64::from_str(range2[0]).unwrap(),
    i64::from_str(range2[1]).unwrap(),
  );
}

fn range_contained(from1: i64, to1: i64, from2: i64, to2: i64) -> bool {
  if from1 >= from2 && to1 <= to2 {
    return true;
  } else if from2 >= from1 && to2 <= to1 {
    return true;
  }
  return false;
}

fn range_overlapping(from1: i64, to1: i64, from2: i64, to2: i64) -> bool {
  if from1 >= from2 || to1 <= to2 {
    return true;
  } else if from2 >= from1 || to2 >= to1 {
    return true;
  }
  return false;
}
