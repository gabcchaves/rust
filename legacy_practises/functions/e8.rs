fn is_prime(n: u32) -> bool {
    let mut is_prime = true;

    if n > 2 {
        for i in (3..=n/2).step_by(2) {
            if n % i == 0 {
                is_prime = false;
            }
        }
    } else if n < 2 {
        is_prime = false;
    }

    is_prime
}

fn main() {
    let a = 7;
    let b = 2;
    let c = 8;

    assert!(is_prime(a));
    assert!(is_prime(b));
    assert!(is_prime(c));

    println!("Success!");
}
