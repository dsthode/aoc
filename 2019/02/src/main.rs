use std::io::prelude::*;
use std::fs::File;
use std::ops::IndexMut;
use std::convert::TryFrom;

struct Computer {
    pc: usize,
    memoria: Vec<i32>
}

impl Computer {
    fn new() -> Computer {
        Computer {
            pc: 0,
            memoria: Vec::new()
        }
    }

    fn cargar_memoria(&mut self, nombre:String) {
        let mut f = File::open(nombre).expect("No se ha podido abrir el archivo de la memoria");
        let mut linea = String::new();
        f.read_to_string(&mut linea).expect("No se ha podido leer el archivo de la memoria");
        for num in linea.split(',').map(|x| x.parse::<i32>().unwrap()) {
            self.memoria.push(num);
        }
    }

    fn leer_memoria(&self, pos:usize) -> i32 {
        self.memoria[pos]
    }

    fn establecer_memoria(&mut self, dir:usize, valor:i32) {
        *self.memoria.index_mut(dir) = valor;
    }

    fn volcar_memoria(&self) {
        print!("pc:{} - {:?}\n", self.pc, self.memoria);
    }

    fn ejecutar(&mut self) -> bool {
        if self.pc >= self.memoria.len() {
            panic!("El puntero ha excedido el tamaño de la memoria");
        }
        match self.memoria[self.pc] {
            1 => {
                let addr_1 = self.leer_memoria(self.pc+1);
                let addr_2 = self.leer_memoria(self.pc+2);
                let addr_res = self.leer_memoria(self.pc+3);
                self.establecer_memoria(
                    usize::try_from(addr_res).unwrap(), 
                    self.leer_memoria(usize::try_from(addr_1).unwrap()) 
                        + self.leer_memoria(usize::try_from(addr_2).unwrap()));
                self.pc += 4;
                true
            },
            2 => {
                let addr_1 = self.leer_memoria(self.pc+1);
                let addr_2 = self.leer_memoria(self.pc+2);
                let addr_res = self.leer_memoria(self.pc+3);
                self.establecer_memoria(
                    usize::try_from(addr_res).unwrap(), 
                    self.leer_memoria(usize::try_from(addr_1).unwrap()) 
                        * self.leer_memoria(usize::try_from(addr_2).unwrap()));
                self.pc += 4;
                true
            },
            99 => {
                false
            },
            _ => {
                panic!("Instruccion indefinida: {}", self.memoria[self.pc]);
            }
        }
    }
}

fn main() {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut c = Computer::new();
            c.cargar_memoria(std::env::args().nth(1).unwrap());
            c.establecer_memoria(1, noun);
            c.establecer_memoria(2, verb);
            while c.ejecutar() { }
            let mem_0 = c.leer_memoria(0);
            if mem_0 == 19690720 { 
                print!("noun: {}, verb: {}\n", noun, verb);
                break; 
            }
        }
    }
}
