fn sum(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

fn main() {
    assert_eq!(sum(2, 2), 4);
    assert_eq!(sum(-2, 2), 0);
    println!("Success!");
}
