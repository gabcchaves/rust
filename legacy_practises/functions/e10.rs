fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut v1: Vec<u8> = s1.bytes().collect();
    let mut v2: Vec<u8> = s2.bytes().collect();
    v1.sort() == v2.sort()
}

fn main() {
    assert!(is_permutation("Teste", "sTete"));
    println!("Success!");
}
