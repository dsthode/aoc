use std::str::FromStr;

fn criteria_6digits(number:i32) -> bool {
    number.to_string().len() == 6
}

fn criteria_2adjacents(number:i32) -> bool {
    let num_str = number.to_string();
    let mut current_digit : char = num_str.chars().nth(0).unwrap();
    let mut two_adjacents = false;
    for c in num_str.chars().skip(1) {
        if c == current_digit {
            two_adjacents = true;
        }
        current_digit = c;
    }
    two_adjacents
}

fn criteria_only2adjacents(number:i32) -> bool {
    let num_str = number.to_string();
    let mut only2adjacents = false;
    let mut had2adjacents = false;
    let mut occurs = 0i32;
    let mut last : char = 'a';
    for c in num_str.chars() {
        if c == last || last == 'a' {
            occurs += 1;
            if occurs == 2 {
                only2adjacents = true;
            } else {
                only2adjacents = false;
            }
        } else {
            occurs = 1;
            if only2adjacents {
                had2adjacents = true;
            }
        }
        last = c;
    }
    only2adjacents || had2adjacents
}

fn criteria_neverdecrease(number:i32) -> bool {
    let num_str = number.to_string();
    let mut never_decrease = true;
    let mut previous_char : char = num_str.chars().nth(0).unwrap();
    for c in num_str.chars() {
        never_decrease = never_decrease && (previous_char <= c);
        previous_char = c;
    }
    never_decrease
}

fn main() {
    let from = i32::from_str(&std::env::args().nth(1).unwrap()).unwrap();
    let to = i32::from_str(&std::env::args().nth(2).unwrap()).unwrap();
    let mut counter = 0i32;
    for num in from..to {
        if criteria_6digits(num)
        && criteria_2adjacents(num)
        && criteria_neverdecrease(num)
        && criteria_only2adjacents(num) {
            counter += 1;
        }
    }
    println!("matching numbers: {}", counter);
}
