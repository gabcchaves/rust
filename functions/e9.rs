fn collect_even(vec: &Vec<i32>) -> Vec<i32> {
    vec
        .to_owned()
        .into_iter()
        .filter(|x| x % 2 == 0)
        .collect::<Vec<i32>>()
}

fn main() {
    let vec: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8];

    assert_eq!(collect_even(&vec), vec![2, 4, 6, 8]);
    println!("Success!");
}
