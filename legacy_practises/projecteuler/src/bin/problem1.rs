fn main() {
    let mut sum = 0;
    // Run through odd numbers
    for n in 3..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            sum = sum + n;
        }
    }
    println!("{}", sum);
}
