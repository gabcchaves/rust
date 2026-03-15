fn main() {
    let big_number: i64 = 600851475143;
    // The largest factor is less than or equal to the half.
    let mut largest = 2;
    // Run through odd numbers only, checking if they are prime.
    for n in 3 .. big_number / 2 {
        if is_prime(n) && big_number % n == 0 {
            largest = n;
        }
    }

    println!("{}", largest);
}

fn is_prime(n: i64) -> bool {
    for i in 2 .. n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}
