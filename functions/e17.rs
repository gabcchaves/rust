fn remove_whitespaces(s: &str) -> String {
    s.split_whitespace().collect()
}

fn main() {
    assert_eq!(remove_whitespaces("A string literal."), String::from("Astringliteral."));
    println!("Success!");
}
