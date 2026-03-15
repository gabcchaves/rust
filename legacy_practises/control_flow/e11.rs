use std::io;

fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        factorial(n - 1) * n
    }
}

fn main() {
    let mut input = String::new();
    let n: u32;

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STIDIN.");

    input = input
        .trim()
        .to_string();

    n = input
        .parse()
        .expect("Error converting String to u32.");

    println!("Factorial: {}.", factorial(n));
}
