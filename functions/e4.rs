fn count_vowels(s: &str) -> u32 {
    let mut count = 0;

    for c in s.chars() {
        if ['a', 'e', 'i', 'o', 'u'].contains(&c) {
            count += 1;
        }
    }

    count
}

fn main() {
    assert_eq!(count_vowels("String"), 1);
    println!("Success!");
}
