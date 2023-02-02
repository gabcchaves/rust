fn main() {
    let mut prod = 1;

    for i in 1..=10 {
        prod *= i;
    }

    println!("{}", prod);
}
