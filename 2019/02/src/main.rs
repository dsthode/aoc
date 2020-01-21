extern crate intcode;

fn main() {
    for noun in 0..99 {
        for verb in 0..99 {
            let mut c = intcode::Computer::new();
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
