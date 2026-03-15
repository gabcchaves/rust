use std::io;

fn main() {
    let mut input = String::new();
    let mut sum: u32;
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

    sum = 0;
    for i in 0..=n {
        sum += i;
    }

    println!("Sum of all natural numbers up to the given number: ");
    println!("{}", sum);
}
