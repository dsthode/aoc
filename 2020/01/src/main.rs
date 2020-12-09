use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("input.txt").unwrap();
    let reader = BufReader::new(f);
    let mut vec:Vec<i32> = Vec::new();
    for line in reader.lines() {
        vec.push(line.unwrap().parse::<i32>().unwrap());
    }
    let (primen1,segun1) = parte1(&vec);
    println!("Resultado parte1:{}+{} = 2020; {}*{}={}", primen1, segun1, primen1, segun1, primen1*segun1);
    let (primen2,segun2, tercer2) = parte2(&vec);
    println!("Resultado parte2:{}+{}+{} = 2020; {}*{}*{}={}", primen2, segun2, tercer2, primen2, segun2, tercer2, primen2*segun2*tercer2);
}

fn parte1(vec:&Vec<i32>) -> (i32, i32) {
    let mut primen: i32 = 0;
    let mut segun: i32 = 0;
    let mut encontrado: bool = false;
    for num1 in vec {
        for num2 in vec {
            if num1+num2 == 2020 {
                primen = *num1;
                segun = *num2;
                encontrado = true;
            }
            if encontrado {
                break;
            }
        }
        if encontrado {
            break;
        }
    }
    (primen, segun)
}

fn parte2(vec:&Vec<i32>) -> (i32, i32, i32) {
    let mut primen: i32 = 0;
    let mut segun: i32 = 0;
    let mut tercer: i32 = 0;
    let mut encontrado: bool = false;
    for num1 in vec {
        for num2 in vec {
            for num3 in vec {
                if num1+num2+num3 == 2020 {
                    primen = *num1;
                    segun = *num2;
                    tercer = *num3;
                    encontrado = true;
                }
                if encontrado {
                    break;
                }
                }
            if encontrado {
                break;
            }
        }
        if encontrado {
            break;
        }
    }
    (primen, segun, tercer)
}