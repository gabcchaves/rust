use std::io;

fn main() {
    let mut input = String::new();
    let mut sum: u32;

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    sum = 0;
    for c in input.chars() {
        sum += c
            .to_digit(10)
            .expect("Input is not a number.")
            .pow(input.len() as u32);
    }

    if sum.to_string() == input {
        println!("Is an Armstrong number.");
    } else {
        println!("Is not an Armstrong number.");
    }
}
