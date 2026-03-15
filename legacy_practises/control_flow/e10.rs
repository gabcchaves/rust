use std::io;

fn comp_gcd(n1: u32, n2: u32) -> u32 {
    let gcd: u32;

    if n1 > n2 {
        gcd = cmp_gcd(n1 - n2, n2)
    } else if n2 > n1 {
        gcd = cmp_gcd(n1, n2 - n1)
    } else {
        gcd = n1;
    }
    
    gcd
}

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
    let mut input = String::new();
    let n1: i32;
    let n2: i32;

    println!("Enter two integers, entering after each: ");
    n1 = read_int();
    n2 = read_int();

}
