use std::io;

fn main() {
    let mut input = String::new();
    let number: i32;

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");
    input = input
        .trim()
        .to_string();
    number = input
        .parse()
        .expect("Error converting String to i32.");

    println!("{}² = {}", number, number.pow(2));
}
