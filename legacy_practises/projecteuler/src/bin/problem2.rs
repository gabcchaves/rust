fn main() {
    let mut n = 1;
    let mut sum = 0;
    while fibo(n) <= 4000000 {
        if fibo(n) % 2 == 0 {
            sum = sum + fibo(n);
        }
        n = n + 1;
    }
    println!("{}", sum);
}

fn fibo(n: i32) -> i32 {
    if n <= 1 {
        return 1;
    }
    return fibo(n - 1) + fibo(n - 2);
}
