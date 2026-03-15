fn reverse_words(string: &str) -> String {
    let mut new_string = String::new();

    for word in string.split_whitespace() {
        new_string.push_str(&word.chars().rev().collect::<String>());
        new_string.push(' ');
    }

    new_string.trim().to_string()
}

fn main() {
    assert_eq!(reverse_words("Duas palavras"), "sauD sarvalap");
    println!("Success!");
}
