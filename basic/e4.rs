use std::io;

fn fibonacci(nth: i32) -> i32 {
    if nth >= 2 {
        fibonacci(nth-2) + fibonacci(nth-1)
    } else {
        1
    }
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    let nth: i32;

    io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();
    nth = input.parse().expect("Error converting String to i32.");

    println!("{}", fibonacci(nth));

    Ok(())
}
