fn reverse_vec(vec: Vec<i32>) -> Vec<i32> {
    vec
        .iter()
        .map(|x| x.to_owned())
        .rev()
        .collect()
}

fn main() {
    assert_eq!(reverse_vec(vec![1, 2, 3]), vec![3, 2, 1]);
    println!("Success!");
}
