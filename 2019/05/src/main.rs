extern crate intcode;

fn main() {
    let mut comp = intcode::Computer::new();
    comp.cargar_memoria(std::env::args().nth(1).unwrap());
    while comp.ejecutar() { }
}
