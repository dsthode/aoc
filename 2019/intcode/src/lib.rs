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
      println!("pc:{} - {:?}", self.pc, self.memoria);
  }

  fn modo_parametro(&self, opcode:i32, num_param:u32) -> i32 {
      let divisor:i32 = (10 as i32).pow(1 + num_param);
      let posicion = opcode / divisor;
      posicion % 10
  }

  fn leer_parametro(&self, opcode: i32, num_param: u32) -> i32 {
    let modo = self.modo_parametro(opcode, num_param);
    match modo {
        0 => {
            let addr = self.leer_memoria(self.pc+(num_param as usize));
            self.leer_memoria(usize::try_from(addr).unwrap())
        },
        1 => {
            if num_param == 3 {
                panic!("Error: el modo de variable inmediato para escritura no es valido");
            }
            self.leer_memoria(self.pc+(num_param as usize))
        },
        _ => {
            panic!("Error: modo de acceso a memoria desconocido {}", modo);
        }
    }
  }

  pub fn ejecutar(&mut self) -> bool {
      if self.pc >= self.memoria.len() {
          panic!("El puntero ha excedido el tamaño de la memoria");
      }
      match self.memoria[self.pc] % 100 {
          1 => {
              let param1 = self.leer_parametro(self.memoria[self.pc], 1);
              let param2 = self.leer_parametro(self.memoria[self.pc], 2);
              assert!(self.modo_parametro(self.memoria[self.pc], 3) == 0);
              let addr_res = self.leer_memoria(self.pc+3);
              self.establecer_memoria(
                  usize::try_from(addr_res).unwrap(), 
                  param1 + param2);
              self.pc += 4;
              true
          },
          2 => {
            let param1 = self.leer_parametro(self.memoria[self.pc], 1);
            let param2 = self.leer_parametro(self.memoria[self.pc], 2);
            assert!(self.modo_parametro(self.memoria[self.pc], 3) == 0);
            let addr_res = self.leer_memoria(self.pc+3);
              self.establecer_memoria(
                  usize::try_from(addr_res).unwrap(), 
                  param1 * param2);
              self.pc += 4;
              true
          },
          3 => {
              let addr = self.leer_memoria(self.pc+1);
              let mut result : bool = true;
              let mut line = String::new();
              print!("Entrada: ");
              io::stdout().flush().unwrap();
              match io::stdin().read_line(&mut line) {
                  Ok(_) => {
                    self.establecer_memoria(
                        usize::try_from(addr).unwrap(),
                        line.trim().parse::<i32>().unwrap()
                    );
                  },
                  Err(error) => {
                      eprintln!("Error: {}", error);
                      result = false;
                  }
              }
              self.pc += 2;
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
