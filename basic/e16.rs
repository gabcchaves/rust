use std::io;

fn count_vowels(string: &str) -> i32 {
    let mut count = 0;

    for c in string.chars() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
            count += 1;
        }
    }

    count
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");
    input = input
        .trim()
        .to_string();

    println!("{}", count_vowels(&input));
}
