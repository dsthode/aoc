extern crate regex;

use std::rc::Rc;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use regex::Regex;

struct ContainsBag {
    amount: i32,
    color: Rc<Bag>
}

struct Bag {
    color: String,
    contains: Vec<ContainsBag>,
    contained_by: Vec<Rc<Bag>>
}

impl Bag {
    fn new(color: String) -> Bag {
        Bag {
            color: color, 
            contains: Vec::new(), 
            contained_by: Vec::new()
        }
    }
}

struct RuleSet {
    rules: Vec<Rc<Bag>>
}

impl RuleSet {
    fn new() -> RuleSet {
        RuleSet {rules: Vec::new()}
    }

    fn from_line(&mut self, line: String) {
        let re_bags = Regex::new(r"^(\w+ \w+) bags contain (\d+) (\w+ \w+) bags?(?:.|, )(?:(\d+) (\w+ \w+) bags(?:.|, ))*$").unwrap();
        let re_no_bags = Regex::new(r"(\w+ \w+) bags contain no other bags.").unwrap();
        let matches;
        if re_bags.is_match(&line) {
            matches = re_bags.captures(line.as_ref()).unwrap();
        } else if re_no_bags.is_match(&line) {
            matches = re_no_bags.captures(line.as_ref()).unwrap();
        } else {
            panic!("No encaja ninguna regex: {}", line);
        }
        let mut bag = self.find_or_create(matches[1].to_string());
        if matches.len() > 1 {
            let mut temp_bag;
            for step in (2..matches.len()).step_by(2) {
                temp_bag = self.find_or_create(matches[step].to_string());
            }
        }
    }

    fn find_or_create(&mut self, color: String) -> Rc<Bag> {
        let temp;
        let bag = match self.find_bag(&color) {
            Some(b) => b,
            None => {
                temp = self.create_bag(&color);
                &temp
            }
        };
        Rc::clone(&bag)
    }

    fn find_bag(&self, color: &String) -> Option<&Rc<Bag>> {
        self.rules.iter().find(|&x| x.color == *color)
    }

    fn create_bag(&mut self, color: &String) -> Rc<Bag> {
        let b = Rc::new(Bag::new(color.clone()));
        self.rules.push(Rc::clone(&b));
        Rc::clone(&b)
    }
}

fn main() {
    let mut ruleset = RuleSet::new();
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        ruleset.from_line(line.unwrap());
    }
}
