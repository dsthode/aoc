use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

struct Map {
    lines: Vec<String>,
    x: usize,
    y: usize,
    ancho: usize,
    alto: usize
}

impl Map {
    fn new(file: &str) -> Map {
        let mut map = Map {lines: Vec::new(), x: 0, y: 0, alto: 0, ancho: 0};
        let f = File::open(file).unwrap();
        let reader = BufReader::new(f);
        for line in reader.lines() {
            map.lines.push(line.unwrap());
        }
        map.ancho = map.lines[0].len()-1;
        map.alto = map.lines.len()-1;
        map
    }

    fn avanzar(&mut self, x: usize, y: usize) -> Option<char> {
//        println!("posicion actual: {},{}", self.x, self.y);
        let old_x = self.x;
        let old_y = self.y;
        self.x += x;
        if self.x > self.ancho {
            self.x = (self.x % self.ancho)-1;
        }
        self.y += y;
        if self.y > self.alto {
            self.x = old_x;
            self.y = old_y;
            return None;
        }
//        println!("posicion nueva: {},{}: {:?}", self.x, self.y, self.lines[self.y].chars().nth(self.x));
        self.lines[self.y].chars().nth(self.x)
    }

    fn es_final(&self) -> bool {
        self.alto == self.y
    }

    fn rebobinar(&mut self) {
        self.x = 0;
        self.y = 0;
    }
}

fn main() {
    let mut map = Map::new("input.txt");
    let mut posiciones: Vec<char> = Vec::new();
    while !map.es_final() {
        match map.avanzar(1, 1) {
            Some(casilla) => posiciones.push(casilla),
            None => panic!("No se ha podido avanzar")
        }
    }
    println!("arboles:{}", posiciones.iter().filter(|x| **x == '#').count());
    map.rebobinar();
    posiciones = Vec::new();
    while !map.es_final() {
        match map.avanzar(3, 1) {
            Some(casilla) => posiciones.push(casilla),
            None => panic!("No se ha podido avanzar")
        }
    }
    println!("arboles:{}", posiciones.iter().filter(|x| **x == '#').count());
    map.rebobinar();
    posiciones = Vec::new();
    while !map.es_final() {
        match map.avanzar(5, 1) {
            Some(casilla) => posiciones.push(casilla),
            None => panic!("No se ha podido avanzar")
        }
    }
    println!("arboles:{}", posiciones.iter().filter(|x| **x == '#').count());
    map.rebobinar();
    posiciones = Vec::new();
    while !map.es_final() {
        match map.avanzar(7, 1) {
            Some(casilla) => posiciones.push(casilla),
            None => panic!("No se ha podido avanzar")
        }
    }
    println!("arboles:{}", posiciones.iter().filter(|x| **x == '#').count());
    map.rebobinar();
    posiciones = Vec::new();
    while !map.es_final() {
        match map.avanzar(1, 2) {
            Some(casilla) => posiciones.push(casilla),
            None => panic!("No se ha podido avanzar")
        }
    }
    println!("arboles:{}", posiciones.iter().filter(|x| **x == '#').count());
}
