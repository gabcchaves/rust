fn main() {
    println!("{}", smallest_multiple());
}

fn smallest_multiple() -> i64 {
    for i in 40.. {
        for j in 1..=20 {
            if i % j != 0 {
                break;
            } else if j == 20 {
                return i;            
            }
        }
    }
    return -1;
}
