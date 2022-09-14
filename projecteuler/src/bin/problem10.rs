fn main() {
    println!("{}", sum_primes(2000000));
}

fn sum_primes(range: i32) -> i32 {
    let mut sum = 0;
    for i in 1..range {
        if is_prime(i) {
            sum += i;
        }
    }
    return sum;
}

fn is_prime(number: i32) -> bool {
    if number == 1 { return false; }
    for i in 2..number {
        if number % i == 0 {
            return false;
        }
    }
    return true;
}
