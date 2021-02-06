use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;
use std::num::ParseIntError;
use std::cmp::Ordering;
use std::fmt;

#[derive(Eq)]
struct BoardingPass {
    row: i32,
    seat: i32,
    id: i32
}

impl Ord for BoardingPass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for BoardingPass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BoardingPass {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl FromStr for BoardingPass {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bin = s.replace("F", "0")
                .replace("B", "1")
                .replace("R", "1")
                .replace("L", "0");
        let row = i32::from_str_radix(&bin[0..7], 2).unwrap();
        let seat = i32::from_str_radix(&bin[7..10], 2).unwrap();
        Ok(BoardingPass{ row: row, seat: seat, id: row*8+seat})
    }
}

impl fmt::Display for BoardingPass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "row:{}, seat:{}, id:{}", self.row, self.seat, self.id)
    }
}

fn main() {
    let mut passes: Vec<BoardingPass> = Vec::new();
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    for line in reader.lines() {
        passes.push(BoardingPass::from_str(&line.unwrap()).unwrap());
    }
    passes.sort();
    let mut temp_id = passes[0].id;
    for pass in passes.iter() {
        if pass.id-temp_id > 1 {
            println!("Parte 2: {}", pass.id-1);
            break;
        }
        temp_id = pass.id;
    }
    println!("El maximo id es: {}", passes.iter().max().unwrap());
}
