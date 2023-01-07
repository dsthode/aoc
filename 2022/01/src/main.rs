use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, Clone, Copy)]
struct Data {
    value: u32,
    index: u32
}

fn main() {
    let mut data: Vec<Data> = Vec::new();
    let file = File::open("input.txt").unwrap();
    let mut value: u32;
    let mut sum: u32 = 0;
    let mut index: u32 = 1;
    for line in BufReader::new(file).lines() {
        match line {
            Ok(item) => {
                if item.len() > 1 {
                    value = item.parse::<u32>().unwrap();
                    sum += value;
                } else {
                    data.push(Data{ value: sum, index: index });
                    sum = 0;
                    index += 1;
                }
            }
            _ => {}
        }
    }
    data.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
    println!("Max index: {} with value: {}", data[data.len()-1].index, data[data.len()-1].value);
    sum = 0;
    for i in 1..4 {
        println!("{}", i);
        sum += data[data.len()-i].value;
        println!("Amount from elf {}: {}", data[data.len()-i].index, data[data.len()-i].value);
    }
    println!("Amount from three most loaded elves: {}", sum);
}
