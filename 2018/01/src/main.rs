use std::process;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let mut result:i32 = 0;
    let input_file = match std::env::args().nth(1) {
        None => {
            println!("Missing input file");
            process::exit(1);
        },
        Some(input_file) => input_file
    };
    let fhandle = File::open(input_file).expect("Can't read input file");
    let reader = BufReader::new(fhandle);
    for l in reader.lines() {
        let line = match l {
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            },
            Ok(l) => l
        };
        let converted:i32 = match i32::from_str_radix(line.as_str(), 10) {
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            },
            Ok(num) => num
        };
        result += converted;
    }
    println!("The resulting frequency is {}", result);
}
