// Program to find the first triangle number to have over
// five hundred divisors.
fn main() {
    let mut nth = 7;
    loop {
        if get_divisors_number(get_nth_triangle_number(nth)) > 500 {
            println!("{}º: {}", nth, get_nth_triangle_number(nth));
            break;
        }
        nth += 1;
    }
}

fn get_divisors_number(number: i64) -> i16 {
    let mut count: i16 = 0;
    for i in 1..=number/2 {
        if number % i == 0 {
            count += 1;
        }
    }
    return count + 1; // Include the number itself.
}

fn get_nth_triangle_number(nth: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..=nth {
        sum += i;
    }
    return sum;
}
