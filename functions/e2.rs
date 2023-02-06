fn concat(s1: &str, s2: &str) -> String {
    s1.to_owned() + s2
}

fn main() {
    let s1 = "Str";
    let s2 = "ing";
    let s3 = concat(s1, s2);

    assert_eq!(s3, "String");
    println!("Success!");
}
