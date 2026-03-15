fn count_b(string: &str) -> u32 {
    string
        .matches('b')
        .collect::<Vec<&str>>()
        .len()
        as u32
}

fn main() {
    assert_eq!(count_b("abacate"), 1);
    println!("Success!");
}
