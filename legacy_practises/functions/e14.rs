use std::cmp::Ordering;

fn sort_words(words: Vec<&str>) -> Vec<String> {
    let mut vec_bytes_vec: Vec<&[u8]> = Vec::new();
    let mut vec_ordered: Vec<String> = Vec::new();

    for word in words {
        vec_bytes_vec.push(word.as_bytes());
    }

    vec_bytes_vec.sort();

    for byte_slice in vec_bytes_vec {
        let string: String = std::str::from_utf8(byte_slice)
            .expect("Error converting bytes to string.")
            .to_string();
        vec_ordered.push(string);
    }

    vec_ordered
}

fn main() {
    sort_words(vec!["fruta", "amor", "coisa"]);
    assert_eq!(sort_words(vec!["amor", "fruta", "coisa"]), vec!["amor", "coisa", "fruta"]);
    println!("Success!");
}
