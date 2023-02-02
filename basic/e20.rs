use std::io;

fn main() {
    let mut input = String::new();
    let mut rev: String;

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    rev = input.chars().rev().collect();

    println!("{}", rev);
}
