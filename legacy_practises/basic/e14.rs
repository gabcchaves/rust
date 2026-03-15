use std::io;

fn main() {
    let mut input = String::new();
    let n: i32;
    let mut sum = 0;

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    n = input
        .parse()
        .expect("Error converting String to i32.");

    for i in 0..=n {
        sum += i;
    }

    println!("{}", sum);
}
