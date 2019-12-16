use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Copy, Clone, Debug)]
struct Coordinate {
    x: i32,
    y: i32
}

impl Coordinate {
    fn new(_x:i32, _y:i32) -> Coordinate {
        Coordinate {
            x: _x,
            y: _y
        }
    }

    fn translate(&self, instruction:&str) -> Coordinate {
        let amount = instruction.chars().skip(1).collect::<String>().parse::<i32>().unwrap();
        match instruction.chars().nth(0).unwrap() {
            'U' => {
                Coordinate::new(self.x, self.y + amount)
            },
            'R' => {
                Coordinate::new(self.x + amount, self.y)
            },
            'D' => {
                Coordinate::new(self.x, self.y - amount)
            },
            'L' => {
                Coordinate::new(self.x - amount, self.y)
            },
            _ => {
                panic!("Instruccion desconocida: '{}'", instruction.chars().nth(1).unwrap());
            }
        }
    }

    fn distance_from_central(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

#[derive(Debug)]
struct Intersection {
    coordinate: Coordinate,
    distance_from_central: i32,
    steps: i32
}

struct WireSegment {
    from: Coordinate,
    to: Coordinate
}

impl WireSegment {
    fn new(_from:Coordinate, _to:Coordinate) -> WireSegment {
        WireSegment {
            from: _from,
            to: _to
        }
    }

    fn check_intersection(&self, other: &WireSegment) -> Option<Coordinate> {
        let response : Coordinate;
        if ((self.from.x <= other.from.x && self.to.x >= other.from.x) || (self.to.x <= other.from.x && self.from.x >= other.from.x))
        && ((other.from.y <= self.from.y && other.to.y >= self.from.y) || (other.to.y <= self.from.y && other.from.y >= self.from.y)) {
            response = Coordinate::new(other.from.x, self.from.y);
            Some(response)
        } else if ((other.from.x <= self.from.x && other.to.x >= self.from.x) || (other.to.x <= self.from.x && other.from.x >= self.from.x))
        && ((self.from.y <= other.from.y && self.to.y >= other.from.y) || (self.to.y <= other.from.y && self.from.y >= other.from.y)) {
            response = Coordinate::new(self.from.x, other.from.y);
            Some(response)
        } else {
            None
        }
    }

    fn steps(&self) -> i32 {
        let steps : i32;
        if self.from.x == self.to.x {
            steps = self.from.y - self.to.y
        } else {
            steps = self.from.x - self.to.x
        }
        steps.abs()
    }

    fn steps_to(&self, coord:Coordinate) -> i32 {
        let steps : i32;
        if self.from.x == self.to.x {
            steps = coord.y - self.from.y;
        } else {
            steps = coord.x - self.from.x;
        }
        steps.abs()
    }
}

struct Wire {
    segments: Vec<WireSegment>
}

impl Wire {
    fn new() -> Wire {
        Wire {
            segments: Vec::new()
        }
    }
}

struct Board {
    wires: Vec<Wire>,
    intersections: Vec<Intersection>
}

impl Board {
    fn new() -> Board {
        Board {
            wires: Vec::new(),
            intersections: Vec::new()
        }
    }

    fn add_wire(&mut self, wire:Wire) {
        let mut steps1 = 0i32;
        let mut steps2 : i32;
        for segment1 in &wire.segments {
            for wire2 in &self.wires {
                steps2 = 0;
                for segment2 in &wire2.segments {
                    match segment1.check_intersection(&segment2) {
                        Some(coord) => {
                            self.intersections.push(Intersection {
                                coordinate: coord,
                                distance_from_central: coord.distance_from_central(),
                                steps: steps1 + steps2 + segment1.steps_to(coord) + segment2.steps_to(coord)
                            });
                        },
                        None => { }
                    }
                    steps2 += segment2.steps();
                }
            }
            steps1 += segment1.steps();
        }
        self.wires.push(wire);
    }
    
    fn load_data(&mut self, nombre:String) {
        let f = File::open(nombre).expect("No se ha podido abrir el archivo de datos");
        let mut buf = BufReader::new(f);
        let mut ok = true;
        let mut linea = String::new();
        let mut num_bytes : usize;
        let mut _tmp = String::new();
        while ok {
            linea.clear();
            num_bytes = buf.read_line(&mut linea).expect("Error leyendo el archivo de datos");
            if num_bytes == 0 {
                ok = false;
            } else {
                let mut from = Coordinate::new(0, 0);
                let mut to : Coordinate;
                let mut wire : Wire = Wire::new();
                let mut segment : WireSegment;
                for instruction in linea.trim().split(',') {
                    to = from.translate(instruction);
                    segment = WireSegment::new(from, to);
                    wire.segments.push(segment);
/*                    println!("Desde:{:?}, hasta:{:?}, instruccion:{}", from, to, instruction);
                    std::io::stdin().read_line(&mut _tmp).expect("");
*/                    from = to
                }
                self.add_wire(wire);
            }
        }
    }
}

fn main() {
    let mut board = Board::new();
    board.load_data(std::env::args().nth(1).unwrap());
    board.intersections.sort_by(|a, b| a.distance_from_central.cmp(&b.distance_from_central));
    println!("distance:{:?}", board.intersections[1]);
    board.intersections.sort_by(|a, b| a.steps.cmp(&b.steps));
    println!("steps:{:?}", board.intersections[1]);
}
