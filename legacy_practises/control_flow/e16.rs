use std::io;

fn main() {
    let mut input = String::new();
    let rev: String;

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    rev = input.chars().rev().collect();

    if input == rev {
        println!("Is a palindrome.");
    } else {
        println!("Is not a palindrome.");
    }
}
