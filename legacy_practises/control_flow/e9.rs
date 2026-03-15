use std::io;

fn is_prime(n: u32) -> bool {
    for i in 2..=n/2 {
        if n % i == 0 {
            return false;
        }
    }

    true
}

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

    if is_prime(number as u32) {
        println!("Prime number.");
    } else {
        println!("Not prime number.");
    }
}
