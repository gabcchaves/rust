use std::io;

fn read_int() -> i32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    input
        .parse()
        .expect("Error converting String to i32.")
}

fn main() {
    let n1: i32;
    let n2: i32;

    println!("Enter a number: ");
    n1 = read_int();

    println!("Enter another number: ");
    n2 = read_int();

    if n1 % n2 == 0 {
        println!("{} is divisible by {}.", n1, n2);
    } else {
        println!("{} is not divisible by {}.", n1, n2);
    }
}
