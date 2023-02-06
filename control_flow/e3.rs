use std::io;

fn main() {
    let mut input = String::new();
    let c: i32;
    let f: i32;

    println!("Enter a temperature in Celsius: ");
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    c = input
        .parse()
        .expect("Error converting String to i32.");

    f = c * 9 / 5 + 32;

    println!("F°: {}.", f);
}
