fn remove_vowels(s: &str) -> String {
    let mut string = String::new();

    for c in s.chars() {
        if !['a', 'e', 'i', 'o', 'u'].contains(&c) {
            string.push(c.to_owned());
        }
    }

    string
}

fn main() {
    let string = String::from("String");
    assert_eq!(remove_vowels(&string), "Strng");
    println!("Success!");
}
