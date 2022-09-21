use std::io;
use std::str::FromStr;

fn main() {
    println!("Enter the value of a: ");
    let a: f64 = {
        let mut input = read_float();
        while input == 0.0 {
            println!("Enter a non-zero value for a: ");
            input = read_float();
        }
        input
    };

    println!("Enter the value of b: ");
    let b: f64 = read_float();

    println!("Enter the value of c: ");
    let c: f64 = read_float();

    let d: f64 = b.powi(2) - 4.0 * a * c;
    let x1: f64 = ( -b + d.sqrt() ) / ( 2.0 * a );
    let x2: f64 = ( -b - d.sqrt() ) / ( 2.0 * a );

    println!("Results:");
    if x1 == f64::NAN {
        println!("X1: Non real");
    } else {
        println!("{}", x1);
    }
    if x2 == f64::NAN {
        println!("X2: Non real");
    } else {
        println!("{}", x2);
    }
}

fn read_float() -> f64 {
    let mut str_input = String::new();
    let mut num: f64;
    io::stdin().read_line(&mut str_input);
    num = f64::from_str(&str_input.trim()).unwrap();
    num
}
