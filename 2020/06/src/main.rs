use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::collections::HashMap;

struct Answers {
    people: i32,
    questions: i32,
    questions2: HashMap<char, i32>
}

impl Answers {
    fn new(lines: &Vec<String>) -> Answers {
        let mut questions: Vec<char> = Vec::new();
        let mut part2: HashMap<char, i32> = HashMap::new();
        for line in lines.iter() {
            for ch in line.chars() {
                questions.push(ch);
                part2.entry(ch).and_modify(|e| {*e+= 1}).or_insert(1);
            }
        }
        questions.sort();
        questions.dedup();
        Answers{
            people: lines.len() as i32, 
            questions: questions.len() as i32, 
            questions2: part2
        }
    }
}

fn main() {
    let mut answers: Vec<Answers> = Vec::new();
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut lines: Vec<String> = Vec::new();
    let mut temp_str;
    for line in reader.lines() {
        temp_str = line.unwrap();
        if temp_str.len() == 0 {
            answers.push(Answers::new(&lines));
            lines.clear();
        } else {
            lines.push(temp_str);
        }
    }
    answers.push(Answers::new(&lines));
    let mut responses: i32 = 0;
    let mut responses2: i32 = 0;
    for answer in answers.iter() {
        responses += answer.questions;
        for (_, val) in answer.questions2.iter() {
            if *val == answer.people {
                responses2 += 1;
            }
        }
    }
    println!("El numero de respuestas afirmativas en la parte 1 es {}", responses);
    println!("El numero de respuestas afirmativas en la parte 2 es {}", responses2);
}
