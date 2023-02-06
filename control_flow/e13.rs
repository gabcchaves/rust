use std::io;

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        fibonacci(n - 2) + fibonacci(n - 1)
    }
}

fn main() {
    let mut input = String::new();
    let mut i: u32;
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

    i = 1;
    while fibonacci(i) < n {
        print!("{} ", fibonacci(i));
        i += 1;
    }
    println!();
}
