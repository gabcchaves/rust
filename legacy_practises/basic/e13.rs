use std::io;

fn main() {
    let mut input = String::new();
    let number: f32;

    io::stdin().read_line(&mut input);
    input = input
        .trim()
        .to_string();

    number = input
        .parse()
        .expect("Error converting String to f32.");

    println!("{}", number.sqrt());
}
