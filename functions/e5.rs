fn get_lengths(strings: Vec<&str>) -> Vec<usize> {
    let mut lengths = Vec::<usize>::new();

    for s in strings {
        lengths.push(s.len());
    }

    lengths
}

fn main() {
    assert_eq!(get_lengths(vec!["one", "two", "three"]), vec![3, 3, 5]);
    println!("Success!");
}
