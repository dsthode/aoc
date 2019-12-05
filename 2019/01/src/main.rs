use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;

fn main() {
    let f = File::open(std::env::args().nth(1).unwrap()).expect("No se ha podido leer el archivo de entrada");
    let reader = BufReader::new(f);
    let mut masa : f32;
    let mut fuel_tmp: f32;
    let fuel_modulos : f32;
    let fuel_fuel : f32;
    let mut vec_fuel_modulos : Vec<f32> = Vec::new();
    let mut vec_fuel_fuel : Vec<f32> = Vec::new();
    for line in reader.lines() {
        masa = line.unwrap().parse::<f32>().unwrap();
        fuel_tmp = calcular_fuel_modulo(masa);
        vec_fuel_modulos.push(fuel_tmp);
        fuel_tmp = calcular_fuel_fuel(fuel_tmp);
        vec_fuel_fuel.push(fuel_tmp);
    }
    fuel_modulos = vec_fuel_modulos.iter().fold(0.0, |acc, x| acc + x);
    fuel_fuel = vec_fuel_fuel.iter().fold(0.0, |acc, x| acc + x);
    print!("El fuel de los modulos es: {}\n", fuel_modulos);
    print!("El fuel del fuel es: {}\n", fuel_fuel);
    print!("El fuel total es: {}\n", fuel_modulos + fuel_fuel);
}

fn calcular_fuel_modulo(masa:f32) -> f32 {
    return (masa/3.0).floor() - 2.0;
}

fn calcular_fuel_fuel(fuel:f32) -> f32 {
    let mut fuel_part:f32 = fuel;
    let mut fuel_total:f32 = 0.0;
    loop {
        fuel_part = (fuel_part / 3.0).floor() - 2.0;
        if fuel_part <= 0.0 {
            break;
        }
        fuel_total = fuel_total + fuel_part;
    }
    return fuel_total;
}