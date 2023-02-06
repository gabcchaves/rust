use std::io;

fn read_float() -> f32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    input
        .parse()
        .expect("Error converting String to f32.")
}

fn main() {
    let height: f32;
    let base: f32;
    let area: f32;

    println!("Enter the height of a triangle: ");
    height = read_float();

    println!("Enter the base of the same triangle: ");
    base = read_float();

    area = base * height / 2.0;

    println!("Area: {}.", area);
}
