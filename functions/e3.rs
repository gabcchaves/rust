fn get_largest(vec: Vec<i32>) -> i32 {
    vec.into_iter().max().unwrap()
}

fn main() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

    assert_eq!(get_largest(vec), 8);
    println!("Success!");
}
