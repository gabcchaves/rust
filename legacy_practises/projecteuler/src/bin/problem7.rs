fn main() {
    println!("{}", find_nth_prime(10001));
}

fn find_nth_prime(_n: i64) -> i64 {
    let mut index: i64 = 1;
    let mut n = 3;
    loop {
        if is_prime(n) {
            index += 1;
        }
        if index == _n {
            return n;
        }
        n += 2;
    }
}

fn is_prime(_n: i64) -> bool {
    for i in 2 .. _n / 2 {
        if _n % i == 0 {
            return false;
        }
    }
    return true;
}
