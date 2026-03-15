fn to_uppercase(s: &str) -> String {
    s.to_uppercase()
}

fn main() {
    assert_eq!(to_uppercase("Hi"), String::from("HI"));
    println!("Success!");
}
