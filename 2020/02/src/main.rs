extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::num::ParseIntError;
use regex::Regex;

struct PassData {
    char_min: i32,
    char_max: i32,
    the_char: String,
    the_pass: String
}

impl FromStr for PassData {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z]+)").unwrap();
        let parts = re.captures(s).unwrap();
        Ok(PassData{
            char_min: parts[1].parse::<i32>().unwrap(), 
            char_max: parts[2].parse::<i32>().unwrap(), 
            the_char: parts[3].to_string(), 
            the_pass: parts[4].to_string()
        })
    }
}

impl PassData {
    fn count_chars(&self) -> i32 {
        let matches: Vec<&str> = self.the_pass.matches(&self.the_char).collect();
        matches.len() as i32
    }

    fn is_valid(&self) -> bool {
        let char_count = self.count_chars();
        (char_count >= self.char_min) && (char_count  <= self.char_max)
    }

    fn is_valid_part2(&self) -> bool {
        let first_char = self.the_pass.get(((self.char_min-1) as usize)..((self.char_min) as usize));
        let second_char = self.the_pass.get(((self.char_max-1) as usize)..((self.char_max) as usize));
        (first_char.unwrap() == self.the_char || second_char.unwrap() == self.the_char) && (first_char.unwrap() != second_char.unwrap())
    }
}

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut vec:Vec<PassData> = Vec::new();
    for line in reader.lines() {
        vec.push(line.unwrap().parse::<PassData>().unwrap());
    }
    let valid = vec.iter().filter(|x| x.is_valid());
    println!("El numero de elementos validos en la parte 1 es {}", valid.count());
    let valid2 = vec.iter().filter(|x| x.is_valid_part2());
    println!("El numero de elementos validos en la parte 2 es {}", valid2.count());
}
