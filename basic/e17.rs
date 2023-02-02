use std::io;

fn main() {
    let mut input = String::new();
    let mut sum = 0;

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    for c in input.chars() {
        sum += c
            .to_digit(10)
            .expect("Error converting char to i32.");
    }

    println!("{}", sum);
}
