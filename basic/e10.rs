fn main() {
    let nums = [1, 2, 3, 4, 5, 6, 7, 8];
    let mut max = 0;

    for i in nums {
        if i > max {
            max = i;
        }
    }

    println!("{}", max);
}
