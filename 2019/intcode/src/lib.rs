use std::io::prelude::*;
use std::fs::File;
use std::ops::IndexMut;
use std::convert::TryFrom;
use std::io;

pub struct Computer {
  pc: usize,
  memoria: Vec<i32>
}

impl Computer {
  pub fn new() -> Computer {
      Computer {
          pc: 0,
          memoria: Vec::new()
      }
  }

  pub fn cargar_memoria(&mut self, nombre:String) {
      let mut f = File::open(nombre).expect("No se ha podido abrir el archivo de la memoria");
      let mut linea = String::new();
      f.read_to_string(&mut linea).expect("No se ha podido leer el archivo de la memoria");
      for num in linea.split(',').map(|x| x.parse::<i32>().unwrap()) {
          self.memoria.push(num);
      }
  }

  pub fn leer_memoria(&self, pos:usize) -> i32 {
      self.memoria[pos]
  }

  pub fn establecer_memoria(&mut self, dir:usize, valor:i32) {
      *self.memoria.index_mut(dir) = valor;
  }

  pub fn volcar_memoria(&self) {
      print!("pc:{} - {:?}\n", self.pc, self.memoria);
  }

  pub fn ejecutar(&mut self) -> bool {
      if self.pc >= self.memoria.len() {
          panic!("El puntero ha excedido el tamaño de la memoria");
      }
      match self.memoria[self.pc] % 100 {
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
          3 => {
              let addr = self.leer_memoria(self.pc+1);
              let mut result : bool = true;
              let mut line = String::new();
              match io::stdin().read_line(&mut line) {
                  Ok(_) => {
                    self.establecer_memoria(
                        usize::try_from(addr).unwrap(),
                        line.parse::<i32>().unwrap()
                    );
                  },
                  Err(error) => {
                      println!("Error: {}", error);
                    result = false;
                  }
              }
              result
          },
          4 => {
              let addr = self.leer_memoria(self.pc+1);
              println!("Salida: {}", self.leer_memoria(usize::try_from(addr).unwrap()));
              self.pc += 2;
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
