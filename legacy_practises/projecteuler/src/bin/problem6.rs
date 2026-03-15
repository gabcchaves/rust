fn main() {
    println!("{}", calc_square_diff(10));
}

fn calc_square_diff(_n: i32) -> i32 {
    let mut squares_sum: i32 = 0;
    let mut sum_square: i32 = 0;
    for i in 1..=_n {
        squares_sum = squares_sum + (i as i32).pow(2);
    }
    for i in 1..=_n {
        sum_square = sum_square + i;
    }
    sum_square = sum_square.pow(2);

    let result = squares_sum - sum_square;
    if result < 0 {
        return - result;
    }
    return squares_sum - sum_square;
}
