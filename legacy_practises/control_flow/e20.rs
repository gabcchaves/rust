use std::io;

fn read_int() -> u32 {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    input
        .parse::<u32>()
        .expect("Error converting String to u32.")
}

fn comp_gcd(n1: u32, n2: u32) -> u32 {
    let mut a = n1;
    let mut b = n2;

    while a != b {
        if a > b {
            a -= b;
        } else {
            b -= a;
        }
    }

    a
}

fn comp_lcm(n1: u32, n2: u32) -> u32 {
    n1 * n2 / comp_gcd(n1, n2)
}

fn main() {
    let n1: u32;
    let n2: u32;

    println!("Enter an integer: ");
    n1 = read_int();

    println!("Enter another integer: ");
    n2 = read_int();

    println!("LCM: {}", comp_lcm(n1, n2));
}
