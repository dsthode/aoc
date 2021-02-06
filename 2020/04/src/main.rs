extern crate regex;

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::fmt;
use regex::Regex;

struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<String>
}

impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "byr:{:?} iyr:{:?} eyr:{:?} hgt:{:?} hcl:{:?} ecl:{:?} pid:{:?} cid:{:?}",
            self.byr,
            self.iyr,
            self.eyr,
            self.hgt,
            self.hcl,
            self.ecl,
            self.pid,
            self.cid
        )
    }
}

impl Passport {
    fn new(data: &Vec<String>) -> Passport {
        let mut p = Passport {byr:None, iyr:None, eyr:None, hgt:None, hcl:None, ecl:None, pid:None, cid:None};
        for linea in data {
            p.procesar_linea(linea);
        }
        p
    }

    fn procesar_linea(&mut self, linea: &String) {
        let params = linea.split(' ');
        for param in params {
            let parts: Vec<&str> = param.split(':').collect();
            match parts[0] {
                "byr" => self.byr = Some(parts[1].to_string()),
                "iyr" => self.iyr = Some(parts[1].to_string()),
                "eyr" => self.eyr = Some(parts[1].to_string()),
                "hgt" => self.hgt = Some(parts[1].to_string()),
                "hcl" => self.hcl = Some(parts[1].to_string()),
                "ecl" => self.ecl = Some(parts[1].to_string()),
                "pid" => self.pid = Some(parts[1].to_string()),
                "cid" => self.cid = Some(parts[1].to_string()),
                _ => panic!("Inesperado")
            }
        }
    }

    fn is_valid(&self) -> bool {
        self.byr != None
            && self.iyr != None
            && self.eyr != None
            && self.hgt != None
            && self.hcl != None
            && self.ecl != None
            && self.pid != None
    }

    fn is_valid2(&self) -> bool {
        self.is_valid()
            && self.valid_byr()
            && self.valid_iyr()
            && self.valid_eyr()
            && self.valid_hgt()
            && self.valid_hcl()
            && self.valid_ecl()
            && self.valid_pid()
    }

    fn valid_byr(&self) -> bool {
        let byr = self.byr.as_ref().unwrap().parse::<i32>().unwrap();
        byr >= 1920 && byr <= 2002
    }

    fn valid_iyr(&self) -> bool {
        let iyr = self.iyr.as_ref().unwrap().parse::<i32>().unwrap();
        iyr >= 2010 && iyr <= 2020
    }

    fn valid_eyr(&self) -> bool {
        let eyr = self.eyr.as_ref().unwrap().parse::<i32>().unwrap();
        eyr >= 2020 && eyr <= 2030
    }

    fn valid_hgt(&self) -> bool {
        let re = Regex::new(r"^(\d+)(cm|in)$").unwrap();
        if !re.is_match(self.hgt.as_ref().unwrap()) {
            return false;
        }
        let matches = re.captures(self.hgt.as_ref().unwrap()).unwrap();
        let hgt = matches[1].parse::<i32>().unwrap();
        match &matches[2] {
            "cm" => hgt >= 150 && hgt <= 193,
            "in" => hgt >= 59 && hgt <= 76,
            _ => false
        }
    }

    fn valid_hcl(&self) -> bool {
        let re = Regex::new("^#[0-9a-f]{6}$").unwrap();
        re.is_match(self.hcl.as_ref().unwrap())
    }

    fn valid_ecl(&self) -> bool {
        let valid = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        valid.contains(&self.ecl.as_ref().unwrap().as_str())
    }

    fn valid_pid(&self) -> bool {
        let re = Regex::new(r"^\d{9}$").unwrap();
        re.is_match(self.pid.as_ref().unwrap())
    }
}

fn main() {
    let mut passports: Vec<Passport> = Vec::new();
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut passport_lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() > 0 {
            passport_lines.push(line);
        } else {
            passports.push(Passport::new(&passport_lines));
            passport_lines.clear();
        }
    }
    passports.push(Passport::new(&passport_lines));
    println!("Leidos {} pasaportes", passports.len());
    println!("Pasaportes validos: {}", passports.iter().filter(|x| x.is_valid()).count());
    println!("Pasaportes validos 2: {}", passports.iter().filter(|x| x.is_valid2()).count());
}
