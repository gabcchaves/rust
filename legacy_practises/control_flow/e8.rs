use std::io;

fn main() {
    let mut input = String::new();
    let grade: f32;

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");

    input = input
        .trim()
        .to_string();

    grade = input
        .parse()
        .expect("Error converting String to f32.");

    if grade >= 6.0 {
        println!("You passed!");
    } else {
        println!("You failed.");
    }
}
