use std::io;

fn main() {
    let mut input = String::new();
    let number: i32;

    println!("Enter an integer.");

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    number = input
        .parse()
        .expect("Error converting String to i32.");

    if number % 2 == 0 {
        println!("The number is even.");
    } else {
        println!("The number is odd.");
    }
}
