use std::io;

fn main() {
    let mut input = String::new();
    let vec: Vec<i32>;

    println!("Enter a list of numbers separated by whitespace: ");
    io::stdin().read_line(&mut input);
    vec = input
        .trim()
        .split_whitespace()
        .into_iter()
        .map(|x| {
            x.parse::<i32>().expect("Error converting &str to i32.")
        })
        .collect::<Vec<_>>();

    for i in vec {
        if i % 2 == 0 {
            print!("{} ", i);
        }
    }
    println!();
}
