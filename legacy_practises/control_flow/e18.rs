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
    for i in input.chars() {
        sum += i
            .to_digit(10)
            .expect("Given input is not a number.");
    }

    println!("Sum of the digits: {}.", sum);
}
