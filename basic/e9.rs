use std::io;

fn factorial(n: i32) -> i32 {
    if n == 1 {
        1
    } else {
        n * factorial(n-1)
    }
}

fn main() {
    let mut input = String::new();
    let n: i32;

    io::stdin().read_line(&mut input).expect("Error reading STDIN.");
    input = input
        .trim()
        .to_string();
    n = input.parse().expect("Error converting String to i32.");

    println!("{}", factorial(n));
}
