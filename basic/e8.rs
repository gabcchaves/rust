use std::io;

fn main() {
    let mut input = String::new();
    let l1: i32;
    let l2: i32;

    println!("Enter a dimension of a rectangle: ");
    io::stdin().read_line(&mut input);
    input = input
        .trim()
        .to_string();
    l1 = input
        .parse()
        .expect("Error converting String to i32.");
    input = String::new();

    println!("Enter another dimension of the same rectangle: ");
    io::stdin().read_line(&mut input);
    input = input
        .trim()
        .to_string();
    l2 = input
        .parse()
        .expect("Error converting String to i32.");

    println!("Its area is {}.", l1 * l2);
}
