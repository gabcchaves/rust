use std::io;

fn scan_int() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    input
        .parse()
        .expect("Error converting input String to i32.")
}

fn main() {
    let n1: i32;
    let n2: i32;

    println!("Enter two numbers, pressing enter after each: ");
    n1 = scan_int();
    n2 = scan_int();

    if n1 >= n2 {
        println!("{}", n1);
    } else {
        println!("{}", n2);
    }
}
