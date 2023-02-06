use std::io;

fn main() {
    let mut input = String::new();
    let n: u32;

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    n = input
        .parse()
        .expect("Error converting String to u32.");

    println!("Square root: {}.", f32::sqrt(n as f32));
}
