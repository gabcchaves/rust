/* Program to read a string and invert it. */
use std::io;

fn main() {
    let scn = io::stdin();
    let mut str_input = String::new();
    match scn.read_line(&mut str_input) {
        Ok(..) => {
            println!("{}", str_input.chars().rev().collect::<String>());
        }
        Err(error) => println!("Input error: {error}"),
    }
}
