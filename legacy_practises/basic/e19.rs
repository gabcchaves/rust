use std::io;

fn compute_gcd(n1: i32, n2: i32) -> i32 {
    if n1 != n2 {
        if n1 > n2 {
            compute_gcd(n1 - n2, n2)
        } else {
            compute_gcd(n1, n2 - n1)
        }
    } else {
        n1
    }
}

fn compute_lcm(n1: i32, n2: i32) -> i32 {
    n1 / compute_gcd(n1, n2) * n2
}

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading STDIN.");
    input = input
        .trim()
        .to_string();

    let vec: Vec<i32> = input
        .split_whitespace()
        .collect::<Vec<_>>()
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    if vec.len() > 2 {
        panic!("Enter only two numbers.");
    }

    println!("{}", compute_lcm(vec[0], vec[1]));
}
