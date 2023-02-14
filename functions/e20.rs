fn measure_longest_substring(s1: &str, s2: &str) -> usize {
    let mut length: usize = 0;

    for substring in s1.split_whitespace() {
        if s2.contains(substring) && substring.len() > length {
            length = substring.len();
        }
    }

    length
}

fn main() {
    assert_eq!(measure_longest_substring("A string", "Another string"), 6);
    println!("Success!");
}
